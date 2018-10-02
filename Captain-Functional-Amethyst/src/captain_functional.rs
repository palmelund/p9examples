use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Entity, Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiTransform};
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender, 
};

use std::f32::consts::PI;

pub const PLAYER_SIZE: f32 = 60.0;
pub const BULLET_SIZE: f32 = 30.0;
pub const ENEMY_SIZE: f32 = 50.0;

pub const BOSS_SIZE: f32 = 100.0;
pub const BOSS_SHIELD_SIZE: f32 = 80.0;

pub const ARENA_HEIGHT: f32 = 800.0;
pub const ARENA_WIDTH: f32 = 1600.0;

pub const ENEMY_SCORE_REWARD: i32 = 10;

const BULLET_POOL: i32 = 20;
const ENEMY_POOL: i32 = 20;

pub struct Captain_Functional;

pub struct Player {
    pub width: f32,
    pub height: f32,
}

pub struct Enemy {
    pub width: f32,
    pub height: f32,
	pub active: bool,
	pub fired: bool,
}

pub struct Player_Bullet {
	pub active: bool,
}

#[derive(Default)]
pub struct GameStats{
	pub score: i32,
	pub player_health: i32,
}

pub struct EnemyBullet {
	pub spawn: bool,
	pub spawn_x: f32,
	pub spawn_y: f32,
	pub active: bool,
}

pub struct Boss {
    pub width: f32,
    pub height: f32,
	pub moveTime: f32,
	pub moveDir: f32,
	pub shieldAmount: i32,
	pub active: bool,
}

pub struct Boss_Shield {
    pub width: f32,
    pub height: f32,
	pub progress: f32,
	pub active: bool,
}

pub struct UI_Entities {
    pub player_health: Entity,
    pub player_score: Entity,
}

impl Boss_Shield {
	pub fn new(position: f32) -> Boss_Shield {
		let _progress: f32 = (PI * 2.0) * ( position/3.0);
		Boss_Shield {
			width: BOSS_SHIELD_SIZE,
			height: BOSS_SHIELD_SIZE,
			progress: _progress,
			active, false,
		}
	}
}

impl GameStats {
	pub fn new() -> GameStats {
		GameStats {
			score: 0,
			player_health: 3,
		}
	}
}

fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}

fn initialise_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "/font/CREABBB.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        -50.,
        50.,
        1.,
        55.,
        50.,
        0,
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        50.,
        50.,
        1.,
        55.,
        50.,
        0,
    );

    /*let player_health = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "Health: 3".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.,
        )).build();
    let player_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font,
            "Score: 0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.,
        )).build();
    world.add_resource(UI_Entities { player_health, player_score });*/
}

fn initialise_enemys(world: &mut World, spritesheet: TextureHandle) {
    for x in 0..ENEMY_POOL {
		let mut left_transform = Transform::default();

		// Correctly position the paddles.
		let y = ARENA_HEIGHT + 50.0;
		left_transform.translation = Vector3::new(ARENA_WIDTH * 0.9, y, 0.0);

		// Build the sprite for the paddles.
		let sprite = Sprite {
			left: 0.0,
			right: ENEMY_SIZE,
			top: 0.0,
			bottom: ENEMY_SIZE,
		};

		const SPRITESHEET_SIZE: (f32, f32) = (ENEMY_SIZE, ENEMY_SIZE);

		let enemy = Enemy{
				height: ENEMY_SIZE,
				width: ENEMY_SIZE,
				active: false,
				fired: false,
			};
		// Create a left plank entity.
		world
			.create_entity()
			.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
			.expect("Failed to add sprite to enemy")
			.with(enemy)
			.with(GlobalTransform::default())
			.with(left_transform)
			.build();
	}
}

fn initialise_player_bullets(world: &mut World, spritesheet: TextureHandle) {
	for x in 0..BULLET_POOL {
		let mut left_transform = Transform::default();

		// Correctly position the paddles.
		let y = ARENA_HEIGHT + 50.0;
		left_transform.translation = Vector3::new(ARENA_WIDTH * 0.5, y, 0.0);

		// Build the sprite for the paddles.
		let sprite = Sprite {
			left: 0.0,
			right: BULLET_SIZE,
			top: 0.0,
			bottom: BULLET_SIZE,
		};

		const SPRITESHEET_SIZE: (f32, f32) = (BULLET_SIZE, BULLET_SIZE);

		let bullet = Player_Bullet{
			active: false,
		};
		// Create a left plank entity.
		world
			.create_entity()
			.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
			.expect("Failed to add sprite to bullet")
			.with(bullet)
			.with(GlobalTransform::default())
			.with(left_transform)
			.build();
	}
}

