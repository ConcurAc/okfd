mod actions;
mod character_controller;

#[allow(unused_imports)]
pub use character_controller::{
	CharacterControllerPlugin,
	CharacterController,
	CharacterState
};

pub use actions::*;

use std::marker::PhantomData;

use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Default)]
pub struct CharacterPlugin<C: 'static + Sync + Send> {
	_marker: PhantomData<C>
}

impl<C: 'static + Sync + Send + Component + Character> Plugin for CharacterPlugin<C> {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (
				load_character::<C>,
			)
		);
	}
}

fn load_character<C>(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	character_query: Query<Entity, Added<C>>
) where C: Component + Character {
	for entity in character_query.iter() {
		commands.entity(entity).insert((
			C::scene(&asset_server),
			C::animations(&asset_server),
			C::collider_constructor_hierarchy()
		));
	}
}

#[derive(Bundle)]
pub struct CharacterBundle<C: 'static + Sync + Send + Component + Character> {
	pub character: C,
	controller: CharacterController,
	mass_properties_bundle: MassPropertiesBundle,
	pub rigid_body: RigidBody,
	pub locked_axes: LockedAxes,
}

impl<C: 'static + Sync + Send + Component + Character> CharacterBundle<C> {
	pub fn new(character: C) -> Self {
		Self {
			character,
			controller: CharacterController::new(Mass::default()),
			mass_properties_bundle: MassPropertiesBundle::default(),
			rigid_body: RigidBody::Dynamic,
			locked_axes: LockedAxes::ROTATION_LOCKED,
		}
	}
}

pub trait Character {
	const FILE: &'static str;
	fn scene(asset_server: &AssetServer) -> Handle<Scene>;
	fn animations(asset_server: &AssetServer) -> CharacterActionClips;
	fn collider_constructor_hierarchy() -> ColliderConstructorHierarchy;
	fn mass_properties(&self) -> MassPropertiesBundle;
}

#[derive(Component, Deref, Copy, Clone)]
pub struct ChildAnimationPlayer(pub Entity);

#[derive(Component, Default, Deref)]
pub struct CharacterActionClips(pub Vec<(CharacterAction, Handle<AnimationClip>)>);

