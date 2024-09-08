use super::metadata::{
	AssetMetadata,
	AssetTarget
};

use bevy::{
	input::{
		mouse::MouseMotion,
		touch::TouchPhase
	},
	prelude::*,
};

use bevy_egui::{
	egui,
	EguiContexts,
	EguiPlugin
};

use egui::widgets::{
	Button,
	Slider
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EguiPlugin)
			.insert_state(CharacterMenuTab::General)
			.insert_state(LoadMenuState::Open(AssetTarget::Body))
			.add_systems(Update, (
				load_loader_menu,
				load_edit_menu,
				free_camera_view
			));
	}
}

#[derive(Component)]
pub struct Cursor;

#[derive(States, Hash, Debug, Default, Eq, PartialEq, Clone)]
enum CharacterMenuTab {
	#[default]
	General,
	Head,
	Body,
	Clothes,
	Accessories,
}

#[derive(States, Hash, Debug, Eq, PartialEq, Clone, Default)]
enum LoadMenuState {
	#[default]
	Closed,
	Open(AssetTarget)
}

#[derive(Component)]
pub struct Selected;

fn load_edit_menu(
	mut contexts: EguiContexts,
	meshes: Res<Assets<Mesh>>,
	current_tab: Res<State<CharacterMenuTab>>,
	mut next_tab: ResMut<NextState<CharacterMenuTab>>,
	selected_query: Query<&Children, With<Selected>>,
	target_query: Query<(Entity, &AssetTarget)>,
	children_query: Query<&Children>,
	mut morph_query: Query<&mut MorphWeights>,
) {
	egui::Window::new("Edit Menu").show(contexts.ctx_mut(), |ui| {
		let style = ui.style_mut();
		style.spacing.slider_width = 200.;
		ui.columns(5, |columns| {
			let general_tab = columns[0].add(Button::new("General"));
			let head_tab = columns[1].add(Button::new("Head"));
			let body_tab = columns[2].add(Button::new("Body"));
			let clothes_tab = columns[3].add(Button::new("Clothes"));
			let accessories_tab = columns[4].add(Button::new("Accessories"));
			if general_tab.clicked() {
				next_tab.set(CharacterMenuTab::General);
			} else if head_tab.clicked() {
				next_tab.set(CharacterMenuTab::Head);
			} else if body_tab.clicked() {
				next_tab.set(CharacterMenuTab::Body);
			} else if clothes_tab.clicked() {
				next_tab.set(CharacterMenuTab::Clothes);
			} else if accessories_tab.clicked() {
				next_tab.set(CharacterMenuTab::Accessories);
			}
		});
		match current_tab.get() {
			CharacterMenuTab::General => {

			},
			CharacterMenuTab::Head => {

			},
			CharacterMenuTab::Body => {
				for selected_children in selected_query.iter() {
					for &selected_child in selected_children {
						if let Ok((entity, target)) = target_query.get(selected_child) {
							if *target != AssetTarget::Body {
								continue;
							}
							for child in children_query.iter_descendants(entity) {
								if let Ok(mut morph_weights) = morph_query.get_mut(child) {
									populate_morph_data(
										ui,
										&meshes,
										&mut morph_weights,
									);
									// break;
								}
							}
						}
					}
				}
			},
			CharacterMenuTab::Clothes => {

			},
			CharacterMenuTab::Accessories => {

			}
		}
	});
}

fn populate_morph_data(
	ui: &mut egui::Ui,
	meshes: &Assets<Mesh>,
	morph_weights: &mut MorphWeights,
) {
	let mut names: Option<&[String]> = None;
	if let Some(handle) = morph_weights.first_mesh() {
		if let Some(mesh) = meshes.get(handle) {
			names = mesh.morph_target_names();
		}
	}
	let weights = morph_weights.weights_mut();
	ui.columns(2, |columns| {
		for i in 0..weights.len() {
			if let Some(names) = names {
				columns[0].label(&names[i]);
			}
			columns[1].with_layout(columns[1].layout().with_cross_align(egui::Align::Max), |ui| {
				ui.add(Slider::new(&mut weights[i], 0.0..=1.0));
			});
		}
	});
}

