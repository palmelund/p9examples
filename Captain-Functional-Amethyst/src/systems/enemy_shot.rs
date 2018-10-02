use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use captain_functional::{EnemyBullet, Enemy, Player};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct Enemy_Shot;
const SHOTTINGBUFFER: f32 = 40.0;

impl<'s> System<'s> for Enemy_Shot {
    type SystemData = (
        WriteStorage<'s, EnemyBullet>,
		ReadStorage<'s, Player>,
		WriteStorage<'s, Enemy>,
		ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut bullets, players, mut enemies, transforms): Self::SystemData) {
        let mut player_pos_y: f32 = 0.0;
		for (_, transform) in (&players, &transforms).join() {
			player_pos_y = transform.translation[1];
		}

		for (enemy, transform) in (&mut enemies, &transforms).join() {
			if !enemy.fired && transform.translation[1] < player_pos_y + SHOTTINGBUFFER &&  transform.translation[1] > player_pos_y - SHOTTINGBUFFER {
				for bullet in (&mut bullets).join() {
					if !bullet.active{
						enemy.fired = true;
						bullet.active = true;
						bullet.spawn = true;
						bullet.spawn_x = transform.translation[0];
						bullet.spawn_y = transform.translation[1];
					}
				}
			}
        }	
    }
}
