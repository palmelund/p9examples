use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use captain_functional::{EnemyBullet, ARENA_WIDTH};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct Enemy_Bullet_Movement;
const Bullet_Speed: f32 = 700.0;

impl<'s> System<'s> for Enemy_Bullet_Movement {
    type SystemData = (
        WriteStorage<'s, EnemyBullet>,
        WriteStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (mut bullets, mut transforms, time): Self::SystemData) {
        for (bullet, transform) in (&mut bullets, &mut transforms).join() {
			if bullet.active && bullet.spawn{
				bullet.spawn = false;
				transform.translation[0] = bullet.spawn_x;
				transform.translation[1] = bullet.spawn_y;
			}
			if bullet.active{
				transform.translation[0] = (transform.translation[0] - Bullet_Speed*time.delta_seconds());
				if transform.translation[0] < -100.0{
					bullet.active = false;
				}
			}
			else{
				transform.translation[0] = ARENA_WIDTH+100.0;
			}
        }
    }
}