use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{EnemyBullet, ARENA_WIDTH, Enemy, BULLET_SIZE, ENEMY_SIZE};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct BulletCollision;
const Bullet_Speed: f32 = 500.0;

impl<'s> System<'s> for BulletCollision {
    type SystemData = (
        WriteStorage<'s, EnemyBullet>,
        ReadStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (mut bullets, mut enemies,  transforms, time): Self::SystemData) {
        for (bullet, bullet_trans) in (&mut bullets, &transforms).join() {
			if bullet.active{
				for (enemy, enemy_trans) in (&mut enemies, &transforms).join() {
					if enemy.active{
						if rect_overlap(bullet_trans.translation[0], bullet_trans.translation[1], BULLET_SIZE, enemy_trans.translation[0], enemy_trans.translation[1], ENEMY_SIZE){
							println!("HIT!!");
							println!("x1:{} y1:{}, x2:{} y2{}",bullet_trans.translation[0], bullet_trans.translation[1], enemy_trans.translation[0], enemy_trans.translation[1]);
							bullet.active = false;
							enemy.active = false;
						}
					}
				}
			}
        }
    }
}
