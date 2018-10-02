use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use captain_functional::{Player_Bullet, ARENA_WIDTH, Enemy, BULLET_SIZE, ENEMY_SIZE};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct BulletCollision;
const Bullet_Speed: f32 = 500.0;

impl<'s> System<'s> for BulletCollision {
    type SystemData = (
        WriteStorage<'s, Player_Bullet>,
		WriteStorage<'s, Enemy>,
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