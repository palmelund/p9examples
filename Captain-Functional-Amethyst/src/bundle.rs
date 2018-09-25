use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;
use systems::{PlayerSystem};
/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct CaptainBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CaptainBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
		builder.add(PlayerSystem{counter: 0.0}, "playersystem", &["input_system"]);

        Ok(())
    }
}