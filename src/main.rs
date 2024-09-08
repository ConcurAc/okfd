
use bevy::prelude::*;

use character_maker::{
    DebugPlugin,
    character::CharacterPlugin,
    environment::WorldPlugin,
    metadata::AssetManagerPlugin,
    ui::UiPlugin,
};


#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    App::new().add_plugins((
        DefaultPlugins,
        avian3d::PhysicsPlugins::default(),
        DebugPlugin,
        AssetManagerPlugin,
        UiPlugin,
        CharacterPlugin,
        WorldPlugin
    )).run();
}

