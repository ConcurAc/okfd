use materials::tiling::TiledMaterial;

use bevy::prelude::*;
use avian3d::prelude::*;

pub struct TilingPlugin;

impl Plugin for TilingPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(MaterialPlugin::<TiledMaterial>::default());
		// app.add_systems(Update, update_tiling);
	}

}

#[derive(Bundle, Default)]
pub struct EndlessPlane {
	pub mesh: Handle<Mesh>,
	pub material: Handle<TiledMaterial>,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
	/// User indication of whether an entity is visible
	pub visibility: Visibility,
	/// Inherited visibility of an entity.
	pub inherited_visibility: InheritedVisibility,
	/// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
	pub view_visibility: ViewVisibility,
	pub ridge_body: RigidBody,
	pub collider: Collider,
}

impl EndlessPlane {
	pub fn new(mut meshes: ResMut<Assets<Mesh>>, material: Handle<TiledMaterial>) -> Self {
		let plane = Plane3d::new(Vec3::Y, Vec2::new(10., 10.));
		Self {
			mesh: meshes.add(plane),
			material,
			ridge_body: RigidBody::Static,
			collider: Collider::half_space(Vec3::Y),
			..default()
		}
	}
}
