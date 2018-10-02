use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Player_Bullet, ARENA_WIDTH};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct BulletMovement;
const Bullet_Speed: f32 = 500.0;

impl<'s> System<'s> for BulletMovement {
    type SystemData = (
        WriteStorage<'s, Player_Bullet>,
        WriteStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (mut bullets, mut locals, time): Self::SystemData) {
        for (bullet, transform) in (&mut bullets, &mut locals).join() {
			if bullet.active{
				transform.translation[0] = (transform.translation[0] + Bullet_Speed*time.delta_seconds());
				if transform.translation[0] > ARENA_WIDTH + 100.0{
					bullet.active = false;
				}
			}
			else{
				transform.translation[0] = ARENA_WIDTH+100.0;
			}
        }
    }
}