
use std::fs::{
	DirEntry,
	read_dir,
	read
};

use bevy::prelude::*;

use gltf::{
	Document,
	json::Root
};

const ASSET_DIR: &str = "assets/";

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, document_available_assets);
	}
}

#[derive(Resource, Default)]
pub struct AssetMetadata {
	pub scenes: Vec<SceneMetadata>,
	pub animations: Vec<AnimationMetadata>
}

pub struct SceneMetadata {
	pub index: usize,
	pub name: String,
	pub relative_path: String,
	pub target: AssetTarget
}

pub struct AnimationMetadata {
	pub index: usize,
	pub name: String,
	pub relative_path: String
}

fn document_available_assets(mut commands: Commands) {
	let mut metadata = AssetMetadata::default();
	metadata.document_scenes(format!("{}character/head", ASSET_DIR), AssetTarget::Head);
	metadata.document_scenes(format!("{}character/body", ASSET_DIR), AssetTarget::Body);
	metadata.document_animations(format!("{}character", ASSET_DIR));
	commands.insert_resource(metadata);
}

impl AssetMetadata {
	fn document_scenes(&mut self, relative_root: String, target: AssetTarget) {
		let mut dir_queue = Vec::new();
		dir_queue.push(relative_root.to_owned());
		while !dir_queue.is_empty() {
			for dir in dir_queue.clone().iter() {
				if let Ok(dir) = read_dir(dir) {
					for dir_entry in dir {
						self.document_dir_entry_scenes(&mut dir_queue, &relative_root, target, dir_entry.ok());
					}
				}
				dir_queue.sort_unstable();
				dir_queue.remove(dir_queue.binary_search(dir).unwrap());
			}
		}
	}
	fn document_animations(&mut self, relative_root: String) {
		let mut dir_queue = Vec::new();
		dir_queue.push(relative_root.to_owned());
		while !dir_queue.is_empty() {
			for dir in dir_queue.clone().iter() {
				if let Ok(dir) = read_dir(dir) {
					for dir_entry in dir {
						self.document_dir_entry_animations(&mut dir_queue, &relative_root, dir_entry.ok());
					}
				}
				dir_queue.sort_unstable();
				dir_queue.remove(dir_queue.binary_search(dir).unwrap());
			}
		}
	}
	fn document_dir_entry_scenes(&mut self, dir_queue: &mut Vec<String>, relative_root: &String, target: AssetTarget, dir_entry: Option<DirEntry>) -> Option<()> {
		let file = dir_entry?;
		if file.file_type().ok()?.is_dir() {
			dir_queue.push(file.path().to_string_lossy().to_string());
		}
		let file_name = file.file_name();
		let name = file_name.to_string_lossy();
		if name.strip_suffix(".gltf").is_some() || name.strip_suffix(".glb").is_some() {
			let relative_path = get_relative_path(&file, relative_root.as_str())?;
			let document = document_file(&file)?;
			for scene in document.scenes() {
				self.scenes.push(SceneMetadata {
					index: scene.index(),
					name: scene.name().unwrap_or_default().to_owned(),
					relative_path: relative_path.clone(),
					target,
				});
			}
		}
		Some(())
	}
	fn document_dir_entry_animations(&mut self, dir_queue: &mut Vec<String>, relative_root: &String, dir_entry: Option<DirEntry>) -> Option<()> {
		let file = dir_entry?;
		if file.file_type().ok()?.is_dir() {
			dir_queue.push(file.path().to_string_lossy().to_string());
		}
		let file_name = file.file_name();
		let name = file_name.to_string_lossy();
		if name.strip_suffix(".gltf").is_some() || name.strip_suffix(".glb").is_some() {
			let relative_path = get_relative_path(&file, relative_root.as_str())?;
			let document = document_file(&file)?;
			for animation in document.animations() {
				self.animations.push(AnimationMetadata {
					index: animation.index(),
					name: animation.name().unwrap_or_default().to_owned(),
					relative_path: relative_path.clone(),
				});
			}
		}
		Some(())
	}
}
/*
impl MaterialMeshMetadata {
	pub fn format_material_path(&self, material_index: Option<usize>) -> String {
		if let Some(index) = material_index {
			format!("{}#Material{}", self.relative_path, index.to_string())
		} else {
			format!("{}#DefaultMaterial", self.relative_path)
		}
	}
	pub fn format_mesh_path(&self, mesh_index: usize, primitive_index: usize) -> String {
		format!("{}#Mesh{}/Primitive{}", self.relative_path, mesh_index.to_string(), primitive_index.to_string())
	}
}*/

