use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod components;
mod entities;
mod prefabs;
mod resources;
mod state;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::BallPrefab>::default(),
            "ball_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::WallPrefab>::default(),
            "wall_loader",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.820, 0.886, 1., 1.]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::GravitySystem, "gravity_system", &["ball_loader"])
        .with(
            systems::MovementSystem,
            "movement_system",
            &["ball_loader", "gravity_system"],
        );

    let mut game = Application::new(resources, state::MyState::new(), game_data)?;
    game.run();

    Ok(())
}
