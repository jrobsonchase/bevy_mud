use bevy::{
  prelude::*,
  time::Stopwatch,
};

pub struct LogFrameRatePlugin<const WINDOW: usize>;

#[derive(Resource)]
struct FrameWindow<const WINDOW: usize>(usize, [Option<f64>; WINDOW]);

impl<const WINDOW: usize> Default for FrameWindow<WINDOW> {
  fn default() -> Self {
    FrameWindow(0, [None; WINDOW])
  }
}

#[derive(Resource, Debug, Copy, Clone)]
struct Fps(f64);

#[derive(Resource, Debug, Copy, Clone)]
struct AvgFps(f64);

impl<const WINDOW: usize> Plugin for LogFrameRatePlugin<WINDOW> {
  fn build(&self, app: &mut App) {
    let mut t = Time::default();
    t.update();
    app.insert_resource(Fps(0.0));
    app.insert_resource(AvgFps(0.0));
    app.add_systems(Last, log_frame_rate::<WINDOW>);
  }
}

fn log_frame_rate<const WINDOW: usize>(
  time: Res<Time>,
  mut frames: Local<usize>,
  mut stopwatch: Local<Stopwatch>,
  mut window: Local<FrameWindow<WINDOW>>,
  mut inst_rate: ResMut<Fps>,
  mut avg_rate: ResMut<AvgFps>,
) {
  info!(tracy.frame_mark = true);
  *frames += 1;
  stopwatch.tick(time.delta());

  let now = stopwatch.elapsed_secs_f64();

  if now < 1.0 {
    return;
  }

  inst_rate.0 = *frames as f64 / now;

  *frames = 0;
  stopwatch.reset();

  let i = window.0;

  window.1[i] = Some(inst_rate.0);
  window.0 = (i + 1) % WINDOW;

  let (sum, count) = window
    .1
    .iter()
    .filter_map(|&x| x)
    .fold((0.0, 0.0), |(sum, count), val| (sum + val, count + 1.0));

  let avg_fps = sum / count;

  avg_rate.0 = avg_fps;

  if i == 0 {
    debug!(avg_fps, inst_fps = inst_rate.0);
  }
}
