use super::metadata::{
	AssetMetadata,
	AssetTarget
};

use bevy::prelude::*;

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
			)
		);
	}
}

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

