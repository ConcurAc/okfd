mod bindings;
mod camera;
mod actions;

use bindings::*;
pub use camera::*;
use actions::*;

use bevy::prelude::*;

use character::*;

use bevy_persistent::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(CharacterControllerPlugin)
			.insert_resource(persistent_bindings())
			.add_event::<CharacterAction>()
			.init_state::<CameraMode>()
			.add_systems(Update, (
				init_character_animation_player,
				process_input.before(process_actions),
				process_actions,
				camera_control,
			)
		);
	}
}

#[derive(Component)]
pub struct Controlling;

fn persistent_bindings() -> Persistent<Bindings> {
	Persistent::<Bindings>::builder()
		.name("bindings")
		.format(StorageFormat::Toml)
		.path("bindings.toml")
		.default({
			let mut bindings = Bindings::default();
			bindings.keys.insert(KeyCode::KeyW, CharacterAction::Forward);
			bindings.keys.insert(KeyCode::KeyS, CharacterAction::Backward);
			bindings.keys.insert(KeyCode::KeyA, CharacterAction::Left);
			bindings.keys.insert(KeyCode::KeyD, CharacterAction::Right);
			bindings.keys.insert(KeyCode::ArrowUp, CharacterAction::Forward);
			bindings.keys.insert(KeyCode::ArrowDown, CharacterAction::Backward);
			bindings.keys.insert(KeyCode::ArrowLeft, CharacterAction::Left);
			bindings.keys.insert(KeyCode::ArrowRight, CharacterAction::Right);
			bindings.keys.insert(KeyCode::ShiftLeft, CharacterAction::Sprint);
			bindings.keys.insert(KeyCode::Space, CharacterAction::Jump);

			// bindings.mouse.insert(MouseButton::Right, CharacterAction::Jump);
			bindings
		})
		.revertible(true)
		.build()
		.expect("failed to initialise key bindings")
}
