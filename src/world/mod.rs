mod characters;

use super::{
	ui::Selected,
	control::{
		Controlling,
		CameraMode
	},
};

use level_builder::{
	planes,
	rooms
};

use character::{
	CharacterPlugin,
	CharacterBundle,
};

use characters::DebugCharacter;

use materials::tiling::TiledMaterial;

use bevy::prelude::*;

use world::*;

const DEFAULT_CAMERA_TRANSFORM: Transform = Transform::from_xyz(0., 1., 2.);
const DEFAULT_CURSOR_TRANSFORM: Transform = Transform::from_xyz(0., 1., 0.);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			planes::TilingPlugin,
			CharacterPlugin::<DebugCharacter>::default()
		))
			.add_systems(Startup, (
				setup_environment,
			))
			.add_systems(Startup, spawn_character);
	}
}

fn setup_environment(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<TiledMaterial>>
) {
	let mut grass_material = TiledMaterial::default();
	grass_material.with_image(asset_server.load("maps/seamless_grass2.png"))
		.with_scale(10.)
		.with_rotation(f32::to_radians(270.))
		.with_noise(0.05);
	commands.spawn(planes::EndlessPlane::new(meshes, materials.add(grass_material)));

// 	commands.spawn((
// 		PbrBundle {
// 			mesh: meshes.add(Cylinder::new(5., 5.)),
// 			transform: Transform::from_xyz(0., -2.5, 0.),
// 			..default()
// 		},
// 		RigidBody::Static,
// 		Cylinder::new(5., 5.).collider()
// 	));
//
	// commands.insert_resource(AmbientLight{
	// 	color: Color::WHITE,
	// 	brightness: 500.
	// });
	// commands.spawn(PointLightBundle {
	// 	point_light: PointLight {
	// 		intensity: 5000.,
	// 		// range: 10.,
	// 		..default()
	// 	},
	// 	transform: Transform::from_xyz(3., 2., -3.),
	// 	..Default::default()
	// });
}


fn spawn_character(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	commands.spawn(Camera3dBundle{
		transform: DEFAULT_CAMERA_TRANSFORM.looking_at(DEFAULT_CURSOR_TRANSFORM.translation, Vec3::Y),
		..Default::default()
	});
	let target = commands.spawn((
		PbrBundle {
			transform: DEFAULT_CURSOR_TRANSFORM,
			mesh: meshes.add(Sphere::new(0.1)),
			material: materials.add(Color::srgba(0.3, 0.0, 0.3, 0.7)),
			..default()
		},
	)).id();
	commands.spawn((
		CharacterBundle::new(DebugCharacter {
			height: 1.5,
		}),
		SpatialBundle::from_transform(Transform::from_xyz(0., 10., 0.)),
		Controlling,
		Selected
	));
		// parent.spawn(CharacterPartBundle::head(asset_server.load("character/head/base.gltf#Scene0")));
	commands.insert_resource(State::new(CameraMode::FreeFollowing(target)));
}
