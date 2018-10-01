use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Enemy, Player, EnemyBullet};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct EnemySystem;
const ENEMYSPEED: f32 = 500.0;
const SHOTTINGBUFFER: f32 = 40.0;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (mut enemies, mut transforms, time): Self::SystemData) {

        for (enemy, transform) in (&mut enemies, &mut transforms).join() {
			if enemy.active{
				transform.translation[0] = (transform.translation[0] - ENEMYSPEED*time.delta_seconds());
				if transform.translation[0] < - 100.0{
					enemy.active = false;
				}
				
			}
			else{
				transform.translation[0] = -100.0;
			}
        }
    }
}