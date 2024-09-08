#![feature(fn_traits)]

pub mod metadata;
pub mod ui;
pub mod character;
pub mod environment;
pub mod utils;

use metadata::AssetManagerPlugin;
use ui::UiPlugin;
use character::CharacterPlugin;
use environment::WorldPlugin;


use bevy::{
	prelude::*,
	window::WindowMode
	// ecs::schedule::Stepping
};
use avian3d::debug_render::PhysicsDebugPlugin;

// #[cfg(not(any(target_os = "android", target_os = "ios")))]
// fn main() {
// 	App::new().add_plugins((
// 		DefaultPlugins,
// 		DebugPlugin,
// 		AssetManagerPlugin,
// 		UiPlugin,
// 		CharacterPlugin,
// 		WorldPlugin
// 	)).run();
// }


pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PhysicsDebugPlugin::default());
	}
}

