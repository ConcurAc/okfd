use bevy::{
	prelude::*,
	input::ButtonInput,
	utils::HashMap,
};
use serde::{
	Serialize,
	Deserialize
};
use bevy_persistent::prelude::*;

use character::CharacterAction;

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct Bindings {
	pub keys: HashMap<KeyCode, CharacterAction>,
	pub mouse: HashMap<MouseButton, CharacterAction>
}

pub(super) fn process_input(
	key_input: Res<ButtonInput<KeyCode>>,
	mouse_input: Res<ButtonInput<MouseButton>>,
	bindings: Res<Persistent<Bindings>>,
	mut action_events: EventWriter<CharacterAction>
) {
	let mut actions: Vec<CharacterAction> = Vec::new();

	/* pressed */
	for key in key_input.get_pressed() {
		if let Some(&action) = bindings.get().keys.get(key) {
			match action {
				CharacterAction::Forward | CharacterAction::Backward => actions.push(action),
				CharacterAction::Left | CharacterAction::Right => actions.push(action),
				CharacterAction::Sprint => actions.push(action),
				_ => (),
			}
		}
	}
	for button in mouse_input.get_pressed() {
		if let Some(&action) = bindings.get().mouse.get(button) {
			match action {
				_ => (),
			}
		}
	}

	/* just pressed */
	for key in key_input.get_just_pressed() {
		if let Some(&action) = bindings.get().keys.get(key) {
			match action {
				CharacterAction::Jump => actions.push(action),
				_ => (),
			}
		}
	}
	for button in mouse_input.get_just_pressed() {
		if let Some(&action) = bindings.get().mouse.get(button) {
			match action {
				CharacterAction::Jump => actions.push(action),
				_ => (),
			}
		}
	}

	/* just released */
	for key in key_input.get_just_released() {
		if let Some(&action) = bindings.get().keys.get(key) {
			match action {
				_ => (),
			}
		}
	}
	for button in mouse_input.get_just_released() {
		if let Some(&action) = bindings.get().mouse.get(button) {
			match action {
				_ => (),
			}
		}
	}

	action_events.send_batch(actions);
}
