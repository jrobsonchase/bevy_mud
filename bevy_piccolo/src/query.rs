use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{
    anyhow,
    Context as _,
};
use bevy::ecs::entity::Entity as BEntity;
use bevy::ecs::query::{
    QueryBuilder,
    QueryData,
    QueryState as BQueryState,
};
use bevy::ecs::reflect::{
    AppTypeRegistry,
    ReflectComponent,
};
use bevy::prelude::Deref;
use bevy::reflect::serde::{
    ReflectSerializer,
    TypedReflectDeserializer,
};
use bevy::{
    ecs::component::ComponentId,
    reflect::TypePath,
};
use piccolo::table::NextValue;
use piccolo::{
    Callback,
    CallbackReturn,
    Context,
    UserData,
    Value,
};
use piccolo::{
    FromValue,
    Table,
};
use piccolo::{
    IntoValue,
    MetaMethod,
};
use serde::de::DeserializeSeed;

use crate::methods;
use crate::world::World;

#[derive(Clone, Debug, Deref, Default)]
pub struct Query(Rc<RefCell<QueryInner>>);

impl TypePath for Query {
    fn type_path() -> &'static str {
        "bevy_piccolo::query::Query"
    }
    fn short_type_path() -> &'static str {
        "Query"
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryInner {
    with: Vec<ComponentId>,
    without: Vec<ComponentId>,
    or: Vec<Query>,
    and: Vec<Query>,
    optional: Vec<Query>,
}

impl QueryInner {
    fn with(&mut self, id: ComponentId) -> &mut Self {
        self.with.push(id);
        self
    }
    fn without(&mut self, id: ComponentId) -> &mut Self {
        self.without.push(id);
        self
    }
    fn and(&mut self, query: Query) -> &mut Self {
        self.and.push(query);
        self
    }
    fn or(&mut self, query: Query) -> &mut Self {
        self.or.push(query);
        self
    }
    fn optional(&mut self, query: Query) -> &mut Self {
        self.optional.push(query);
        self
    }

    fn apply<'gc, T: QueryData>(&self, q: &mut QueryBuilder<T>) {
        for id in &self.with {
            q.with_id(*id);
        }
        for id in &self.without {
            q.without_id(*id);
        }
        if !self.and.is_empty() {
            q.and(|q| {
                for and in &self.and {
                    and.borrow().apply(q);
                }
            });
        }
        if !self.or.is_empty() {
            q.or(|q| {
                for or in &self.or {
                    or.borrow().apply(q);
                }
            });
        }
        if !self.optional.is_empty() {
            q.optional(|q| {
                for optional in &self.optional {
                    optional.borrow().apply(q);
                }
            });
        }
    }
}

impl<'gc> IntoValue<'gc> for Query {
    fn into_value(self, ctx: Context<'gc>) -> Value<'gc> {
        methods::wrap_with(ctx, "bevy_piccolo::Query", self, |ctx, methods| {
            let methods_raw =
                Table::from_value(ctx, methods.metatable(ctx).get(ctx, MetaMethod::Index)).unwrap();

            methods_raw
                .set(
                    ctx,
                    "with",
                    Callback::from_fn(&ctx, |ctx, _exec, mut stack| {
                        let (this, id): (Value, crate::world::ComponentId) = stack.consume(ctx)?;
                        let query = <&Query>::from_value(ctx, this)?;
                        query.borrow_mut().with(id.0);
                        stack.replace(ctx, this);
                        Ok(CallbackReturn::Return)
                    }),
                )
                .unwrap();
            methods_raw
                .set(
                    ctx,
                    "without",
                    Callback::from_fn(&ctx, |ctx, _exec, mut stack| {
                        let (this, id): (Value, crate::world::ComponentId) = stack.consume(ctx)?;
                        let query = <&Query>::from_value(ctx, this)?;
                        query.borrow_mut().without(id.0);
                        stack.replace(ctx, this);
                        Ok(CallbackReturn::Return)
                    }),
                )
                .unwrap();
            methods_raw
                .set(
                    ctx,
                    "also",
                    Callback::from_fn(&ctx, |ctx, _exec, mut stack| {
                        let (this, other): (Value, &Query) = stack.consume(ctx)?;
                        let query = <&Query>::from_value(ctx, this)?;
                        query.borrow_mut().and(other.clone());
                        stack.replace(ctx, this);
                        Ok(CallbackReturn::Return)
                    }),
                )
                .unwrap();
            methods_raw
                .set(
                    ctx,
                    "or",
                    Callback::from_fn(&ctx, |ctx, _exec, mut stack| {
                        let (this, other): (Value, &Query) = stack.consume(ctx)?;
                        let query = <&Query>::from_value(ctx, this)?;
                        query.borrow_mut().or(other.clone());
                        stack.replace(ctx, this);
                        Ok(CallbackReturn::Return)
                    }),
                )
                .unwrap();

            methods_raw
                .set(
                    ctx,
                    "optional",
                    Callback::from_fn(&ctx, |ctx, _exec, mut stack| {
                        let (this, other): (Value, &Query) = stack.consume(ctx)?;
                        let query = <&Query>::from_value(ctx, this).context("this query")?;
                        query.borrow_mut().optional(other.clone());
                        stack.replace(ctx, this);
                        Ok(CallbackReturn::Return)
                    }),
                )
                .unwrap();

            methods.add("build", ctx, |this, ctx, _, _: ()| {
                let this = this.borrow();
                let ud = UserData::from_value(ctx, ctx.get_global("WORLD"))?;
                let world = ud.downcast_static::<crate::world::World>()?;
                let state = world.with_mut(|world| {
                    let mut q = QueryBuilder::<BEntity>::new(world);
                    this.apply(&mut q);
                    q.build()
                });
                Ok(QueryState(RefCell::new(state)))
            });
        })
    }
}

impl<'gc> FromValue<'gc> for &'gc Query {
    fn from_value(ctx: Context<'gc>, value: Value<'gc>) -> Result<Self, piccolo::TypeError> {
        let ud: UserData = UserData::from_value(ctx, value)?;
        let query = ud
            .downcast_static::<Query>()
            .map_err(|_| piccolo::TypeError {
                expected: "Query",
                found: "<unknown>",
            })?;
        Ok(query)
    }
}

