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
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (enemys, mut locals, time): Self::SystemData) {

        for (enemy, transform) in (&enemys, &mut locals).join() {
			transform.translation[0] = (transform.translation[0] + EnemySpeed*time.delta_seconds())
					.min(1600.0)
					.max(60.0);
			println!("Works");
        }
    }
}