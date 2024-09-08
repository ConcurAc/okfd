
use super::metadata::AssetMetadata;

use bevy::prelude::*;
use avian3d::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (
				play_action,
				load_animations_to_players,
				generate_rig_colliders
			));
	}
}

// #[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Component)]
pub struct Animations(pub Vec<AnimationNodeIndex>);

 fn play_action(mut player_query: Query<(&mut AnimationPlayer, &Animations), Added<Animations>>) {
	for (mut player, animations) in player_query.iter_mut() {
		player.play(animations.0[0]).repeat();
	}
}

fn load_animations_to_players(
	mut commands: Commands,
	mut graphs: ResMut<Assets<AnimationGraph>>,
	asset_server: Res<AssetServer>,
	mut player_query: Query<(Entity, &Name), Added<AnimationPlayer>>,
	metadata: Res<AssetMetadata>
) {
	for (entity, name) in player_query.iter_mut() {
		let mut graph = AnimationGraph::new();
		let animations = graph.add_clips(metadata.animations.iter().filter(|x| x.name.contains(name.as_str())).map(|x| {
			asset_server.load(format!("{}#Animation{}", x.relative_path, x.index))
		}), 1.0, graph.root).collect();
		let graph = graphs.add(graph);
		commands.entity(entity).insert((
			graph.clone(),
			AnimationTransitions::new(),
			Animations(animations)
		));
	}
}
/*
fn generate_rig_ik(
	mut commands: Commands,
	name_query: Query<&Name>,
	parent_query: Query<&Parent>,
	children_query: Query<&Children>,
	player_query: Query<Entity, Added<AnimationPlayer>>,
) {
	for entity in player_query.iter() {
		for child in children_query.iter_descendants(entity) {
			if let Ok(name) = name_query.get(child) {
				match name.as_str() {
					"left_hand" => {
						let mut chain_length = 0;
						if let Some((target, pole_target)) = get_ik_targets(
							child,
							&name_query,
							&parent_query,
							&children_query,
							&mut chain_length,
							"chest",
							"left_hand_controller",
							"left_elbow_pole"
						) {
							commands.entity(target).insert(RigidBody::Dynamic);
							commands.entity(child).insert(
								IkConstraint {
									chain_length,
									 iterations: 50,
									 target,
									 pole_target,
									 pole_angle: -90.0f32.to_radians(),
									 enabled: true
								});
						}
					},
					"right_hand" => {
						let mut chain_length = 0;
						if let Some((target, pole_target)) = get_ik_targets(
							child,
							&name_query,
							&parent_query,
							&children_query,
							&mut chain_length,
							"chest",
							"right_hand_controller",
							"right_elbow_pole"
						) {
							commands.entity(target).insert(RigidBody::Dynamic);
							commands.entity(child).insert(
								IkConstraint {
									chain_length,
									 iterations: 5,
									 target,
									 pole_target,
									 pole_angle: -120.0f32.to_radians(),
									 enabled: true
								});
						}
					},
					"left_foot" => {
						let mut chain_length = 0;
						if let Some((target, pole_target)) = get_ik_targets(
							child,
							&name_query,
							&parent_query,
							&children_query,
							&mut chain_length,
							"pelvis",
							"left_foot_controller",
							"left_knee_pole"
						) {
							commands.entity(target).insert(RigidBody::Dynamic);
							commands.entity(child).insert(
								IkConstraint {
									chain_length,
									 iterations: 5,
									 target,
									 pole_target,
									 pole_angle: 90.0f32.to_radians(),
									 enabled: true
								});
						}
					},
					"right_foot" => {
						let mut chain_length = 0;
						if let Some((target, pole_target)) = get_ik_targets(
							child,
							&name_query,
							&parent_query,
							&children_query,
							&mut chain_length,
							"pelvis",
							"right_foot_controller",
							"right_knee_pole"
						) {
							commands.entity(target).insert(RigidBody::Dynamic);
							commands.entity(child).insert(
								IkConstraint {
									chain_length,
									iterations: 5,
									target,
									pole_target,
									pole_angle: 90.0f32.to_radians(),
									enabled: true
							});
						}
					},
					_ => ()
				}
			}
		}
	}
}*/

