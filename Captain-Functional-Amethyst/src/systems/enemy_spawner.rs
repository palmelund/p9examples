use amethyst::ecs::{System, WriteStorage, Read, Join};
use amethyst::core::Transform;
use amethyst::core::timing::Time;
use captain_functional::{Enemy, ARENA_HEIGHT, ARENA_WIDTH, Boss};

const spawnCount: i32 = 5;

pub struct SpawnEnemies {
    pub counter: f32,
	pub spawned: i32,
	pub bossSpawned: bool,
}

const spawnrate: f32 = 1.0;

impl<'s> System<'s> for SpawnEnemies {
    type SystemData = (
        WriteStorage<'s, Transform>,
		WriteStorage<'s, Enemy>,
		WriteStorage<'s, Boss>,
		Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut enemies, mut bosses, time): Self::SystemData) {
        self.counter += time.delta_seconds();
        if self.counter > spawnrate {

            self.counter = 0.0;
			for (enemy, transform) in (&mut enemies, &mut transforms).join(){
				if !enemy.active{
					self.spawned = self.spawned+1;
					transform.translation[0] = ARENA_WIDTH+50.0;
					let heightRange = (3.0/(time.delta_seconds())%0.8)+0.1;
					transform.translation[1] = ARENA_HEIGHT*heightRange;
					enemy.active = true;
					break;
				}	
			}
			if self.spawned > spawnCount && !self.bossSpawned {
				for (boss, transform) in (&mut bosses, &mut transforms).join(){
					boss.active = true;
					self.bossSpawned = true;
					transform.translation[0] = ARENA_WIDTH+50.0;
					transform.translation[1] = ARENA_HEIGHT*0.5;
					break;
				}
			}
        }
    }
}

