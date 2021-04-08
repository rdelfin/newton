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
use nalgebra::{Vector2, Vector3};

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
        .with(Gravitational(980.0))
        .build())
}

pub fn new_wall(world: &mut World) -> Result<Entity> {
    let wall_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while crating player."))?;
        sprite_cache.fetch(SpriteKey::Wall)?.clone()
    };

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(100., 1., 1.));

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: wall_handle,
            sprite_number: 0,
        })
        .with(Transparent)
        .with(transform)
        .with(RigidBody {
            position: Vector2::new(885.0, 20.0),
            velocity: Vector2::new(0.0, 0.0),
        })
        .build())
}
