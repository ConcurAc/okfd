use super::Controlling;

use bevy::prelude::*;
use avian3d::prelude::*;

use character::*;

pub(super) fn init_character_animation_player(
	mut commands: Commands,
	mut graphs: ResMut<Assets<AnimationGraph>>,
	player_query: Query<Entity, Added<AnimationPlayer>>,
	parent_query: Query<&Parent, With<CharacterController>>,
	action_clips_query: Query<&CharacterActionClips, Added<CharacterActionClips>>
) {
	for entity in player_query.iter() {
		if let Some(parent) = parent_query.iter_ancestors(entity).last() {
			let mut ancestor_commands = commands.entity(parent);
			ancestor_commands.insert(ChildAnimationPlayer(entity));
			if let Ok(action_clips) = action_clips_query.get(parent) {
				let mut action_animations = CharacterActionAnimations::default();
				let mut graph = AnimationGraph::new();
				for (action, clip) in action_clips.0.iter() {
					let index = graph.add_clip(clip.clone(), 1., graph.root);
					action_animations.0.insert(*action, index);
				}
				ancestor_commands.remove::<CharacterActionClips>();

				let graph_handle = graphs.add(graph);
				commands.entity(entity).insert((
					graph_handle,
					AnimationTransitions::new(),
					action_animations
				));

			}
		}
	}
}

pub(super) fn process_actions(
	mut action_events: EventReader<CharacterAction>,
	mut controlling_query: Query<(&mut CharacterController, &ChildAnimationPlayer, &mut ExternalImpulse, &mut LinearVelocity), With<Controlling>>,
	camera_query: Query<&Transform, With<Camera>>,
	mut animation_query: Query<(&mut AnimationPlayer, &CharacterActionAnimations)>,
	time: Res<Time>,
) {
	if action_events.len() == 0 {
		for (mut controller, &player, _, mut velocity) in controlling_query.iter_mut() {
			controller.movement.reset_velocity();
			match controller.state {
				CharacterState::Able =>	controller.movement.stop_horizontal(&mut (*velocity), time.delta_seconds()),
				_ => (),
			}
			if let Ok((player, animated_actions)) = animation_query.get_mut(*player) {
				play_action_animation(player, animated_actions, &CharacterAction::Idle);
			}
		}
	}
	for ((mut controller, &player, mut impulse, mut velocity), camera_transform) in controlling_query.iter_mut().zip(camera_query.iter()) {
		let mut direction = Vec3::ZERO;
		let mut sprint = false;
		for action in action_events.read() {
			if let Ok((player, animated_actions)) = animation_query.get_mut(*player) {
				play_action_animation(player, animated_actions, action);
			}
			match action {
				CharacterAction::Forward => direction += camera_transform.forward().as_vec3(),
				CharacterAction::Backward => direction += camera_transform.back().as_vec3(),
				CharacterAction::Left => direction += camera_transform.left().as_vec3(),
				CharacterAction::Right => direction += camera_transform.right().as_vec3(),
				CharacterAction::Sprint => {
					match controller.state {
						CharacterState::Able => sprint = true,
						_ => ()
					}
				},
				CharacterAction::Jump => {
					match controller.state {
						CharacterState::Able => {
							controller.state = CharacterState::AirBorne;
							controller.floating.enable = false;
							impulse.persistent = false;
							impulse.apply_impulse(Vec3::Y);
						},
						_ => ()
					}
				}
				_ => (),
			}
		}
		impulse.persistent = false;
		if sprint {
			impulse.apply_impulse(controller.movement.update_sprint_impulse_horizontal(**velocity, direction));
		} else {
			impulse.apply_impulse(controller.movement.update_run_impulse_horizontal(**velocity, direction));
		}

		if direction == Vec3::ZERO {
			match controller.state {
				CharacterState::Able =>	controller.movement.stop_horizontal(&mut (*velocity), time.delta_seconds()),
				_ => (),
			}
			if let Ok((player, animated_actions)) = animation_query.get_mut(*player) {
				play_action_animation(player, animated_actions, &CharacterAction::Idle);
			}
		}
	}
}
