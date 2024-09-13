
/* equilibrium point is implicitly at 0.0 */
#[derive(Default)]
pub struct SpringSystem {
	pub equilibrium: f32,
	pub upper_bound: f32,
	pub lower_bound: f32,
	pub linear_density: f32,
	pub dampening: f32,
}

impl SpringSystem {
	pub fn with_critical_dampening(mut self, mass: f32) -> Self {
		self.dampening = f32::sqrt(4. * self.linear_density * mass);
		self
	}
	pub fn compute_force<'w, 's>(&self, v: f32, x: f32) -> f32 {
		if x < self.lower_bound || x > self.upper_bound {
			return f32::default();
		}
		self.linear_density * (self.equilibrium - x) - self.dampening * v
	}
}
