use crate::components::{Gravitational, RigidBody};
use amethyst::{
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    core::Transform,
    derive::PrefabData,
    ecs::Entity,
    prelude::{Builder, World, WorldExt},
    renderer::{
        sprite::prefab::{SpriteRenderPrefab, SpriteScenePrefab},
        Transparent,
    },
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PrefabData)]
// #[serde(deny_unknown_fields)]
pub struct BallPrefab {
    rigid_body: RigidBody,
    gravitational: Gravitational,
    transparent: Transparent,
    sprite_scene: SpriteScenePrefab,
}

pub fn create_ball_prefab(world: &mut World, progress_counter: &mut ProgressCounter) -> Entity {
    let handle = world.exec(|loader: PrefabLoader<'_, BallPrefab>| {
        loader.load("prefabs/ball.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(handle).build()
}
