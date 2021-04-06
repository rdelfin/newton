use crate::components::RigidBody;
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
};

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
