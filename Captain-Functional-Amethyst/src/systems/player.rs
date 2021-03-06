use amethyst::core::timing::Time;
use amethyst::input::InputHandler;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use captain_functional::{Player, GameStats, Player_Bullet};
use amethyst::renderer::{ VirtualKeyCode };
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
		Read<'s,  GameStats>,

    );

    fn run(&mut self, (players, mut transforms, mut bullets, input, time, gamestats): Self::SystemData) {
		let mut shotBullet = false;
		let mut bulletX = 0.0;
		let mut bulletY = 0.0;
		let x = input.axis_value("player_x_axes");
		let y = input.axis_value("player_y_axes");
		let space = input.key_is_down(VirtualKeyCode::Space);
		self.counter += time.delta_seconds();
		if gamestats.player_health < -3 {
			println!("YOUR LOST!!")
		}
        for (player, transform) in (&players, &mut transforms).join() {
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
			
			if space && self.counter > FireRate {
				self.counter = 0.0;
				
				shotBullet = true;
				bulletX = transform.translation[0];
				bulletY = transform.translation[1];
			}
        }
		if shotBullet {
			for (bullet, transform) in (&mut bullets, &mut transforms).join(){
				if !bullet.active{
					transform.translation[0] = bulletX;
					transform.translation[1] = bulletY;
					bullet.active = true;
					break;
				}	
			}	
		}
		
    }

}