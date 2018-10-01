use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Player_Bullet, ARENA_WIDTH, Enemy, BULLET_SIZE, ENEMY_SIZE};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct BulletCollision;
const SHOTTINGBUFFER: f32 = 40.0;

impl<'s> System<'s> for BulletCollision {
    type SystemData = (
        WriteStorage<'s, EnemyBullet>,
		ReadStorage<'s, Player>,
		ReadStorage<'s, Enemy>,
		WriteStorage<'s, Transform>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut bullets, players, enemies, mut transforms, readTransform): Self::SystemData) {
        let mut player_pos_y: f32 = 0.0;
		for (_, transform) in (&players, &readTransform).join() {
			player_pos_y = transform.translation[1];
		}
		for (enemy, enemy_transform) in (&enemies, &readTransform).join() {
			if !enemy.fired && enemy_transform.translation[1] < player_pos_y + SHOTTINGBUFFER &&  enemy_transform.translation[1] > player_pos_y - SHOTTINGBUFFER {

			}
        }
    }
}
