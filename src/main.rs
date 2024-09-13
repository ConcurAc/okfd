
use bevy::prelude::*;

use okfd::{
    BasePlugin,
    DebugPlugin
};

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    App::new().add_plugins((
        DefaultPlugins,
        avian3d::PhysicsPlugins::default(),
        BasePlugin,
        DebugPlugin
    )).run();
}

