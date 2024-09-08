use super::{
	metadata::AssetTarget,
	ui::{
		Cursor,
		Selected,
	},
};

use bevy::prelude::*;

use avian3d::prelude::*;

const DEFAULT_CAMERA_TRANSFORM: Transform = Transform::from_xyz(0., 1., 2.);
const DEFAULT_CURSOR_TRANSFORM: Transform = Transform::from_xyz(0., 1., 0.);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, (
					setup_camera,
					setup_environment,

			))
			.add_systems(Startup, spawn_character);
	}
}

fn setup_camera(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	commands.spawn(Camera3dBundle{
		transform: DEFAULT_CAMERA_TRANSFORM.looking_at(DEFAULT_CURSOR_TRANSFORM.translation, Vec3::Y),
		..Default::default()
	});
	commands.spawn((
		PbrBundle {
			transform: DEFAULT_CURSOR_TRANSFORM,
			mesh: meshes.add(Sphere::new(0.1)),
			material: materials.add(Color::srgba(0.3, 0.0, 0.3, 0.7)),
			..default()
		},
		Cursor
	));
}

fn setup_environment(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
	// let map: Handle<Scene> = asset_server.load("maps/main_world.glb#Scene0");
	// commands.spawn(SceneBundle{
	// 	scene: map,
	// 	..default()
	// });
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Cylinder::new(5., 5.)),
			transform: Transform::from_xyz(0., -2.5, 0.),
			..default()
		},
		RigidBody::Static,
		Cylinder::new(5., 5.).collider()
	));
	// commands.insert_resource(AmbientLight{
	// 	color: Color::WHITE,
	// 	brightness: 500.
	// });
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 100000.,
			// range: 10.,
			..default()
		},
		transform: Transform::from_xyz(3., 2., -3.),
		..Default::default()
	});
}


fn spawn_character(mut commands: Commands) {
	commands.spawn((
		TransformBundle::default(),
		VisibilityBundle::default(),
		// RigidBody::Dynamic,
		Selected
	)).with_children(|parent| {
		// parent.spawn((
		// 	SceneBundle::default(),
		// 	AssetTarget::Head
		// ));

		parent.spawn((
			SceneBundle::default(),
			AssetTarget::Body
		));
	});
}
