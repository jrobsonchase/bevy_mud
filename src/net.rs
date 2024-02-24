use std::{
  env,
  fmt::{
    Debug,
    Write,
  },
  net::SocketAddr,
};

use bevy::{
  app::AppExit,
  prelude::*,
};
use bevy_async_util::AsyncRuntime;
use bytes::BytesMut;
use futures::{
  prelude::*,
  StreamExt,
};
use ngrok::prelude::{
  ConnInfo,
  TunnelBuilder,
};
use tellem::{
  Cmd,
  Event,
  KnownOpt,
  Opt,
};
use tokio::{
  io::{
    AsyncRead,
    AsyncWrite,
    BufStream,
  },
  net::TcpListener,
  runtime::Handle,
  sync::mpsc::{
    self,
    error::TryRecvError,
    UnboundedReceiver,
    UnboundedSender,
  },
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::codec::Decoder;
use tracing::{
  info,
  instrument,
  Instrument,
};

use crate::{
  core::CantonStartup,
  oneshot::run_system,
  util::HierEntity,
};

#[macro_export]
macro_rules! command {
  ($out:expr, $cmd:tt) => {
    $out.telnet(tellem::Event::Cmd(tellem::Cmd::$cmd))
  };
  ($out:expr, $cmd:tt, $opt:tt) => {
    $out.telnet(tellem::Event::Negotiation(
      tellem::Cmd::$cmd,
      tellem::Opt::Known(tellem::KnownOpt::$opt),
    ))
  };
}

#[macro_export]
macro_rules! negotiate {
  ($out:expr, sub, $opt:tt, $data:expr) => {
    $out.telnet(tellem::Event::Subnegotiation(
      tellem::Opt::Known(tellem::KnownOpt::$opt),
      $data,
    ))
  };
  ($out:expr, $cmd:tt, $opt:tt) => {
    $out.telnet(tellem::Event::Negotiation(
      tellem::Cmd::$cmd,
      tellem::Opt::Known(tellem::KnownOpt::$opt),
    ))
  };
}

pub struct TelnetPlugin;

impl Plugin for TelnetPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<NewConns>()
      .register_type::<TelnetIn>()
      .register_type::<TelnetOut>()
      .register_type::<Listener>()
      .register_type::<ClientConn>()
      .add_systems(Startup, start_listener.in_set(CantonStartup::Io))
      .add_systems(First, new_conns)
      .add_systems(First, telnet_handler)
      .add_systems(Last, reap_conns)
      .add_systems(Last, print_reaped_conns.after(reap_conns));
  }
}

#[derive(Resource, Debug, Copy, Clone)]
pub struct PortArg(pub u32);

#[derive(Component, Debug, Reflect)]
#[reflect(from_reflect = false)]
struct NewConns {
  #[reflect(ignore)]
  channel: UnboundedReceiver<ClientBundle>,
}

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
#[reflect(Component)]
struct Listener {
  #[allow(dead_code)]
  port: u32,
}

impl Default for Listener {
  fn default() -> Self {
    Listener { port: 23840 }
  }
}

fn start_ngrok(rt: &AsyncRuntime, domain: &str) -> anyhow::Result<UnboundedReceiver<ClientBundle>> {
  let mut l = rt.block_on(async move {
    Ok::<_, anyhow::Error>(
      ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?
        .tls_endpoint()
        .domain(domain)
        .termination(Default::default(), Default::default())
        .listen()
        .await?,
    )
  })?;

  info!(domain, "started ngrok tls listener");
  let (new_tx, new_rx) = mpsc::unbounded_channel();

  let h = rt.handle();

  rt.spawn(async move {
    while let Some(Ok(conn)) = l.next().await {
      let addr = conn.remote_addr();
      if new_tx.send(handle_conn(&h, conn, addr)?).is_err() {
        break;
      }
    }
    Ok(())
  });

  Ok(new_rx)
}

fn start_tcp(rt: &AsyncRuntime, port: u32) -> anyhow::Result<UnboundedReceiver<ClientBundle>> {
  let l = rt.block_on(TcpListener::bind(format!("0.0.0.0:{port}")))?;
  info!(port, "started tcp listener");
  let (new_tx, new_rx) = mpsc::unbounded_channel();

  let h = rt.handle();

  rt.spawn(async move {
    while let Ok((conn, addr)) = l.accept().await {
      if new_tx.send(handle_conn(&h, conn, addr)?).is_err() {
        break;
      }
    }
    Ok(())
  });

  Ok(new_rx)
}

