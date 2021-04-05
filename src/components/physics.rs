use amethyst::ecs::{Component, VecStorage};
use nalgebra::Vector2;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct RigidBody {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
}