#[derive(Debug)]
pub struct QueryState(RefCell<BQueryState<BEntity>>);

impl TypePath for QueryState {
    fn type_path() -> &'static str {
        "bevy_piccolo::query::QueryState"
    }
    fn short_type_path() -> &'static str {
        "QueryState"
    }
}

impl<'gc> IntoValue<'gc> for QueryState {
    fn into_value(self, ctx: Context<'gc>) -> Value<'gc> {
        methods::wrap_with(ctx, "bevy_piccolo::QueryState", self, |ctx, methods| {
            let methods_raw =
                Table::from_value(ctx, methods.metatable(ctx).get(ctx, MetaMethod::Index)).unwrap();

            methods_raw
                .set(
                    ctx,
                    "run",
                    Callback::from_fn(&ctx, |ctx, _, mut stack| {
                        let this_ud: UserData = stack.consume(ctx)?;
                        let this = this_ud.downcast_static::<QueryState>().unwrap();
                        let mut this = this.0.borrow_mut();
                        let world = ctx.get_global("WORLD").to_native::<&World>(ctx)?;
                        let ents: Table = world
                            .with(|world| Table::from_iter(ctx, this.iter(world).map(Entity)))?;

                        stack.replace(ctx, ents);

                        Ok(CallbackReturn::Return)
                    }),
                )
                .unwrap();
        })
    }
}

#[derive(TypePath, Debug, Clone, Copy, Deref)]
pub struct Entity(pub BEntity);

impl<'gc> IntoValue<'gc> for Entity {
    fn into_value(self, ctx: Context<'gc>) -> Value<'gc> {
        methods::wrap_with(ctx, "bevy_piccolo::Entity", self, |ctx, methods| {
            methods.add(
                "set",
                ctx,
                |this, ctx, _, (component_id, value): (crate::world::ComponentId, Value)| {
                    let world = ctx
                        .get_global("WORLD")
                        .to_native::<&World>(ctx)
                        .context("get world")?;
                    let de = piccolo_util::serde::de::Deserializer::from_value(value);
                    world.with_mut(|world| {
                        let registry = world.resource::<AppTypeRegistry>().clone();
                        let registry = registry.read();
                        let reg_entry = registry.get(component_id.1).unwrap();
                        let seed = TypedReflectDeserializer::new(reg_entry, &registry);
                        let data = seed.deserialize(de)?;

                        let mut entity = world
                            .get_entity_mut(this.0)
                            .ok_or_else(|| anyhow!("no entity found with id {this:?}"))?;

                        reg_entry
                            .data::<ReflectComponent>()
                            .unwrap()
                            .apply_or_insert(&mut entity, &*data, &registry);
                        Ok(())
                    })
                },
            );
            methods.add(
                "get",
                ctx,
                |this, ctx, _, component_id: crate::world::ComponentId| {
                    let world = ctx
                        .get_global("WORLD")
                        .to_native::<&World>(ctx)
                        .context("get world")?;
                    world.with(|world| {
                        let entity = world
                            .get_entity(this.0)
                            .ok_or_else(|| anyhow!("no entity found with id {this:?}"))?;
                        let registry = world.resource::<AppTypeRegistry>().read();
                        let reg_entry = registry.get(component_id.1).unwrap();
                        let reflect_component = match reg_entry
                            .data::<ReflectComponent>()
                            .unwrap()
                            .reflect(entity)
                        {
                            Some(c) => c,
                            None => return Ok(None),
                        };
                        let value = piccolo_util::serde::to_value(
                            ctx,
                            &ReflectSerializer {
                                value: reflect_component,
                                registry: &registry,
                            },
                        )?;
                        let value = match Table::from_value(ctx, value)?.next(Value::Nil) {
                            NextValue::Found { value, .. } => value,
                            _ => panic!("component had no content"),
                        };

                        Ok(Some(value))
                    })
                },
            );
        })
    }
}

impl<'gc> FromValue<'gc> for Entity {
    fn from_value(ctx: Context<'gc>, value: Value<'gc>) -> Result<Self, piccolo::TypeError> {
        let ud = UserData::from_value(ctx, value)?;
        let id = ud
            .downcast_static::<Entity>()
            .map_err(|_| piccolo::TypeError {
                expected: "Entity",
                found: "<unknown>",
            })?;
        Ok(*id)
    }
}
