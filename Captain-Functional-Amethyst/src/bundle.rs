use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;
use systems::{PlayerSystem, SpawnEnemies, EnemySystem, BulletMovement, BulletCollision, BossMovement, Enemy_Shot,Enemy_Bullet_Movement, Player_Collision};
/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct CaptainBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CaptainBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
		builder.add(PlayerSystem{counter: 0.0}, "playersystem", &["input_system"]);
		builder.add(SpawnEnemies{counter: 0.0, spawned: 0, bossSpawned: false}, "enemyspawn", &[]);
		builder.add(EnemySystem, "enemysystem", &[]);
		builder.add(BulletMovement, "bullet_movement", &[]);
		builder.add(BulletCollision, "BulletCollision", &[]);
		builder.add(BossMovement::new(), "boss_movement", &[]);
		builder.add(Enemy_Shot, "enemy_shot", &[]);
		builder.add(Enemy_Bullet_Movement, "enemy_bullet_movement", &[]);
		builder.add(Player_Collision, "player_collision", &[]);
        Ok(())
    }
}