fn start_listener(
  arg: Res<PortArg>,
  rt: Res<AsyncRuntime>,
  mut commands: Commands,
  mut exit: EventWriter<AppExit>,
) {
  let port = arg.0;

  let res = start_tcp(&rt, port);
  let l = match res {
    Ok(l) => l,
    Err(err) => {
      warn!(?err, "failed to start tcp listener, exiting.");
      exit.send(AppExit);
      return;
    }
  };
  commands.spawn((Listener { port }, NewConns { channel: l }));

  if env::var("NGROK_AUTHTOKEN").is_ok() && env::var("NGROK_DOMAIN").is_ok() {
    let res = start_ngrok(&rt, env::var("NGROK_DOMAIN").unwrap().as_str());
    let l = match res {
      Ok(l) => l,
      Err(err) => {
        warn!(?err, "failed to start ngrok listener.");
        return;
      }
    };
    commands.spawn(NewConns { channel: l });
  }
}

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct TelnetIn {
  #[reflect(ignore)]
  channel: UnboundedReceiver<tellem::Event>,
  #[reflect(ignore)]
  peek: Option<tellem::Event>,
  closed: bool,
}

#[derive(Copy, Clone, Debug, Reflect, Default)]
pub struct Closed;

impl TelnetIn {
  fn new(channel: UnboundedReceiver<tellem::Event>) -> Self {
    Self {
      channel,
      peek: None,
      closed: false,
    }
  }
  fn update(&mut self) {
    if self.peek.is_some() {
      return;
    }
    match self.channel.try_recv() {
      Ok(v) => self.peek = Some(v),
      Err(TryRecvError::Disconnected) => self.closed = true,
      _ => (),
    }
  }

  pub fn closed(&self) -> bool {
    self.closed && self.peek.is_none()
  }

  pub fn next<F, T>(&mut self, f: F) -> Option<T>
  where
    F: FnOnce(tellem::Event) -> Result<T, tellem::Event>,
  {
    self.update();
    let event = self.peek.take()?;
    match f(event) {
      Ok(v) => Some(v),
      Err(e) => {
        self.peek = Some(e);
        None
      }
    }
  }

  pub fn next_telnet(&mut self) -> Option<tellem::Event> {
    self.next(|v| {
      extract! {v,
            tellem::Event::Cmd(_)
          | tellem::Event::Negotiation(_, _)
          | tellem::Event::Subnegotiation(_, _)
      }
    })
  }

  pub fn next_line(&mut self) -> Option<String> {
    self.next(|v| {
      extract! {v,
          tellem::Event::Data(data) => String::from_utf8_lossy(&data).into_owned(),
      }
    })
  }

  #[allow(dead_code)]
  pub fn peek(&self) -> Option<&tellem::Event> {
    self.peek.as_ref()
  }
}

#[derive(Component, Clone, Reflect)]
#[reflect(from_reflect = false)]
pub struct TelnetOut {
  #[reflect(ignore)]
  channel: UnboundedSender<tellem::Event>,
}

impl TelnetOut {
  fn new(channel: UnboundedSender<tellem::Event>) -> Self {
    Self { channel }
  }

  pub fn telnet(&self, event: tellem::Event) -> &Self {
    let _ = self.channel.send(event);
    self
  }

  fn normalize_string(s: impl AsRef<str>) -> BytesMut {
    let mut s: &str = s.as_ref();
    let mut data = BytesMut::with_capacity(s.len());

    loop {
      if s.is_empty() {
        break;
      }

      match s.find('\n') {
        Some(i) => {
          data.extend_from_slice(s[..i].as_bytes());
          if i == 0 || s.as_bytes()[i - 1] != b'\r' {
            data.extend_from_slice("\r\n".as_bytes());
          } else {
            data.extend_from_slice(&[b'\n']);
          }
          s = &s[i + 1..];
        }
        None => {
          data.extend_from_slice(s.as_bytes());
          break;
        }
      }
    }

    data
  }

  pub fn line(&self, s: impl AsRef<str>) -> &Self {
    if self.closed() {
      return self;
    }

    let mut data = TelnetOut::normalize_string(s);

    if !matches!(data.last(), Some(b'\n')) {
      data.extend_from_slice("\r\n".as_bytes());
    }

    self.telnet(tellem::Event::Data(data))
  }

  pub fn string(&self, s: impl AsRef<str>) -> &Self {
    if self.closed() {
      return self;
    }

    self.telnet(tellem::Event::Data(TelnetOut::normalize_string(s)))
  }

