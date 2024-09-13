mod movement;
mod jump;
mod floating;

use movement::MovementSystem;
use floating::FloatingSystem;

use std::f32::consts::PI;

use bevy::prelude::*;
use avian3d::prelude::*;

use world::SpatialTypes;

const FLOAT_HEIGHT: f32 = 0.095;
const LINEAR_DENSITY: f32 = 5.0;
const SLOPE_CRITICAL_ANGLE: f32 = PI / 6.0;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (
				controller_floating,
			)
		);
	}
}

#[derive(Component, Default)]
pub enum CharacterState {
	#[default]
	Able,
	Unable,
	AirBorne,
}

#[derive(Component, Default)]
pub struct CharacterController {
	pub state: CharacterState,
	pub movement: MovementSystem,
	pub floating: FloatingSystem,
}

impl CharacterController {
	pub fn new(mass: Mass) -> Self {
		Self {
			floating: FloatingSystem::new(mass, FLOAT_HEIGHT, LINEAR_DENSITY, SLOPE_CRITICAL_ANGLE),
			movement: MovementSystem::new(2., 3.5, 245.0, -15.0),
			..default()
		}
	}
}

fn controller_floating(
	mut systems_query: Query<(&mut CharacterController, &Transform, &mut ExternalForce, &LinearVelocity)>,
	spatial_query: SpatialQuery
) {
	for (mut controller, transform, mut force, linear_velocity) in systems_query.iter_mut() {
		let filter = SpatialQueryFilter::from_mask(SpatialTypes::Character);
		if let Some(hit) = controller.floating.cast_ray(&spatial_query, filter, transform.translation) {
			match controller.state {
				CharacterState::AirBorne => controller.state = CharacterState::Able,
				_ => (),
			}
			force.persistent = false;
			force.apply_force(controller.floating.compute_force(linear_velocity.y, hit));
		} else {
			controller.state = CharacterState::AirBorne;
			controller.floating.enable = true;
		}
	}
}
