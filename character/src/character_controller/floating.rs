use bevy::prelude::*;
use avian3d::prelude::*;

use physics::spring::SpringSystem;

#[derive(Component, Default)]
pub struct FloatingSystem {
	pub enable: bool,
	pub spring: SpringSystem,
	pub slope_critical_angle: f32,
}

impl FloatingSystem {
	pub fn new(mass: Mass, float_height: f32, speed: f32, max_slope_angle: f32) -> Self {
		Self {
			enable: true,
			spring: SpringSystem {
				linear_density: speed,
				upper_bound: float_height,
				lower_bound: -float_height,
				..default()
			}.with_critical_dampening(*mass),
			slope_critical_angle: max_slope_angle,
			..default()
		}
	}
	pub fn compute_force(&self, y_velocity: f32, hit: RayHitData) -> Vec3 {
		if self.enable {
			if hit.normal == Vec3::ZERO || f32::to_radians(90.0) - Dir3::Y.angle_between(hit.normal) > self.slope_critical_angle {
				return Vec3::ZERO.with_y(self.spring.compute_force(y_velocity, hit.time_of_impact - self.spring.upper_bound));
			}
		}
		Vec3::ZERO
	}
	pub fn cast_ray(&self, spatial_query: &SpatialQuery, filter: SpatialQueryFilter, position: Vec3) -> Option<RayHitData> {
		spatial_query.cast_ray(
			position.with_y(position.y + self.spring.equilibrium),
			Dir3::NEG_Y,
			self.spring.upper_bound - self.spring.lower_bound,
			true,
			filter,
		)
	}
	pub fn float_height(&self) -> f32 {
		self.spring.equilibrium - self.spring.lower_bound
	}
	pub fn float_range(&self) -> f32 {
		self.spring.upper_bound - self.spring.lower_bound
	}
}