// fn get_ik_targets(
// 	entity: Entity,
// 	name_query: &Query<&Name>,
// 	parent_query: &Query<&Parent>,
// 	children_query: &Query<&Children>,
// 	chain_length: &mut usize,
// 	break_target_name: &str,
// 	target_name: &str,
// 	pole_target_name: &str
// ) -> Option<(Entity, Option<Entity>)> {
// 	let parent = parent_query.get(entity).ok()?.get();
// 	if let Ok(name) = name_query.get(parent) {
// 		if name.as_str() == break_target_name {
// 			let mut target = None;
// 			let mut pole_target = None;
// 			for child in children_query.iter_descendants(parent) {
// 				let child_name = name_query.get(child).ok()?;
// 				if child_name.as_str() == target_name {
// 					target = Some(child);
// 				}
// 				else if child_name.as_str() == pole_target_name {
// 					pole_target = Some(child);
// 				}
// 				if target.is_some() && pole_target.is_some() {
// 					break;
// 				}
// 			}
// 			return Some((target?, pole_target))
// 		}
// 	}
// 	*chain_length += 1;
// 	get_ik_targets(parent, name_query, parent_query, children_query, chain_length, break_target_name, target_name, pole_target_name)
// }

fn generate_rig_colliders(
	mut commands: Commands,
	name_query: Query<&Name>,
	children_query: Query<&Children>,
	player_query: Query<Entity, Added<AnimationPlayer>>,
) {
	for entity in player_query.iter() {
		for child in children_query.iter_descendants(entity) {
			if let Ok(name) = name_query.get(child) {
				if let Some(collider) = match_collider_by_name(&name) {
					commands.entity(child).insert(collider);
				}
			}
		}
	}
}

fn match_collider_by_name(name: &str) -> Option<Collider> {
	match name {
		"head" => {
			let length = 0.04;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), Collider::capsule(0.08, length)),
			]);
			Some(collider)
		},
		"neck" => {
			let length = 0.07;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), Collider::cylinder(0.04, length)),
			]);
			Some(collider)
		},
		"chest" => {
			let length = 0.1;
			let mut capsule = Collider::capsule(0.12, length);
			capsule.set_scale(Vec3::new(0.75, 0.6, 1.), 8);
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), capsule),
			]);
			Some(collider)
		},
		"waist" => {
			let length = 0.03;
			let mut capsule = Collider::cylinder(0.1, length);
			capsule.set_scale(Vec3::new(0.75, 1., 1.), 8);
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, 2.*length * 0.5, 0.0), Quat::default(), capsule),
			]);
			Some(collider)
		},
		"pelvis" => {
			let length = 0.1;
			let mut capsule = Collider::cylinder(0.13, length);
			capsule.set_scale(Vec3::new(0.6, 0.6, 1.), 8);
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), capsule),
			]);
			Some(collider)
		},
		"left_breast" | "right_breast" => {
			let size = 0.04;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, 0.0, 0.0), Quat::default(), Collider::sphere(size)),
			]);
			Some(collider)
		}
		"left_shoulder" | "right_shoulder" => {
			let length = 0.175;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), Collider::capsule(0.03, length)),
			]);
			Some(collider)
		}
		"left_elbow" | "right_elbow" => {
			let length = 0.2;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), Collider::capsule(0.03, length)),
			]);
			Some(collider)
		},
		"left_hip" | "right_hip" => {
			let length = 0.25;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), Collider::capsule(0.05, length)),
			]);
			Some(collider)
		},
		"left_knee" | "right_knee" => {
			let length = 0.3;
			let collider = Collider::compound(vec![
				(Vec3::new(0.0, length * 0.5, 0.0), Quat::default(), Collider::capsule(0.04, length)),
			]);
			Some(collider)
		},
		_ => None
	}
}
