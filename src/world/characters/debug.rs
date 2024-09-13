use crate::world::SpatialTypes;

use bevy::{
	prelude::*,
	utils::HashMap,
};
use avian3d::prelude::*;

use character::*;

#[derive(Component, Default)]
pub struct DebugCharacter {
	pub height: f32
}

impl Character for DebugCharacter {
	const FILE: &'static str = "character/body/base.gltf";
	fn scene(asset_server: &AssetServer) -> Handle<Scene> {
		asset_server.load(format!("{}#Scene0", Self::FILE))
	}
	fn animations(asset_server: &AssetServer) -> CharacterActionClips {
		let mut animations = Vec::new();
		animations.push((
		CharacterAction::Idle,
		asset_server.load(format!("{}#Animation0", Self::FILE))
		));
		animations.push((
		CharacterAction::Forward,
		asset_server.load(format!("{}#Animation1", Self::FILE))
		));
		CharacterActionClips(animations)
	}
	fn mass_properties(&self) -> MassPropertiesBundle {
		let collider = Collider::cuboid(1.0, self.height, 1.0);
		let mut mass_properties = MassPropertiesBundle::new_computed(&collider, 1.0);
		mass_properties.center_of_mass.y = self.height / 2.0;
		mass_properties
	}
	fn collider_constructor_hierarchy() -> ColliderConstructorHierarchy {
		let mut config = HashMap::new();
		config.insert(
			String::from("head"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.08,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.04, 0.),
				}),
				..default()
			})
		);
		config.insert(
			String::from("neck"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.04,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.07, 0.),
				}),
				..default()
			})
		);
		config.insert(
			String::from("chest"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::Cuboid {
					x_length: 0.185,
					y_length: 0.2,
					z_length: 0.1,
				}),
				..default()
			})
		);
		config.insert(
			String::from("waist"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::Cuboid {
					x_length: 0.175,
					y_length: 0.1,
					z_length: 0.1,
				}),
				..default()
			})
		);
		config.insert(
			String::from("pelvis"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::Cuboid {
					x_length: 0.2,
					y_length: 0.125,
					z_length: 0.1,
				}),
				..default()
			})
		);
		config.insert(
			String::from("left_breast"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::Sphere {
					radius: 0.04,
				}),
				..default()
			})
		);
		config.insert(
		String::from("right_breast"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::Sphere {
					radius: 0.04,
				}),
				..default()
			})
		);
		config.insert(
		String::from("left_shoulder"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.03,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.175, 0.)
				}),
				..default()
			})
		);
		config.insert(
		String::from("right_shoulder"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.03,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.175, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("left_elbow"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.03,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.2, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("right_elbow"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.03,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.2, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("left_hip"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.05,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.25, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("left_hip"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.05,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.25, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("right_hip"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.05,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.25, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("left_knee"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.04,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.3, 0.)
				}),
				..default()
			})
		);
		config.insert(
			String::from("right_knee"),
			Some(ColliderConstructorHierarchyConfig {
				constructor: Some(ColliderConstructor::CapsuleEndpoints {
					radius: 0.04,
					a: Vec3::ZERO,
					b: Vec3::new(0., 0.3, 0.)
				}),
				..default()
			})
		);
		ColliderConstructorHierarchy {
			config,
			default_layers: CollisionLayers::new(SpatialTypes::Character, LayerMask::NONE),
			..default()
		}
	}
}

