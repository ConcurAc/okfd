use bevy::{
	prelude::*,
	utils::HashMap
};

use serde::{
	Serialize,
	Deserialize
};

#[derive(Event, Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub enum CharacterAction {
	Idle,
	Forward,
	Backward,
	Left,
	Right,
	Sprint,
	Jump,
}

#[derive(Component, Default)]
pub struct CharacterActionAnimations(pub HashMap<CharacterAction, AnimationNodeIndex>);


pub fn play_action_animation(
	mut player: Mut<AnimationPlayer>,
	animated_actions: &CharacterActionAnimations,
	action: &CharacterAction,
) -> Option<()> {
	let animation = *animated_actions.0.get(action)?;
	if !player.is_playing_animation(animation) {
		match action {
			CharacterAction::Idle => {
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Forward);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Backward);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Left);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Right);
			},
			CharacterAction::Forward => {
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Backward);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Idle);
			},
			CharacterAction::Backward => {
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Forward);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Idle);
			},
			CharacterAction::Left => {
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Right);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Idle);
			},
			CharacterAction::Right => {
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Left);
				stop_action_animation(player.as_mut(), animated_actions, CharacterAction::Idle);
			},
			_ => ()
		}
		player.start(animation).repeat();
	}
	Some(())
}

fn stop_action_animation(player: &mut AnimationPlayer, animated_actions: &CharacterActionAnimations, action: CharacterAction) {
	if let Some(&animation) = animated_actions.0.get(&action) {
		player.stop(animation);
	}
}
