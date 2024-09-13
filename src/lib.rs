#![feature(fn_traits)]

mod metadata;
mod ui;
mod world;
mod utils;
mod control;

use self::{
	metadata::AssetManagerPlugin,
	control::ControlPlugin,
	ui::UiPlugin,
	world::WorldPlugin,
};

use bevy::prelude::*;
use avian3d::debug_render::PhysicsDebugPlugin;

pub struct BasePlugin;

impl Plugin for BasePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			AssetManagerPlugin,
			ControlPlugin,
			UiPlugin,
			WorldPlugin,
		));
	}
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PhysicsDebugPlugin::default());
	}
}

