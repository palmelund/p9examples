use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage, Write};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Player, Enemy, PLAYER_SIZE, EnemyBullet, ENEMY_SIZE, GameStats, ENEMY_SCORE_REWARD, Boss, Boss_Shield, BOSS_SHIELD_SIZE, BOSS_SIZE, BULLET_SIZE};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct Player_Collision;

impl<'s> System<'s> for Player_Collision {
    type SystemData = (
        WriteStorage<'s, Player>,
		WriteStorage<'s, Enemy>,
		WriteStorage<'s, EnemyBullet>,
		WriteStorage<'s, Boss>,
		WriteStorage<'s, Boss_Shield>,
        ReadStorage<'s, Transform>,
		Write<'s, GameStats>,
    );

    fn run(&mut self, (mut players, mut enemies, mut enemy_bullets, mut bosses, mut boss_shields,  transforms, mut game_stats): Self::SystemData) {
        for (player, player_trans) in (&mut players, &transforms).join() {
			for (enemy, enemy_trans) in (&mut enemies, &transforms).join() {
				if enemy.active && rect_overlap(player_trans.translation[0], player_trans.translation[1], PLAYER_SIZE, enemy_trans.translation[0], enemy_trans.translation[1], ENEMY_SIZE){
					enemy.active = false;
					game_stats.score += ENEMY_SCORE_REWARD;
					game_stats.player_health -= 1;
				}
			}
			for (bullet, bullet_trans) in (&mut enemy_bullets, &transforms).join() {
				if bullet.active && rect_overlap(player_trans.translation[0], player_trans.translation[1], PLAYER_SIZE, bullet_trans.translation[0], bullet_trans.translation[1], BULLET_SIZE){
					bullet.active = false;
					game_stats.player_health -= 1;
				}
			}
			for (boss, boss_trans) in (&mut bosses, &transforms).join() {
				if boss.shieldAmount == 0 && boss.active && rect_overlap(player_trans.translation[0], player_trans.translation[1], PLAYER_SIZE, boss_trans.translation[0], boss_trans.translation[1], BOSS_SIZE){
					boss.active = false;
					game_stats.player_health -= 1;
					println!("YOUR WON!!");
				}
			}
			for (shield, shield_trans) in (&mut boss_shields, &transforms).join() {
				if shield.active && rect_overlap(player_trans.translation[0], player_trans.translation[1], PLAYER_SIZE, shield_trans.translation[0], shield_trans.translation[1], BOSS_SHIELD_SIZE){
					shield.active = false;
					game_stats.player_health -= 1;
				}
			}
		}
    }
}
//Should make a libary
fn rect_overlap(x1: f32, y1: f32, size1: f32, x2: f32, y2: f32, size2: f32) -> bool {
	let l1x = x1;
	let l1y = y1;
	let r1x = x1+size1;
	let r1y = y1-size1;
	let l2x = x2;
	let l2y = y2;
	let r2x = x2+size2;
	let r2y = y2-size2;

    // If one rectangle is on left side of other 
    if (l1x > r2x || l2x > r1x) {
		return false;
	} 
  
    // If one rectangle is above other 
    if (l1y < r2y || l2y < r1y) {
		
		return false;
	} 
	
    return true; 
}