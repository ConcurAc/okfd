use bevy::{
	prelude::*,
	input::mouse::{
		MouseButton,
		MouseMotion,
	}
};
use bevy_egui::EguiContexts;

#[derive(States, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CameraMode {
	#[default]
	Tracked,
	FreePerspective,
	FreeFollowing(Entity),
	Perspective(Entity),
	Following(Entity),
}

pub(super) fn camera_control(
	camera_query: Query<&mut Transform, With<Camera>>,
	mut transform_query: Query<&mut Transform, Without<Camera>>,
	motion_reader: EventReader<MouseMotion>,
	mouse_buttons: Res<ButtonInput<MouseButton>>,
	time: Res<Time>,
	camera_state: Res<State<CameraMode>>,
	mut context: EguiContexts
) {
	if context.ctx_mut().wants_pointer_input() {
		return;
	}
	match *camera_state.get() {
		CameraMode::FreePerspective => camera_free_perspective(
			camera_query,
			motion_reader,
			time
		),
		CameraMode::FreeFollowing(entity) => camera_free_following(
			camera_query,
			transform_query.get_mut(entity).expect("camera target not found"),
			motion_reader,
			mouse_buttons,
			time
		),
		CameraMode::Perspective(entity) => camera_perspective(
			camera_query,
			transform_query.get(entity).expect("camera target not found"),
			motion_reader,
			time
		),
		CameraMode::Following(entity) => camera_following(
			camera_query,
			transform_query.get(entity).expect("camera target not found"),
			motion_reader,
			time
		),
		_ => ()
	}
}

fn camera_free_perspective(
	mut camera_query: Query<&mut Transform, With<Camera>>,
	mut motion_reader: EventReader<MouseMotion>,
	time: Res<Time>,
) {
	if let Some(motion) = motion_reader.read().max_by(|&x, &y| x.delta.length().total_cmp(&y.delta.length())) {
		let rotation_x = Quat::from_rotation_x(-motion.delta.y * time.delta_seconds());
		let rotation_y = Quat::from_rotation_y(-motion.delta.x * time.delta_seconds());
		for mut camera_transform in camera_query.iter_mut() {
			camera_transform.rotation *= rotation_x;
			camera_transform.rotation *= rotation_y;
			let forward = camera_transform.forward();
			camera_transform.look_to(forward.as_vec3(), Vec3::Y);
		}
	}
}

fn camera_free_following(
	mut camera_query: Query<&mut Transform, With<Camera>>,
	mut target_transform: Mut<Transform>,
	mut motion_reader: EventReader<MouseMotion>,
	mouse_buttons: Res<ButtonInput<MouseButton>>,
	time: Res<Time>,
) {
	if let Some(motion) = motion_reader.read().max_by(|&x, &y| x.delta.length().total_cmp(&y.delta.length())) {
		for input in mouse_buttons.get_pressed() {
			match input {
				/* Rotate */
				MouseButton::Left => {
					let rotation_x = Quat::from_rotation_x(-motion.delta.y * time.delta_seconds());
					let rotation_y = Quat::from_rotation_y(-motion.delta.x * time.delta_seconds());
					for mut camera_transform in camera_query.iter_mut() {
						camera_transform.look_at(target_transform.translation, Vec3::Y);
						let distance = (camera_transform.translation - target_transform.translation).length();
						camera_transform.translation = target_transform.translation;
						camera_transform.rotation *= rotation_x;
						camera_transform.rotation *= rotation_y;
						let back = camera_transform.back() * distance;
						camera_transform.translation += back;
						camera_transform.look_at(target_transform.translation, Vec3::Y);
					}
				},
				/* Pan */
				MouseButton::Middle => {
					for mut camera_transform in camera_query.iter_mut() {
						let right = camera_transform.right() * motion.delta.x * time.delta_seconds();
						let down = camera_transform.down() * motion.delta.y * time.delta_seconds();
						camera_transform.translation += right;
						camera_transform.translation += down;
						target_transform.translation += right;
						target_transform.translation += down;
					}
				},
				/* Zoom + Translate Y */
				MouseButton::Right => {
					for mut camera_transform in camera_query.iter_mut() {
						let distance = (camera_transform.translation - target_transform.translation).length();
						let neg_y = Vec3::NEG_Y * motion.delta.y * time.delta_seconds();
						let forward = Vec3::ZERO.lerp(camera_transform.forward() * motion.delta.x * time.delta_seconds(), distance);
						camera_transform.translation += neg_y;
						camera_transform.translation += forward;
						target_transform.translation += neg_y;
					}
				},
				_ => ()
			}
		}
	}
}

fn camera_perspective(
	mut camera_query: Query<&mut Transform, With<Camera>>,
	target_transform: &Transform,
	mut motion_reader: EventReader<MouseMotion>,
	time: Res<Time>,
) {
	if let Some(motion) = motion_reader.read().max_by(|&x, &y| x.delta.length().total_cmp(&y.delta.length())) {
		let rotation_x = Quat::from_rotation_x(-motion.delta.y * time.delta_seconds());
		let rotation_y = Quat::from_rotation_y(-motion.delta.x * time.delta_seconds());
		for mut camera_transform in camera_query.iter_mut() {
			camera_transform.rotation *= rotation_x;
			camera_transform.rotation *= rotation_y;
			let forward = camera_transform.forward();
			camera_transform.look_to(forward.as_vec3(), Vec3::Y);
		}
	} else {
		for mut camera_transform in camera_query.iter_mut() {
			camera_transform.translation = target_transform.translation;
		}
	}
}

fn camera_following(
	mut camera_query: Query<&mut Transform, With<Camera>>,
	target_transform: &Transform,
	mut motion_reader: EventReader<MouseMotion>,
	time: Res<Time>,
) {
	if let Some(motion) = motion_reader.read().max_by(|&x, &y| x.delta.length().total_cmp(&y.delta.length())) {
		let rotation_x = Quat::from_rotation_x(-motion.delta.y * time.delta_seconds());
		let rotation_y = Quat::from_rotation_y(-motion.delta.x * time.delta_seconds());
		for mut camera_transform in camera_query.iter_mut() {
			camera_transform.look_at(target_transform.translation, Vec3::Y);
			let distance = (camera_transform.translation - target_transform.translation).length();
			camera_transform.translation = target_transform.translation;
			camera_transform.rotation *= rotation_x;
			camera_transform.rotation *= rotation_y;
			let back = camera_transform.back() * distance;
			camera_transform.translation += back;
			camera_transform.look_at(target_transform.translation, Vec3::Y);
		}
	}
}


