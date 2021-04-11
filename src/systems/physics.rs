use crate::components::{Collider, Gravitational, RigidBody};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, RigidBody>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut rigid_bodies, mut transforms, time): Self::SystemData) {
        // First, update any positions for avaliable velocities
        for rigid_body in (&mut rigid_bodies).join() {
            let frame_delta_s = time.fixed_time().as_secs_f32();
            rigid_body.position += rigid_body.velocity * frame_delta_s;
        }

        // Then, anything with a position gets the velocity updated
        for (rigid_body, transform) in (&rigid_bodies, &mut transforms).join() {
            transform.set_translation_xyz(
                rigid_body.position.x.round(),
                rigid_body.position.y.round(),
                transform.translation().z,
            );
        }
    }
}

#[derive(SystemDesc)]
pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, RigidBody>,
        ReadStorage<'s, Gravitational>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut rigid_bodies, gravitationals, time): Self::SystemData) {
        for (rigid_body, gravitational) in (&mut rigid_bodies, &gravitationals).join() {
            let frame_delta_s = time.fixed_time().as_secs_f32();
            rigid_body.velocity += Vector2::new(0.0, -1.0) * gravitational.0 * frame_delta_s;
        }
    }
}

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (WriteStorage<'s, RigidBody>, ReadStorage<'s, Collider>);

    fn run(&mut self, (mut rigid_bodies, colliders): Self::SystemData) {}
}
