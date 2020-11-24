use amethyst::{
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

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // Get a reference to the application root directory
    let app_root = application_root_dir()?;

    // Get a reference to the assets folder
    let resources = app_root.join("assets");

    // Get a reference to the display configuration file
    let display_config = app_root.join("config/display_config.ron");

    // Get a reference to the input configuration file
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?;

    // Pass the game its resoiufces, game data, and initial state
    let mut game = Application::new(resources, state::MyState, game_data)?;

    // Run - which really just starts the event loop?
    game.run();

    Ok(())
}
