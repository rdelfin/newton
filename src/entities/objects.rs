use crate::{
    components::{Gravitational, RigidBody},
    resources::{SpriteCache, SpriteKey},
};
use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use anyhow::{anyhow, Result};
use nalgebra::Vector2;

pub fn new_ball(world: &mut World) -> Result<Entity> {
    let ball_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while crating player."))?;
        sprite_cache.fetch(SpriteKey::Ball)?.clone()
    };

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: ball_handle,
            sprite_number: 0,
        })
        .with(Transparent)
        .with(Transform::default())
        .with(RigidBody {
            position: Vector2::new(885.0, 500.0),
            velocity: Vector2::new(0.0, 0.0),
        })
        .with(Gravitational(98.0))
        .build())
}
