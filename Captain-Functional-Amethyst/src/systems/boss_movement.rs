use amethyst::core::timing::Time;
use amethyst::core::transform::{Transform};
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::core::cgmath::{Vector2};
use std::f32::consts::PI;
use captain_functional::{Boss, Boss_Shield, ARENA_HEIGHT};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct BossMovement{
	circumference: f32,
}

const BOSSSPEED: f32 = 150.0;
const BOSSSHIELDSPEED: f32 = 100.0;
const MOVETIME: f32 = 3.0;
const RADIUS: f32 = 200.0;

impl<'s> System<'s> for BossMovement {
    type SystemData = (
		WriteStorage<'s, Boss>,
		WriteStorage<'s, Boss_Shield>,
        WriteStorage<'s, Transform>,
		Read<'s,  Time>,
    );

    fn run(&mut self, (mut bosses, mut boss_shields, mut transforms, time): Self::SystemData) {
		let mut xpos:f32 = 0.0;
		let mut ypos:f32 = -500.0;
		for (boss, transform) in (&mut bosses, &mut transforms).join() {
			if boss.active {
				boss.moveTime = boss.moveTime+time.delta_seconds();
				if MOVETIME > boss.moveTime {
					transform.translation[0] = transform.translation[0] - (BOSSSPEED*time.delta_seconds());
				}
				else {
					transform.translation[1] = transform.translation[1] + ((BOSSSPEED*time.delta_seconds())*boss.moveDir);
				}
				if 	(transform.translation[1] > ARENA_HEIGHT*0.8 && boss.moveDir == 1.0) ||
					(transform.translation[1] < ARENA_HEIGHT*0.2 && boss.moveDir == -1.0) {
					boss.moveDir = boss.moveDir*-1.0;
				}
				xpos = transform.translation[0];
				ypos = transform.translation[1];
			}
			else{
				transform.translation[0] = ARENA_HEIGHT*5.0;
				transform.translation[1] = ARENA_HEIGHT*3.0;
			}
		}
		for (shield, transform) in (&mut boss_shields, &mut transforms).join() {
			if shield.active{
				shield.progress += (PI*2.0)*(time.delta_seconds() * BOSSSHIELDSPEED) / self.circumference;
				transform.translation[0] = f32::cos(shield.progress)*RADIUS+xpos;
				transform.translation[1] = f32::sin(shield.progress)*RADIUS+ypos;
			}
			else{
				transform.translation[0] = ARENA_HEIGHT*5.0;
				transform.translation[1] = ARENA_HEIGHT*3.0;
			}
			
		}
    }
}

impl BossMovement {
	pub fn new() -> BossMovement {
		let _circumference: f32 = PI * 2.0 * RADIUS;
		BossMovement {
			circumference: _circumference,
		}
	}
}