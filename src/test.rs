use std::{
    future::Future,
    thread,
    time::{
        Duration,
        Instant,
    },
};

use bevy::{
    app::MainScheduleOrder,
    tasks::{
        IoTaskPool,
        TaskPool,
    },
};
use tracing_test::traced_test;

use super::*;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

const TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Component, Reflect, Debug, Eq, PartialEq)]
#[reflect(Component)]
struct Foo {
    pub bar: String,
    pub baz: u32,
}

const FOO_NAME: &str = "bevy_sqlite::test::Foo";

fn block_on<T: Send + 'static>(f: impl Future<Output = T> + Send + 'static) -> T {
    super::block_on(IoTaskPool::get_or_init(TaskPool::new).spawn(f))
}

fn wait_load<T: Component>(app: &mut App, ent: Entity) -> &T {
    let start = Instant::now();

    loop {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to load");
        }

        app.update();

        if app.world.entity(ent).contains::<T>() {
            break;
        };
    }

    app.world.get::<T>(ent).unwrap()
}

fn wait_save(app: &mut App, ent: Entity) -> i64 {
    let start = Instant::now();

    let idx = loop {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }

        app.update();

        if let Some(db_entity) = app.world.get::<DbEntity>(ent) {
            break db_entity.to_index();
        };
    };

    idx
}

fn wait_save_component<T: Component>(app: &mut App, ent: Entity) -> &T {
    let start = Instant::now();

    loop {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }

        app.update();

        let ent = app.world.entity(ent);

        if ent.contains::<Dirty<T>>() || ent.contains::<Saving<T>>() {
            continue;
        }
        break;
    }

    app.world.get::<T>(ent).unwrap()
}

fn setup(db: Option<Db>) -> Result<(App, Db), BoxError> {
    let db = match db {
        Some(db) => db,
        _ => block_on(async {
            let db = Db::connect_lazy("sqlite::memory:")?;

            sqlx::query(include_str!("../schema.sql"))
                .execute(&*db)
                .await?;
            Result::<_, BoxError>::Ok(db)
        })?,
    };

    let mut app = App::new();
    app.add_plugins(SqlitePlugin);
    app.persist_component::<Foo>();
    app.insert_resource(db.clone());
    app.update();
    Ok((app, db))
}

#[test]
#[traced_test]
fn save_entity() -> Result<(), BoxError> {
    let (mut app, _) = setup(None)?;

    // Entity saves to db
    let ent = app.world.spawn_empty().id();

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(ent));
    app.world.entity_mut(ent).insert(task);

    assert!(app.world.entity(ent).contains::<Creating>());

    let start = Instant::now();

    while !app.world.entity(ent).contains::<DbEntity>() {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    }

    assert!(!app.world.entity(ent).contains::<Creating>());
    Ok(())
}

#[traced_test]
#[test]
// Entity saves to db with component
fn save_with_component() -> Result<(), BoxError> {
    let (mut app, db) = setup(None)?;

    let ent = app
        .world
        .spawn(Foo {
            bar: "spam".into(),
            baz: 42,
        })
        .id();

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(ent));
    app.world.entity_mut(ent).insert(task);

    let start = Instant::now();

    let idx = loop {
        if let Some(db_entity) = app.world.get::<DbEntity>(ent) {
            break db_entity.to_index();
        };
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    };

    let conn = db.clone();
    let data = block_on(async move {
        sqlx::query!(
            r#"
                select ec.data from entity_component ec, component c
                where ec.component = c.id
                and ec.entity = ?
                and c.name = ?
            "#,
            idx,
            FOO_NAME,
        )
        .fetch_one(&*conn)
        .await
    })?
    .data;

    assert_eq!(r#"(bar:"spam",baz:42)"#, data);
    Ok(())
}

#[traced_test]
#[test]
// Entity saves to db with component added a frame later
fn late_add() -> Result<(), BoxError> {
    let (mut app, db) = setup(None)?;

    let ent = app.world.spawn_empty().id();

    // This is effectively an end-of-frame, since it's a world sync point.
    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(ent));
    app.world.entity_mut(ent).insert(task);

    app.world.entity_mut(ent).insert(Foo {
        bar: "spam".into(),
        baz: 42,
    });

    app.update();

    let start = Instant::now();

    let idx = loop {
        if let Some(db_entity) = app.world.get::<DbEntity>(ent) {
            break db_entity.to_index();
        };
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    };

    let start = Instant::now();

    while app.world.entity(ent).contains::<Saving<Foo>>()
        || app.world.entity(ent).contains::<Dirty<Foo>>()
    {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    }

    let conn = db.clone();
    let data = block_on(async move {
        sqlx::query!(
            r#"
                select ec.data from entity_component ec, component c
                where ec.component = c.id
                and ec.entity = ?
                and c.name = ?
            "#,
            idx,
            FOO_NAME,
        )
        .fetch_one(&*conn)
        .await
    })?
    .data;

    assert_eq!(r#"(bar:"spam",baz:42)"#, data);
    Ok(())
}

