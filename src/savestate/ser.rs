use std::borrow::Cow;

use bevy::{
  prelude::*,
  reflect::{
    serde::{
      TypedReflectDeserializer,
      TypedReflectSerializer,
    },
    TypeRegistryArc,
  },
  scene::DynamicEntity,
  utils::HashMap,
};
use serde::{
  de::DeserializeSeed,
  Serialize,
};

use super::DbEntity;

pub fn serialize_ron<S>(serialize: S) -> Result<String, ron::Error>
where
  S: Serialize,
{
  ron::ser::to_string(&serialize)
}

#[allow(dead_code)]
// Component name -> serialized component
pub type SerializedComponents<'a> = HashMap<Cow<'a, str>, String>;
#[allow(dead_code)]
// Entity -> Components
pub type SerializedEntities<'a> = HashMap<DbEntity, SerializedComponents<'a>>;

#[allow(dead_code)]
pub fn deserialize_entity(
  type_registry: &TypeRegistryArc,
  components: &SerializedComponents,
) -> Result<Vec<Box<dyn Reflect>>, ron::Error> {
  debug!(?components, "deserializing entity");
  let components = components
    .iter()
    .map(|(name, serialized)| deserialize_component(type_registry, name, serialized))
    .collect::<Result<_, ron::Error>>()?;
  Ok(components)
}

pub fn serialize_component(
  type_registry: &TypeRegistryArc,
  component: &dyn Reflect,
) -> Result<(Cow<'static, str>, String), ron::Error> {
  let name = component
    .get_represented_type_info()
    .map(|i| Cow::Borrowed(i.type_path()))
    .unwrap_or_else(|| Cow::Owned(component.reflect_type_path().to_string()));
  Ok((
    name,
    serialize_ron(TypedReflectSerializer::new(
      component,
      &type_registry.read(),
    ))?,
  ))
}

pub fn serialize_components(
  type_registry: &TypeRegistryArc,
  components: &[Box<dyn Reflect>],
) -> Result<SerializedComponents<'static>, ron::Error> {
  components
    .iter()
    .map(AsRef::as_ref)
    .map(|c| serialize_component(type_registry, c))
    .collect()
}

#[allow(dead_code)]
pub fn serialize_entity(
  type_registry: &TypeRegistryArc,
  entity: &DynamicEntity,
) -> Result<(Entity, SerializedComponents<'static>), ron::Error> {
  serialize_components(type_registry, &entity.components).map(|c| (entity.entity, c))
}

#[allow(dead_code)]
pub fn serialize_entities(
  type_registry: &TypeRegistryArc,
  entities: &[DynamicEntity],
) -> Result<SerializedEntities<'static>, ron::Error> {
  entities
    .iter()
    .map(|entity| {
      serialize_components(type_registry, &entity.components).map(|c| (DbEntity(entity.entity), c))
    })
    .collect()
}

#[allow(dead_code)]
pub fn deserialize_component(
  type_registry: &TypeRegistryArc,
  name: &str,
  value: &str,
) -> Result<Box<dyn Reflect>, ron::Error> {
  let type_registry = type_registry.read();
  let registration =
    type_registry
      .get_with_type_path(name)
      .ok_or_else(|| ron::Error::NoSuchStructField {
        expected: &["a valid component"],
        found: name.to_string(),
        outer: None,
      })?;
  let deserializer = TypedReflectDeserializer::new(registration, &type_registry);
  let mut seed = ron::Deserializer::from_str(value)?;
  deserializer.deserialize(&mut seed)
}
