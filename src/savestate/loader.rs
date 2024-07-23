use std::{
  any::TypeId,
  fmt,
};

use async_std::io::ReadExt;
use bevy::{
  asset::{
    io::Reader,
    AssetLoader,
    LoadContext,
  },
  prelude::*,
  reflect::{
    TypeInfo,
    TypeRegistry,
  },
  scene::serde::{
    SceneEntitiesDeserializer,
    SceneMapDeserializer,
  },
  utils::ConditionalSendFuture,
};
use serde::{
  de::{
    DeserializeSeed,
    MapAccess,
    SeqAccess,
    Visitor,
  },
  Deserialize,
};

use super::entity::{
  Save,
  SavedEntity,
};

pub struct SavedEntityLoader {
  registry: AppTypeRegistry,
}

impl FromWorld for SavedEntityLoader {
  fn from_world(world: &mut World) -> Self {
    let registry = world.resource::<AppTypeRegistry>().clone();

    SavedEntityLoader { registry }
  }
}

impl AssetLoader for SavedEntityLoader {
  type Asset = SavedEntity;
  type Settings = ();
  type Error = anyhow::Error;

  /// Asynchronously loads [`AssetLoader::Asset`] (and any other labeled assets) from the bytes provided by [`Reader`].
  fn load<'a>(
    &'a self,
    reader: &'a mut Reader,
    _settings: &'a Self::Settings,
    load_context: &'a mut LoadContext,
  ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
    async {
      let mut bytes = vec![];
      reader.read_to_end(&mut bytes).await?;
      let mut de = ron::Deserializer::from_bytes(&bytes)?;

      let registry = self.registry.read();

      let seed = SavedEntitySeed {
        registry: &registry,
      };

      let mut entity = seed.deserialize(&mut de)?;

      for child in entity.entities.values_mut() {
        let mut child_handle = None;
        for child_component in child.iter() {
          let Some(type_info) = child_component.get_represented_type_info() else {
            continue;
          };
          if TypeInfo::type_id(type_info) == TypeId::of::<Save>() {
            let mut save = Save::default();
            save.apply(&**child_component);
            child_handle = Some(load_context.load::<SavedEntity>(Clone::clone(&*save)));
          }
        }

        if let Some(handle) = child_handle {
          child.push(Box::new(handle));
        }
      }

      Ok(entity)
    }
  }
}

pub struct SavedEntitySeed<'a> {
  registry: &'a TypeRegistry,
}

impl<'de> DeserializeSeed<'de> for SavedEntitySeed<'de> {
  type Value = SavedEntity;
  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_struct(
      "SavedEntity",
      &["components", "entities"],
      SavedEntityVisitor {
        registry: self.registry,
      },
    )
  }
}

pub struct SavedEntityVisitor<'a> {
  registry: &'a TypeRegistry,
}

impl<'de> Visitor<'de> for SavedEntityVisitor<'de> {
  type Value = SavedEntity;
  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("saved entity struct")
  }

  fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
  where
    A: SeqAccess<'de>,
  {
    let components = seq
      .next_element_seed(SceneMapDeserializer {
        registry: self.registry,
      })?
      .unwrap_or_default();

    let entities = seq
      .next_element_seed(SceneEntitiesDeserializer {
        type_registry: self.registry,
      })?
      .unwrap_or_default();

    Ok(SavedEntity {
      components,
      entities: entities
        .into_iter()
        .map(|ent| (ent.entity, ent.components))
        .collect(),
    })
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: MapAccess<'de>,
  {
    let mut components = vec![];
    let mut entities = vec![];

    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum SavedEntityField {
      Components,
      Entities,
    }

    use SavedEntityField::*;

    while let Some(key) = map.next_key()? {
      match key {
        Components => {
          components.extend(map.next_value_seed(SceneMapDeserializer {
            registry: self.registry,
          })?);
        }
        Entities => {
          entities.extend(map.next_value_seed(SceneEntitiesDeserializer {
            type_registry: self.registry,
          })?);
        }
      }
    }

    Ok(SavedEntity {
      components,
      entities: entities
        .into_iter()
        .map(|ent| (ent.entity, ent.components))
        .collect(),
    })
  }
}
