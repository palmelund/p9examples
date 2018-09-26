use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::timing::Time;
use amethyst::input::InputHandler;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage,Entities};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Player, Player_Bullet, PLAYER_SIZE};
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};
/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct PlayerSystem{
	pub counter: f32,
}
const PlayerSpeed: f32 = 500.0;
const FireRate: f32 = 0.8;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
		WriteStorage<'s, Player_Bullet>,
        Read<'s,  InputHandler<String, String>>,
		Read<'s,  Time>,
		Entities<'s>,
    );

    fn run(&mut self, (players, mut locals, mut bullets, input, time, entities): Self::SystemData) {
		let x = input.axis_value("player_x_axes");
		let y = input.axis_value("player_y_axes");
		let space = input.key_is_down(VirtualKeyCode::Space);
		self.counter += time.delta_seconds();

        for (player, transform) in (&players, &mut locals).join() {
			if let Some(y_amount) = y{
				transform.translation[1] = (transform.translation[1] + ((y_amount as f32)*time.delta_seconds() * PlayerSpeed))
						.min(800.0)
						.max(60.0);
			}
			if let Some(x_amount) = x{
				transform.translation[0] = (transform.translation[0] + ((x_amount as f32)*time.delta_seconds() * PlayerSpeed))
						.min(1600.0)
						.max(60.0);
			}
			let currentTime: u128 = ((time.absolute_time().as_secs()*1000) + (time.absolute_time().subsec_millis() as u64)) as u128;
			
			if space && self.counter > FireRate {
				self.counter = 0.0;
				// Load the spritesheet necessary to render the graphics.
				
				let bullet = Player_Bullet{
					height: PLAYER_SIZE,
					width: PLAYER_SIZE,
				};

				
			}
        }
    }
}