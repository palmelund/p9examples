use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};

pub const PLAYER_SIZE: f32 = 60.0;

pub const ARENA_HEIGHT: f32 = 800.0;
pub const ARENA_WIDTH: f32 = 1600.0;

pub struct Captain_Functional;

pub struct Player {
    pub width: f32,
    pub height: f32,
	pub lastShot: u128,
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

	// Create a left plank entity.
	world
		.create_entity()
		.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
		.expect("Failed to add sprite render on left paddle")
		.with(Player{
			height: PLAYER_SIZE,
			width: PLAYER_SIZE,
			lastShot: 0,
		})
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

		initialise_player(world, spritesheet);
		initialise_camera(world);
	}
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