fn load_loader_menu(
	mut contexts: EguiContexts,
	asset_server: Res<AssetServer>,
	selected_query: Query<&Children, With<Selected>>,
	mut asset_query: Query<(&mut Handle<Scene>, &AssetTarget)>,
	load_menu_state: Res<State<LoadMenuState>>,
	metadata: Res<AssetMetadata>,
) {
	match load_menu_state.get() {
		LoadMenuState::Closed => (),
		LoadMenuState::Open(load_target) => {
			egui::Window::new("Loader Menu").show(contexts.ctx_mut(), |ui| {
				for children in selected_query.iter() {
					for &child in children {
						if let Ok((scene, target)) = asset_query.get_mut(child) {
							if *load_target == *target {
								populate_asset_selection_menu(
									ui,
									&asset_server,
									&metadata,
									scene,
									target
								);
							}
						}
					}
				}
			});
		}
	}
}

fn populate_asset_selection_menu(
	ui: &mut egui::Ui,
	asset_server: &AssetServer,
	metadata: &AssetMetadata,
	mut scene: Mut<Handle<Scene>>,
	target: &AssetTarget
) {
	// ui.columns(2, |columns| {
		let scenes_metadata = metadata.scenes.iter().filter(|&x| x.target == *target);
		let mut new_scene = None;
		for scene_metadata in scenes_metadata {
			if ui.button(&scene_metadata.name).clicked() {
				new_scene = Some(scene_metadata);
			}
		}
		if let Some(scene_metadata) = new_scene {
			*scene.as_mut() = asset_server.load(format!("{}#Scene{}", scene_metadata.relative_path, scene_metadata.index));
		}
	// });
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn free_camera_view(
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Cursor>)>,
	mut cursor_query: Query<&mut Transform, (With<Cursor>, Without<Camera>)>,
	mut motion_reader: EventReader<MouseMotion>,
	mouse_button_input: Res<ButtonInput<MouseButton>>,
	time: Res<Time>,
	mut context: EguiContexts,
) {
	if context.ctx_mut().wants_pointer_input() {
		return;
	}
	if let Some(motion) = motion_reader.read().max_by(|&x, &y| x.delta.length().total_cmp(&y.delta.length())) {
		for input in mouse_button_input.get_pressed() {
			match input {
				/* Rotate */
				MouseButton::Left => {
					let rotation_x = Quat::from_rotation_x(-motion.delta.y * time.delta_seconds());
					let rotation_y = Quat::from_rotation_y(-motion.delta.x * time.delta_seconds());
					// let angle = motion.delta.length().to_radians();
					// let rotation_axis = Vec3::new(-motion.delta.y, -motion.delta.x, 0.).normalize();
					for (mut camera_transform, cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter()) {
						camera_transform.look_at(cursor_transform.translation, Vec3::Y);
						let distance = (camera_transform.translation - cursor_transform.translation).length();
						camera_transform.translation = cursor_transform.translation;
						camera_transform.rotation *= rotation_x;
						camera_transform.rotation *= rotation_y;
						let back = camera_transform.back() * distance;
						camera_transform.translation += back;
					}
					// for (mut camera_transform, cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter()) {
					// 	camera_transform.look_at(cursor_transform.translation, Vec3::Y);
					// 	let distance = (camera_transform.translation - cursor_transform.translation).length();
					// 	camera_transform.translation = cursor_transform.translation;
					// 	camera_transform.rotate_local_axis(rotation_axis, angle);
					// 	let forward = transform.forward();
					// 	transform.look_to(forward.xyz(), Vec3::Y);
					// 	let back = camera_transform.back() * distance;
					// 	camera_transform.translation += back;
					// }
				},
				/* Pan */
				MouseButton::Middle => {
					for (mut camera_transform, mut cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter_mut()) {
						let right = camera_transform.right() * motion.delta.x * time.delta_seconds();
						let down = camera_transform.down() * motion.delta.y * time.delta_seconds();
						camera_transform.translation += right;
						camera_transform.translation += down;
						cursor_transform.translation += right;
						cursor_transform.translation += down;
					}
				},
				/* Zoom + Translate Y */
				MouseButton::Right => {
					for (mut camera_transform, mut cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter_mut()) {
						let distance = (camera_transform.translation - cursor_transform.translation).length();
						let neg_y = Vec3::NEG_Y * motion.delta.y * time.delta_seconds();
						let forward = Vec3::ZERO.lerp(camera_transform.forward() * motion.delta.x * time.delta_seconds(), distance);
						camera_transform.translation += neg_y;
						camera_transform.translation += forward;
						cursor_transform.translation += neg_y;
					}
				},
				_ => ()
			}
		}
	}
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn free_camera_view(
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Cursor>)>,
	mut cursor_query: Query<&mut Transform, (With<Cursor>, Without<Camera>)>,
	mut touch_inputs: EventReader<TouchInput>,
	mut prev_inputs: Local<Vec<TouchInput>>,
	time: Res<Time>,

	mut context: EguiContexts,
) {
	if context.ctx_mut().wants_pointer_input() {
		return;
	}
	let inputs: Vec<TouchInput> = touch_inputs.read().cloned().collect();
	/* Rotate */
	if inputs.len() == 1 {
		let input = inputs[0];
		match input.phase {
			TouchPhase::Moved => {
				if let Some(prev_input) = prev_inputs.iter().find(|x| x.id == input.id) {
					let rotation_x = Quat::from_rotation_x(prev_input.position.y - input.position.y * time.delta_seconds());
					let rotation_y = Quat::from_rotation_y(prev_input.position.x - input.position.x * time.delta_seconds());

					for (mut camera_transform, cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter()) {
						camera_transform.look_at(cursor_transform.translation, Vec3::Y);
						let distance = (camera_transform.translation - cursor_transform.translation).length();
						camera_transform.translation = cursor_transform.translation;
						camera_transform.rotation *= rotation_x;
						camera_transform.rotation *= rotation_y;
						let back = camera_transform.back() * distance;
						camera_transform.translation += back;
					}
				}
			}
			_ => (),
		}
	} else if inputs.len() >= 2 {
		/* Pan */
		if let Some(first_prev_input) = prev_inputs.iter().find(|x| x.id == inputs[0].id) {
			if let Some(second_prev_input) = prev_inputs.iter().find(|x| x.id == inputs[1].id) {
				let first_delta = inputs[0].position - first_prev_input.position;
				let second_delta = inputs[1].position - second_prev_input.position;
				let delta = first_delta - second_delta;
				let pan = (first_delta + second_delta);
				let zoom = delta.length();
				for (mut camera_transform, mut cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter_mut()) {
					let right = camera_transform.right() * pan.x * time.delta_seconds();
					camera_transform.translation += right;
					cursor_transform.translation += right;

					let down = camera_transform.down() * pan.y * time.delta_seconds();
					camera_transform.translation += down;
					cursor_transform.translation += down;

					let distance = camera_transform.translation.distance(cursor_transform.translation);
					let forward = Vec3::ZERO.lerp(camera_transform.forward() * zoom * time.delta_seconds(), distance);
					camera_transform.translation += forward;
				}
			}
		}


	}
	*prev_inputs = inputs;
	//
	// 			/* Zoom + Translate Y */
	// 			MouseButton::Right => {
	// 				for (mut camera_transform, mut cursor_transform) in camera_query.iter_mut().zip(cursor_query.iter_mut()) {
	//
	// 					let neg_y = Vec3::NEG_Y * motion.delta.y * time.delta_seconds();
	//
	// 					camera_transform.translation += neg_y;
	//
	// 					cursor_transform.translation += neg_y;
	// 				}
	// 			},
	// 			_ => ()
	// 		}
	// 	}
	// }
}