fn initialise_enemy_bullets(world: &mut World, spritesheet: TextureHandle) {
	for x in 0..BULLET_POOL {
		let mut left_transform = Transform::default();

		// Correctly position the paddles.
		let y = ARENA_HEIGHT + 50.0;
		left_transform.translation = Vector3::new(ARENA_WIDTH * 0.5, y, 0.0);

		// Build the sprite for the paddles.
		let sprite = Sprite {
			left: 0.0,
			right: BULLET_SIZE,
			top: 0.0,
			bottom: BULLET_SIZE,
		};

		const SPRITESHEET_SIZE: (f32, f32) = (BULLET_SIZE, BULLET_SIZE);

		let bullet = EnemyBullet{
			spawn: false,
			spawn_x: 0.0,
			spawn_y: 0.0,
			active: false,
		};
		// Create a left plank entity.
		world
			.create_entity()
			.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
			.expect("Failed to add sprite to bullet")
			.with(bullet)
			.with(GlobalTransform::default())
			.with(left_transform)
			.build();
	}
}

fn initialise_boss(world: &mut World, spritesheet: TextureHandle, spriteShield: [TextureHandle;3]) {
    let mut left_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT * 4.0;
    left_transform.translation = Vector3::new(ARENA_WIDTH*1.5, y, 0.0);

	// Build the sprite for the paddles.
	let sprite_boss = Sprite {
		left: 0.0,
		right: BOSS_SIZE,
		top: 0.0,
		bottom: BOSS_SIZE,
	};

	let sprite_shield = Sprite {
		left: 0.0,
		right: BOSS_SHIELD_SIZE,
		top: 0.0,
		bottom: BOSS_SHIELD_SIZE,
	};

	const SPRITEBOSS_SIZE: (f32, f32) = (BOSS_SIZE, BOSS_SIZE);
	const SPRITEBSHIELD_SIZE: (f32, f32) = (BOSS_SHIELD_SIZE, BOSS_SHIELD_SIZE);

	let boss = Boss{
		height: BOSS_SIZE,
		moveTime: 0.0,
		moveDir: 1.0,
		width: BOSS_SIZE,
		shieldAmount: 3,
		active: false,
	};


	let boss_shield2 = Boss_Shield::new(2.0);
	let boss_shield3 = Boss_Shield::new(3.0);

	world
		.create_entity()
		.with_sprite(&sprite_boss, spritesheet.clone(), SPRITEBOSS_SIZE)
		.expect("Failed to add sprite render to boss")
		.with(boss)
		.with(GlobalTransform::default())
		.with(left_transform.clone())
		.build();

	// Create a left plank entity.
	for x in 1..4 {
		let boss_shield = Boss_Shield::new(x as f32);
		world
			.create_entity()
			.with_sprite(&sprite_shield, spriteShield[x-1].clone(), SPRITEBSHIELD_SIZE)
			.expect("Failed to add sprite render to boss")
			.with(boss_shield)
			.with(GlobalTransform::default())
			.with(left_transform.clone())
			.build();
	}
}

fn initialise_player(world: &mut World, spritesheet: TextureHandle) {
    let mut left_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.translation = Vector3::new(PLAYER_SIZE * 0.5, y, 0.0);

	// Build the sprite for the paddles.
	let sprite = Sprite {
		left: 0.0,
		right: PLAYER_SIZE,
		top: 0.0,
		bottom: PLAYER_SIZE,
	};

    const SPRITESHEET_SIZE: (f32, f32) = (PLAYER_SIZE, PLAYER_SIZE);

	let player = Player{
			height: PLAYER_SIZE,
			width: PLAYER_SIZE,
		};
	// Create a left plank entity.
	world
		.create_entity()
		.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
		.expect("Failed to add sprite render to player")
		.with(player)
		.with(GlobalTransform::default())
		.with(left_transform)
		.build();
}




impl<'a, 'b> State<GameData<'a, 'b>> for Captain_Functional {
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }

	fn on_start(&mut self, data: StateData<GameData>) {
		let world = data.world;
		world.register::<Player>();
		world.register::<Enemy>();
		world.register::<Player_Bullet>();
		world.register::<Boss>();
		world.register::<Boss_Shield>();
		world.register::<EnemyBullet>();

		// Load the spritesheet necessary to render the graphics.
		let spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/rust_icon_01_scale.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		let bullet_spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/player_bullet.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		let enemy_spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/cpp_logo.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		let boss_spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/Javascript-736400_960_720.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		let shield1_spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/Vue.js_Logo.svg.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		let shield2_spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/500px-React-icon.svg.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		let shield3_spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"textures/Angular_full_color_logo.svg.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};


		initialise_enemys(world, enemy_spritesheet);
		initialise_boss(world,boss_spritesheet,[shield1_spritesheet,shield2_spritesheet,shield3_spritesheet]);
		initialise_player_bullets(world, bullet_spritesheet.clone());
		initialise_enemy_bullets(world, bullet_spritesheet);
		initialise_player(world, spritesheet);
		initialise_camera(world);
		//Cant load fonts for some reason?
		//initialise_score(world);
	}
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
impl Component for Player_Bullet {
    type Storage = DenseVecStorage<Self>;
}
impl Component for Enemy {
	type Storage = DenseVecStorage<Self>;
}
impl Component for EnemyBullet {
	type Storage = DenseVecStorage<Self>;
}
impl Component for Boss {
	type Storage = DenseVecStorage<Self>;
}
impl Component for Boss_Shield {
	type Storage = DenseVecStorage<Self>;
}