  pub fn closed(&self) -> bool {
    self.channel.is_closed()
  }
}

impl<'a> Write for &'a TelnetOut {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    self.string(s);
    Ok(())
  }
}

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct ClientConn {
  #[reflect(ignore)]
  pub remote_addr: SocketAddr,
}

#[derive(Bundle)]
struct ClientBundle {
  conn: ClientConn,
  input: TelnetIn,
  output: TelnetOut,
}

fn handle_conn<C>(rt: &Handle, conn: C, remote_addr: SocketAddr) -> anyhow::Result<ClientBundle>
where
  C: AsyncRead + AsyncWrite + Send + 'static,
{
  let (read_tx, read_rx) = mpsc::unbounded_channel();
  let (write_tx, write_rx) = mpsc::unbounded_channel();

  let bundle = ClientBundle {
    conn: ClientConn { remote_addr },
    input: TelnetIn::new(read_rx),
    output: TelnetOut::new(write_tx.clone()),
  };

  let _span = info_span!("client", ?remote_addr).entered();

  info!("got new connection");

  let (twrite, tread) = tellem::Parser::default()
    .framed(BufStream::new(conn))
    .split();

  rt.spawn(
    async move {
      let mut tread = tread;
      let mut buffer = BytesMut::new();
      'read: while let Some(res) = tread.next().await {
        let event = match res {
          Ok(Event::Data(data)) => {
            buffer.extend_from_slice(&data);
            while let Some(i) = buffer.iter().position(|b| *b == b'\n') {
              let mut line = buffer.split_to(i + 1);
              while let Some(b'\r' | b'\n') = line.last() {
                line.truncate(line.len() - 1);
              }
              if read_tx.send(Event::Data(line)).is_err() {
                break 'read;
              }
            }
            continue;
          }
          Ok(event) => event,
          Err(err) => {
            debug!(?err, "error reading from client stream");
            break;
          }
        };
        if read_tx.send(event).is_err() {
          break;
        }
      }
      debug!("client read task exit");
      Ok::<(), anyhow::Error>(())
    }
    .in_current_span(),
  );
  rt.spawn(
    async move {
      let mut twrite = twrite;
      let write_rx = UnboundedReceiverStream::new(write_rx);
      write_rx.map(Ok).forward(&mut twrite).await?;
      twrite.flush().await?;
      twrite.close().await?;
      Ok::<(), anyhow::Error>(())
    }
    .in_current_span(),
  );
  Ok(bundle)
}

#[instrument(skip_all)]
fn new_conns(mut cmd: Commands, mut query: Query<(Entity, &mut NewConns, Option<&Children>)>) {
  for (listener_id, mut conns, children) in query.iter_mut() {
    let bundle = match conns.channel.try_recv() {
      Ok(v) => v,
      Err(TryRecvError::Empty) => continue,
      Err(TryRecvError::Disconnected) => {
        for child in children.iter().flat_map(|c| c.iter()) {
          cmd.entity(*child).remove_parent();
        }
        cmd.entity(listener_id).despawn();
        cmd.add(run_system(start_listener));
        continue;
      }
    };

    let entity_id = cmd.spawn(bundle).set_parent(listener_id).id();

    cmd.entity(listener_id).add_child(entity_id);
  }
}

#[allow(clippy::type_complexity)]
fn reap_conns(mut cmd: Commands, conns: Query<(HierEntity, &TelnetIn)>) {
  for (child, input) in conns.iter() {
    if input.closed() {
      debug!(?child.entity, "reaping connection");
      child.despawn(&mut cmd);
    }
  }
}

fn print_reaped_conns(mut conns: RemovedComponents<TelnetIn>) {
  for entity in conns.read() {
    debug!(?entity, "connection despawned");
  }
}

#[derive(Copy, Clone, Default, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct GMCP;

pub fn telnet_handler(mut cmd: Commands, mut query: Query<(Entity, &mut TelnetIn)>) {
  for (entity, mut input) in query.iter_mut() {
    while let Some(event) = input.next_telnet() {
      match event {
        Event::Negotiation(Cmd::DONT, Opt::Known(KnownOpt::GMCP)) => {
          debug!(?entity, "not enabling GMCP");
        }
        Event::Negotiation(Cmd::DO, Opt::Known(KnownOpt::GMCP)) => {
          debug!(?entity, "enabling GMCP");
          cmd.entity(entity).insert(GMCP);
        }
        _ => debug!(?event, "ignoring telnet event"),
      }
    }
  }
}
