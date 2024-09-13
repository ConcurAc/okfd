use bevy::prelude::*;

#[derive(Component)]
pub struct MovementSystem {
	pub max_run_speed: f32,
	pub max_sprint_speed: f32,
	pub acceleration: f32,
	pub deceleration: f32,
	previous_velocity: Vec3,
}

impl Default for MovementSystem {
	fn default() -> Self {
		Self {
			max_run_speed: 1.0,
			max_sprint_speed: 1.5,
			acceleration: 1.0,
			deceleration: 1.0,
			previous_velocity: Vec3::ZERO,
		}
	}
}

impl MovementSystem {
	pub fn new(max_run_speed: f32, max_sprint_speed: f32, acceleration: f32, deceleration: f32) -> Self {
		debug_assert!(max_run_speed >= 0. && max_sprint_speed >= max_run_speed);
		debug_assert!(acceleration >= 0. && deceleration <= 0.);
		Self {
			max_run_speed,
			max_sprint_speed,
			acceleration,
			deceleration,
			..default()
		}
	}
	pub fn reset_velocity(&mut self) -> Vec3 {
		let previous_velocity = self.previous_velocity;
		self.previous_velocity = Vec3::ZERO;
		previous_velocity
	}

	pub fn stop(&self, velocity: &mut Vec3, delta_seconds: f32) {
		*velocity += velocity.normalize_or_zero() * self.deceleration * delta_seconds;
	}

	pub fn stop_horizontal(&self, velocity: &mut Vec3, delta_seconds: f32) {
		*velocity += velocity.with_y(0.) * self.deceleration * delta_seconds;
	}

	fn update_velocity(&mut self, direction: Dir3, delta_seconds: f32, max_speed: f32) -> Vec3 {
		let mut velocity = (direction * self.acceleration * delta_seconds).clamp_length_max(max_speed);
		dbg!(velocity.length());
		dbg!(self.previous_velocity.length());
		if self.previous_velocity != Vec3::ZERO {
			let tangent = velocity.project_onto(self.previous_velocity);
			let normal = velocity - tangent;
			let difference = {
				let distance_sq = self.previous_velocity.distance_squared(tangent);
				if distance_sq < tangent.length_squared() {
					tangent - self.previous_velocity
				} else {
					tangent + self.previous_velocity
				}
			};
			velocity -= tangent - self.previous_velocity + difference.clamp_length_max(max_speed);
			velocity -= normal.clamp_length_max(max_speed);
		} else {
			velocity = velocity.clamp_length_max(max_speed);
		}
		self.previous_velocity += dbg!(velocity);
		velocity
	}

	pub fn update_run_velocity(&mut self, direction: Dir3, delta_seconds: f32) -> Vec3 {
		self.update_velocity(direction, delta_seconds, self.max_run_speed)
	}

	pub fn update_run_velocity_horizontal(&mut self, direction: Vec3, delta_seconds: f32) -> Vec3 {
		if let Ok(direction) = Dir3::new(direction.with_y(0.)) {
			self.update_run_velocity(direction, delta_seconds)
		} else {
			Vec3::ZERO
		}
	}

	pub fn update_sprint_velocity(&mut self, direction: Dir3, delta_seconds: f32) -> Vec3 {
		self.update_velocity(direction, delta_seconds, self.max_sprint_speed)
	}

	pub fn update_sprint_velocity_horizontal(&mut self, direction: Vec3, delta_seconds: f32) -> Vec3 {
		if let Ok(direction) = Dir3::new(direction.with_y(0.)) {
			self.update_sprint_velocity(direction, delta_seconds)
		} else {
			Vec3::ZERO
		}
	}

	fn update_impulse(&mut self, velocity: Vec3, direction: Dir3, max_speed: f32) -> Vec3 {
		let impulse = direction * self.acceleration;
		if velocity.distance_squared(impulse) > velocity.length_squared() - impulse.length_squared() {
			impulse.clamp_length_max(max_speed - velocity.length())
		} else {
			Vec3::ZERO
		}

	}

	pub fn update_run_impulse(&mut self, velocity: Vec3, direction: Dir3) -> Vec3 {
		self.update_impulse(velocity, direction, self.max_run_speed)
	}

	pub fn update_run_impulse_horizontal(&mut self, velocity: Vec3, direction: Vec3) -> Vec3 {
		if let Ok(direction) = Dir3::new(direction.with_y(0.)) {
			self.update_run_impulse(velocity, direction)
		} else {
			Vec3::ZERO
		}
	}

	pub fn update_sprint_impulse(&mut self, velocity: Vec3, direction: Dir3) -> Vec3 {
		self.update_impulse(velocity, direction, self.max_sprint_speed)
	}

	pub fn update_sprint_impulse_horizontal(&mut self, velocity: Vec3, direction: Vec3) -> Vec3 {
		if let Ok(direction) = Dir3::new(direction.with_y(0.)) {
			self.update_sprint_impulse(velocity, direction)
		} else {
			Vec3::ZERO
		}
	}
}
