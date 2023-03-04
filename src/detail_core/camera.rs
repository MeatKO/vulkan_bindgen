use crate::cotangens::{
	vec3::*,
	mat4x4::*,
};

use crate::detail_core::{
	input_buffer::*,
};

use crate::loseit::{
	window_events::*,
};

pub enum CameraMovement
{
	FORWARD,
	BACKWARD,
	LEFT,
	RIGHT
}

pub struct Camera
{
	pub position: Vec3,
	front: Vec3,
	up: Vec3,
	right: Vec3,
	world_up: Vec3,

	// We only need 2 angles to avoid gimbal locking
	yaw: f32,
	pitch: f32,

	movement_speed: f32,
	mouse_sensitivity: f32,
	zoom: f32
}

impl Camera 
{
	pub fn new(
		in_position: Vec3,
		in_world_up: Vec3,
		in_yaw: f32,
		in_pitch: f32,
	) -> Camera
	{
		return Camera{
			position: in_position,
			front: Vec3 { x: 0.0f32, y: 0.0f32, z: 1.0f32 },
			right: Vec3::new(0.0f32), // this will be calculated later
			up: Vec3::new(0.0f32), // this will be calculated later
			world_up: in_world_up,
			yaw: in_yaw,
			pitch: in_pitch,
			movement_speed: 0.01f32,
			mouse_sensitivity: 0.1f32,
			zoom: 1.0f32,
		}
	}

	pub fn process_movement(&mut self, delta_time_ms: f32, input_buffer: &InputBuffer)
	{
		let velocity = self.movement_speed * delta_time_ms;

		if input_buffer.is_pressed(KeyValues::W as u8)
		{
			self.position += &(&self.front * &velocity)
		}
		if input_buffer.is_pressed(KeyValues::A as u8)
		{
			self.position -= &(&self.right * &velocity)
		}
		if input_buffer.is_pressed(KeyValues::S as u8)
		{
			self.position -= &(&self.front * &velocity)
		}
		if input_buffer.is_pressed(KeyValues::D as u8)
		{
			self.position += &(&self.right * &velocity)
		}
	}

	pub fn get_view_matrix(&self) -> Mat4x4
	{
		return Mat4x4::new_lookat(self.position.clone(), &self.position + &self.front, self.up.clone());
		// return Mat4x4::new_lookat(self.position.clone(), Vec3::new(0.0f32), self.up.clone());
	}

	pub fn update_camera_vectors(&mut self)
	{
		let front = Vec3{
			x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			y: self.pitch.to_radians().sin(),
			z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
		}.normalize();

		self.right = self.front.cross(&self.world_up).normalize();
		self.up = self.right.cross(&self.front).normalize();
	}
}
