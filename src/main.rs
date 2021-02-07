use amethyst::utils::application_root_dir;
use amethyst::prelude::*;
use amethyst::renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::input::{InputBundle, StringBindings};
use amethyst::core::transform::TransformBundle;
mod spades;
mod bundle;
mod systems;

use crate::spades::Spades;
use crate::bundle::SpadesBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(SpadesBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let assets_dir = app_root.join("assets/");
    let mut game = Application::new(assets_dir, Spades::default(), game_data)?;

    game.run();
    Ok(())
}