#[traced_test]
#[test]
fn delete() -> Result<(), BoxError> {
    let (mut app, db) = setup(None)?;

    let ent = app
        .world
        .spawn(Foo {
            bar: "spam".into(),
            baz: 42,
        })
        .id();

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(ent));
    app.world.entity_mut(ent).insert(task);

    let start = Instant::now();

    let idx = loop {
        if let Some(db_entity) = app.world.get::<DbEntity>(ent) {
            break db_entity.to_index();
        };
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    };

    app.world.entity_mut(ent).remove::<Foo>();

    app.update();

    let start = Instant::now();

    loop {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for component to delete");
        }

        let conn = db.clone();
        let data = block_on(async move {
            sqlx::query!(
                r#"
                select ec.* from entity_component ec, component c
                where ec.component = c.id
                and ec.entity = ?
                and c.name = ?
            "#,
                idx,
                FOO_NAME,
            )
            .fetch_one(&*conn)
            .await
        });

        match data {
            Ok(data) => {
                info!(?data, "still seeing foo");
            }
            Err(_) => break,
        }

        app.update();
    }
    Ok(())
}

#[traced_test]
#[test]
fn load() -> Result<(), BoxError> {
    let (mut app, db) = setup(None)?;

    let ent = app
        .world
        .spawn(Foo {
            bar: "spam".into(),
            baz: 42,
        })
        .id();

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(ent));
    app.world.entity_mut(ent).insert(task);

    let start = Instant::now();

    let idx = loop {
        if let Some(db_entity) = app.world.get::<DbEntity>(ent) {
            break db_entity.to_index();
        };
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    };

    let (mut app, _) = setup(Some(db))?;

    let ent = app.world.db_entity(DbEntity::from_index(idx)).id();

    app.update();

    assert!(app.world.entity(ent).contains::<Loading>());

    let start = Instant::now();

    while app.world.entity(ent).contains::<Loading>() {
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for load");
        }

        app.update();
    }

    assert_eq!(
        &Foo {
            bar: "spam".into(),
            baz: 42,
        },
        app.world
            .entity(ent)
            .get::<Foo>()
            .expect("should have a foo")
    );

    Ok(())
}

#[traced_test]
#[test]
fn despawn() -> Result<(), BoxError> {
    let (mut app, db) = setup(None)?;

    let ent = app
        .world
        .spawn(Foo {
            bar: "spam".into(),
            baz: 42,
        })
        .id();

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(ent));
    app.world.entity_mut(ent).insert(task);

    let start = Instant::now();

    let idx = loop {
        if let Some(db_entity) = app.world.get::<DbEntity>(ent) {
            break db_entity.to_index();
        };
        if Instant::now().duration_since(start) > TIMEOUT {
            panic!("timed out waiting for entity to save");
        }
        app.update();
    };

    app.world.despawn(ent);

    for _ in 0..10 {
        thread::sleep(Duration::from_millis(100));
        app.update();
    }

    let conn = db.clone();
    let res = block_on(async move {
        sqlx::query!(
            r#"
                select * from entity_component
                where entity = ?
            "#,
            idx
        )
        .fetch_all(&*conn)
        .await
    })?;

    assert!(!res.is_empty());

    Ok(())
}

#[traced_test]
#[test]
fn hierarchy() -> Result<(), BoxError> {
    let (mut app, db) = setup(None)?;

    app.add_plugins(HierarchyPlugin);
    app.persist_component::<Children>();
    app.persist_component::<Parent>();

    info!("saving initial parent/child");

    let parent = app.world.spawn_empty().id();

    let child = app.world.spawn_empty().id();

    app.world.entity_mut(parent).add_child(child);

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(parent));
    app.world.entity_mut(parent).insert(task);

    let task = SaveDb::from_world(&app.world).save_entity(app.world.entity(child));
    app.world.entity_mut(child).insert(task);

    let parent_idx = wait_save(&mut app, parent);
    let _child_idx = wait_save(&mut app, child);

    let conn = db.clone();
    let res = block_on(async move {
        sqlx::query!(
            r#"
                select * from entity_component
            "#,
        )
        .fetch_all(&*conn)
        .await
    });

    info!(?res);

    info!("creating new app and loading parent");

    let (mut app, db) = setup(Some(db))?;

    app.add_plugins(HierarchyPlugin);
    app.persist_component::<Children>();
    app.persist_component::<Parent>();

    for _ in 0..20 {
        app.world.spawn_empty();
    }

    let parent = app.world.db_entity(DbEntity::from_index(parent_idx)).id();

    let loaded_children = wait_load::<Children>(&mut app, parent)
        .into_iter()
        .copied()
        .collect::<Vec<_>>();

    info!(?parent, ?loaded_children, "loaded parent with children");

    let child = loaded_children[0];

    app.world.load(child);

    let loaded_parent = wait_load::<Parent>(&mut app, child).get();

    info!(?parent, ?child, ?loaded_children, ?loaded_parent,);

    assert_eq!(parent, loaded_parent);
    assert!(loaded_children.contains(&child));

    info!("swapping children and waiting for save");

    let child_two = app.world.spawn_empty().id();
    app.world
        .entity_mut(parent)
        .add_child(child_two)
        .remove_children(&[child]);

    wait_save_component::<Children>(&mut app, parent);

    info!("creating new app");

    let (mut app, _db) = setup(Some(db))?;

    app.add_plugins(HierarchyPlugin);
    app.persist_component::<Children>();
    app.persist_component::<Parent>();

    let parent = app.world.db_entity(DbEntity::from_index(parent_idx)).id();

    let children = wait_load::<Children>(&mut app, parent);

    info!(?children, ?parent);

    let child = children[0];

    assert!(app.world.get_entity(child).is_none());

    Ok(())
}
