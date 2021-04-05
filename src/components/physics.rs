use amethyst::ecs::{Component, VecStorage};
use nalgebra::Vector2;

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct RigidBody {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}
