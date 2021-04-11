use crate::components::{Gravitational, RigidBody};
use amethyst::{
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::Entity,
    prelude::{Builder, World, WorldExt},
    renderer::sprite::prefab::SpriteScenePrefab,
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct GameObjectPrefab {
    rigid_body: RigidBody,
    gravitational: Gravitational,
    sprite_scene: SpriteScenePrefab,
}

pub fn create_ball_prefab(world: &mut World, progress_counter: &mut ProgressCounter) -> Entity {
    let handle = world.exec(|loader: PrefabLoader<'_, GameObjectPrefab>| {
        loader.load("prefabs/ball.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(handle).build()
}

pub fn create_wall_prefab(world: &mut World, progress_counter: &mut ProgressCounter) -> Entity {
    let handle = world.exec(|loader: PrefabLoader<'_, GameObjectPrefab>| {
        loader.load("prefabs/wall.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(handle).build()
}
