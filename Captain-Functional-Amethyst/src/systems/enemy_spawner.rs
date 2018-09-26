use amethyst::ecs::{System, WriteStorage, ReadStorage, Entities};
use amethyst::core::Transform;
use amethyst::core::timing::Time;
use captain_functional::{Enemy};

pub struct SpawnEnemies {
    pub counter: f32,
}

const spawnrate: f32 = 1.0;

impl<'s> System<'s> for SpawnEnemies {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
		ReadStorage<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut enemies, time, entities): Self::SystemData) {
        self.counter += time.delta_seconds();
        if self.counter > spawnrate {
            entities.build_entity()
                .with(Transform::default(), &mut transforms)
                .with(Enemy{height: 60.0,width: 60.0}, &mut enemies)
                .build();
            self.counter = 0.0;
        }
    }
}

