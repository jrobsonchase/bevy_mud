use std::any::TypeId;
use std::fmt;

use bevy::ecs::component::ComponentId as BComponentId;
use bevy::ecs::reflect::AppTypeRegistry;
use bevy::log::{
    debug,
    info,
};
use bevy::prelude::{
    Deref,
    DerefMut,
    World as BWorld,
};
use bevy::reflect::TypePath;
use piccolo::FromValue;
use piccolo::{
    Context,
    IntoValue,
    UserData,
    Value,
};
use piccolo_util::{
    freeze::{
        Freeze,
        Frozen,
    },
    user_methods::StaticUserMethods,
};

use crate::methods;

pub type WorldMutF = Frozen<Freeze![&'freeze mut BWorld]>;

pub trait WithRef {
    type Target;
    fn with_ref<R>(&self, f: impl FnOnce(&Self::Target) -> R) -> R;
}

pub trait WithMut: WithRef {
    fn with_mut<R>(&self, f: impl FnOnce(&mut Self::Target) -> R) -> R;
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with_ref(|world| world.fmt(f))
    }
}

#[derive(Deref, DerefMut)]
pub struct World(pub WorldMutF);

impl TypePath for World {
    fn type_path() -> &'static str {
        "bevy_piccolo::world::WorldMut"
    }
    fn short_type_path() -> &'static str {
        "WorldMut"
    }
}

impl WithRef for World {
    type Target = BWorld;
    fn with_ref<R>(&self, f: impl FnOnce(&Self::Target) -> R) -> R {
        self.with(|w| f(w))
    }
}

impl WithMut for World {
    fn with_mut<R>(&self, f: impl FnOnce(&mut Self::Target) -> R) -> R {
        self.0.with_mut(|w| f(w))
    }
}

fn add_world_methods<'gc, W: WithRef<Target = BWorld> + 'static>(
    ctx: Context<'gc>,
    methods: StaticUserMethods<'gc, W>,
) {
    methods.add("component", ctx, |this, _, _, name: piccolo::String<'_>| {
        this.with_ref(|world| {
            let name = String::from_utf8_lossy(name.as_bytes());
            let registry = world.resource::<AppTypeRegistry>();
            Ok(registry
                .read()
                .get_with_type_path(&name)
                .map(|reg| reg.type_id())
                .and_then(|id| world.components().get_id(id).map(|cid| (cid, id)))
                .map(|(cid, tid)| ComponentId(cid, tid)))
        })
    });
}

fn add_world_mut_methods<'gc, W: WithRef<Target = BWorld> + WithMut + 'static>(
    ctx: Context<'gc>,
    methods: StaticUserMethods<'gc, W>,
) {
    methods.add("hello_mut", ctx, |this, _, _, ()| {
        this.with_mut(|world| {
            let ent = world.spawn_empty().id();
            info!("spawned new entity: {ent:?}");
        });
        Ok(())
    });
    methods.add("query", ctx, |_, _, _, _: ()| {
        Ok(crate::query::Query::default())
    });
}

impl<'gc> IntoValue<'gc> for World {
    fn into_value(self, ctx: Context<'gc>) -> Value<'gc> {
        debug!("converting WorldMut to Value");
        methods::wrap_with(ctx, "bevy_piccolo::World", self, |ctx, methods| {
            add_world_methods(ctx, methods);
            add_world_mut_methods(ctx, methods);
        })
    }
}

impl<'gc> FromValue<'gc> for &'gc World {
    fn from_value(ctx: Context<'gc>, value: Value<'gc>) -> Result<Self, piccolo::TypeError> {
        let ud = value.to_native::<UserData>(ctx)?;
        ud.downcast_static().map_err(|_| piccolo::TypeError {
            expected: "World",
            found: "<unknown>",
        })
    }
}

#[derive(TypePath, Debug, Clone, Copy)]
pub struct ComponentId(pub BComponentId, pub TypeId);

impl<'gc> IntoValue<'gc> for ComponentId {
    fn into_value(self, ctx: Context<'gc>) -> Value<'gc> {
        methods::wrap_with(ctx, "bevy_piccolo::ComponentId", self, |_, _| {})
    }
}

impl<'gc> FromValue<'gc> for ComponentId {
    fn from_value(ctx: Context<'gc>, value: Value<'gc>) -> Result<Self, piccolo::TypeError> {
        let ud = UserData::from_value(ctx, value)?;
        let id = ud
            .downcast_static::<ComponentId>()
            .map_err(|_| piccolo::TypeError {
                expected: "ComponentId",
                found: "<unknown>",
            })?;
        Ok(*id)
    }
}
