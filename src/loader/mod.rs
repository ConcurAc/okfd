use bevy::{
	prelude::*,
	utils::HashMap
};

use crate::metadata::AssetMetadata;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(Update, (
		// 	load_animations_to_players,
		// ));
	}
}
/*
#[derive(Component)]
pub struct Animations(pub HashMap<String, AnimationNodeIndex>);

fn load_animations_to_players(
	mut commands: Commands,
	mut graphs: ResMut<Assets<AnimationGraph>>,
	asset_server: Res<AssetServer>,
	mut player_query: Query<(Entity, &Name), Added<AnimationPlayer>>,
	metadata: Res<AssetMetadata>
) {
	for (entity, name) in player_query.iter_mut() {
		let mut graph = AnimationGraph::new();
		let animation_handles = metadata.animations.iter().filter(|x| x.name.contains(name.as_str())).map(|x| {
			asset_server.load(format!("{}#Animation{}", x.relative_path, x.index))
		});
		let indexes = graph.add_clips(animation_handles, 1.0, graph.root);
		let animations =
		let graph_handle = graphs.add(graph);
		commands.entity(entity).insert((
			graph_handle,
			AnimationTransitions::new(),
			Animations(animations)
		));
	}
}*/
