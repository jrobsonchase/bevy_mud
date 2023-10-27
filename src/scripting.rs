mod log;

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

use self::log::LogAPIProvider;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(bevy_mod_scripting::prelude::ScriptingPlugin);
    app.add_script_host::<LuaScriptHost<()>>(PostUpdate);
    app.add_script_handler::<LuaScriptHost<()>, 0, 0>(PostUpdate);
    app.add_systems(
      PreUpdate,
      |mut events: PriorityEventWriter<LuaEvent<()>>,
       query: Query<Entity, Added<ScriptCollection<LuaFile>>>| {
        query.iter().for_each(|entity| {
          debug!(?entity, "sending init to new script");
          events.send(
            LuaEvent {
              hook_name: "init".into(),
              args: (),
              recipients: Recipients::Entity(entity),
            },
            0,
          )
        });
      },
    );
    app.add_systems(
      PreUpdate,
      |mut events: PriorityEventWriter<LuaEvent<()>>| {
        events.send(
          LuaEvent {
            hook_name: "update".into(),
            args: (),
            recipients: Recipients::All,
          },
          0,
        );
      },
    );
    app.add_api_provider::<LuaScriptHost<()>>(Box::new(LuaBevyAPIProvider));
    app.add_api_provider::<LuaScriptHost<()>>(Box::new(LogAPIProvider));
    app.update_documentation::<LuaScriptHost<()>>();
  }
}
