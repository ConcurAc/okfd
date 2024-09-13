use bevy::{
	prelude::*,
	reflect::TypePath,
	render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct TiledMaterial {
	#[uniform(0)]
	params: Vec3,
	#[texture(1)]
	#[sampler(2)]
	#[dependency]
	pub base_color_texture: Option<Handle<Image>>,
	pub alpha_mode: AlphaMode,
}

impl Material for TiledMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/tiling.wgsl".into()
	}
	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}
}

impl TiledMaterial {
	pub fn with_image(&mut self, image: Handle<Image>) -> &mut Self {
		self.base_color_texture = Some(image);
		self
	}
	pub fn with_scale(&mut self, scale: f32) -> &mut Self{
		self.params[0] = scale;
		self
	}
	pub fn with_rotation(&mut self, rotation: f32) -> &mut Self {
		self.params[1] = rotation;
		self
	}
	pub fn with_noise(&mut self, noise: f32) -> &mut Self {
		self.params[2] = noise;
		self
	}
}
