extern crate amethyst;

mod captain_functional;
mod systems;
mod bundle;

use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, Pipeline,
                         PosTex, RenderBundle, Stage, VirtualKeyCode};

use amethyst::core::transform::TransformBundle;
use bundle::CaptainBundle;

fn main() -> Result<(), amethyst::Error> {
    use captain_functional::Captain_Functional;
	use amethyst::input::InputBundle;

	amethyst::start_logger(Default::default());

	let path = "./resources/display_config.ron";

	let config = DisplayConfig::load(&path);

	let pipe = Pipeline::build().with_stage(
    Stage::with_backbuffer()
        .clear_target([0.5, 0.5, 1.0, 1.0], 1.0)
        .with_pass(DrawFlat::<PosTex>::new()),
	);

	

	let binding_path = format!(
		"{}/resources/bindings_config.ron",
		env!("CARGO_MANIFEST_DIR")
	);

	let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

	let game_data = GameDataBuilder::default()
		.with_bundle(TransformBundle::new())?
		.with_bundle(RenderBundle::new(pipe, Some(config)))?
		.with_bundle(input_bundle)?
		.with_bundle(CaptainBundle)?;
	let mut game = Application::new("./", Captain_Functional, game_data)?;
	game.run();
    Ok(())
}
