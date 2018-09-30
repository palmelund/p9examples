use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Enemy};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct EnemySystem;
const EnemySpeed: f32 = 500.0;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (mut enemys, mut locals, time): Self::SystemData) {

        for (enemy, transform) in (&mut enemys, &mut locals).join() {
			if enemy.active{
				transform.translation[0] = (transform.translation[0] - EnemySpeed*time.delta_seconds());
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