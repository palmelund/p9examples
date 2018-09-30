use amethyst::ecs::{System, WriteStorage, ReadStorage, Read, Join};
use amethyst::core::Transform;
use amethyst::core::timing::Time;
use captain_functional::{Enemy, ARENA_HEIGHT, ARENA_WIDTH};

pub struct SpawnEnemies {
    pub counter: f32,
	pub spawned: i32,
}

const spawnrate: f32 = 1.0;

impl<'s> System<'s> for SpawnEnemies {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
		Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut enemies, time): Self::SystemData) {
        self.counter += time.delta_seconds();
        if self.counter > spawnrate {

            self.counter = 0.0;
			for (enemy, transform) in (&mut enemies, &mut transforms).join(){
				if !enemy.active{
					self.spawned++;
					transform.translation[0] = ARENA_WIDTH+50.0;
					let heightRange = (3.0/(time.delta_seconds())%0.8)+0.1;
					transform.translation[1] = ARENA_HEIGHT*heightRange;
					enemy.active = true;
					break;
				}	
			}
        }
    }
}