fn document_file(file: &DirEntry) -> Option<Document> {
	let root = Root::from_slice(read(file.path()).ok()?.as_slice()).ok()?;
	Document::from_json(root).ok()
}
/*
fn get_material_mesh_metadata(document: &Document, name: String, relative_path: String, target: AssetTarget) -> MaterialMeshMetadata {
	let materials = document.materials();
	let mut material_names = Vec::with_capacity(materials.len());
	for material in materials {
		if let Some(name) = material.name() {
			material_names.push(name.to_owned());
		} else {
			material_names.push(String::default());
		}
	}
	let meshes = document.meshes();
	let mut mesh_metadata = Vec::with_capacity(meshes.len());
	for mesh in meshes {
		let primitives = mesh.primitives();
		let mut primitive_metadata = Vec::with_capacity(primitives.len());
		for primitive in primitives {
			primitive_metadata.push(PrimitiveMeshMetadata {
				material: primitive.material().index(),
			});
		}
		let name;
		if let Some(mesh_name) = mesh.name() {
			name = mesh_name;
		} else {
			name = "";
		}
		let weights = mesh.weights().unwrap_or_default().to_vec();
		mesh_metadata.push(MeshMetadata {
			name: name.to_owned(),
			target: target.clone(),
			weights,
			primitives: primitive_metadata,
		});
	}
	MaterialMeshMetadata {
		name,
		relative_path,
		meshes: mesh_metadata,
		materials: material_names
	}
}*/

fn get_relative_path(file: &DirEntry, relative_root: &str) -> Option<String> {
	let path = file.path().to_str()?.to_owned();
	let mut relative_root = relative_root;
	if let Some(cleaned_root) = relative_root.strip_prefix("./") {
		relative_root = cleaned_root;
	}
	let i = path.find(relative_root)?;
	Some(path[i+ASSET_DIR.len()..].to_owned())
}

/*
#[derive(Component, Default)]
pub struct AssetSelection {
	pub meshes: Vec<MeshMetadata>,
}*/

#[derive(Component, Hash, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AssetTarget {
	#[default]
	Misc,
	Head,
	Body
}
/*
#[derive(Resource, Default)]
pub struct MorphInfo {
	id_counter: usize,
	sorted_names: HashMap<MorphId, String>,
	mesh_mappings: HashMap<Handle<Mesh>, Vec<MorphId>>,
}

pub enum MorphInfoError {
	NameNotRegistered,
	NameNotMorphTarget
}

impl MorphInfo {
	pub fn get_mapped_info<'a>(&'a mut self, handle: &Handle<Mesh>) -> Option<impl Iterator<Item = (&MorphId, &String)>> {
		let mapped_ids = self.mesh_mappings.get(handle)?;
		Some(ScriptedIter::new(&self.sorted_names, mapped_ids, |x, i| dbg!(x.get_key_value(&i))))
	}
	pub fn get_mut_mapped_info<'a>(&'a mut self, handle: &Handle<Mesh>) -> Option<impl Iterator<Item = (&MorphId, &mut String)>> {
		let mapped_ids = self.mesh_mappings.get(handle)?;
		Some(ScriptedMutIter::new(&mut self.sorted_names, mapped_ids, |x, i| x.get_key_value_mut(&i)))
	}
	pub fn get_mapped_values<'a>(&'a mut self, handle: &Handle<Mesh>) -> Option<impl Iterator<Item = &String>> {
		let mapped_ids = self.mesh_mappings.get(handle)?;
		Some(ScriptedMutIter::new(&mut self.sorted_names, mapped_ids, |x, i| x.get(&i)))
	}
	pub fn get_mapped_ids(&self, handle: &Handle<Mesh>) -> Option<impl Iterator<Item = &MorphId>> {
		Some(self.mesh_mappings.get(handle)?.iter())
	}
	pub fn queue_load_mesh_info(&mut self, handle: Handle<Mesh>) {
		if !self.contains(&handle) {
			self.mesh_mappings.insert(handle, Vec::new());
		}
	}
	pub fn count_morphs(&self, handle: &Handle<Mesh>) -> Option<usize> {
		Some(self.mesh_mappings.get(handle)?.len())
	}
	pub fn contains(&self, handle: &Handle<Mesh>) -> bool {
		self.mesh_mappings.contains_key(handle)
	}
	fn generate_morph_name_mapping(&mut self, handle: Handle<Mesh>, ordered_names: &[String]) {
		let mut ordered_ids: Vec<MorphId> = Vec::with_capacity(ordered_names.len());
		for name in ordered_names {
			match self.sorted_names.iter().find(|&(_, y)| y.eq(name)) {
				Some((&i, _)) => ordered_ids.push(i),
				None => {
					self.sorted_names.insert(self.id_counter, name.clone());
					ordered_ids.push(self.id_counter);
					self.id_counter += 1;
				},
			}
		}
		self.mesh_mappings.insert(handle, ordered_ids);
	}
}*/
/*
#[derive(Event, Debug)]
pub struct UpdateMesh {
	pub parent: Entity,
	// pub new_mesh: Handle<Mesh>,
	// pub weights: Vec<f32>
}

fn update_mesh(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	meshes: Res<Assets<Mesh>>,
	children_query: Query<&Children>,
	mesh_query: Query<&Handle<Mesh>>,
	mut events: ResMut<Events<UpdateMesh>>,

	mut morph_info: ResMut<MorphInfo>,
) {
	for update in events.drain() {
		for children in children_query.get(update.parent) {
			for &child in children {
				if let Some(mut entity_commands) = commands.get_entity(child) {
					if let Ok(mesh) = mesh_query.get(child) {
						if asset_server.load_state(mesh) == LoadState::Failed {
							continue;
						}
					}

						if let Ok(new_morphs) = MeshMorphWeights::new(update.weights) {
							entity_commands.insert(update.new_mesh.clone())
							.insert(new_morphs);
						}
				}
			}
		}
	}
}*/

