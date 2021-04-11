use crate::{entities, prefabs, resources};
use amethyst::{
    assets::ProgressCounter,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};
use anyhow::Result;

pub struct MyState {
    progress_counter: Option<ProgressCounter>,
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.progress_counter = Some(Default::default());
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);
        load_sprites(world);
        self.load_world(world).unwrap();
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data

        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                // All data loaded
                self.progress_counter = None;
            }
        }
        Trans::None
    }
}

impl MyState {
    pub fn new() -> MyState {
        MyState {
            progress_counter: None,
        }
    }

    fn load_world(&mut self, world: &mut World) -> Result<()> {
        prefabs::create_ball_prefab(world, &mut self.progress_counter.as_mut().unwrap());
        prefabs::create_wall_prefab(world, &mut self.progress_counter.as_mut().unwrap());
        Ok(())
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) {
    let mut sprite_cache = resources::SpriteCache::new();
    sprite_cache.load(resources::SpriteKey::Ball, world);
    sprite_cache.load(resources::SpriteKey::Wall, world);
    world.insert(sprite_cache);
}
