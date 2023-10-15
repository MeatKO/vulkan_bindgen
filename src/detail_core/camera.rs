use crate::cotangens::{
	vec3::*,
	mat4x4::*,
};

use crate::detail_core::{
	input_buffer::*,
};

use parmack::window::event::KeyCode;

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
			front: Vec3 { x: 1.0f32, y: 0.0f32, z: 0.0f32 },
			right: Vec3::new(0.0f32), // this will be calculated later
			up: Vec3::new(0.0f32), // this will be calculated later
			world_up: in_world_up,
			yaw: in_yaw,
			pitch: in_pitch,
			movement_speed: 0.005f32,
			mouse_sensitivity: 0.001f32,
			zoom: 1.0f32,
		}
	}

	pub fn get_front(&self) -> Vec3
	{
		return self.front.clone()
	}

	pub fn get_position(&self) -> Vec3
	{
		return self.position.clone()
	}

	pub fn get_rotation(&self) -> Vec3
	{
		return Vec3 { 
			// x: 0.0f32 / 360.0f32, 
			x: self.pitch, 
			// y: self.yaw, 
			y: self.yaw, 
			z: 0.0f32, 
			// z: self.pitch, 
		}
	}

	pub fn process_movement(&mut self, delta_time_ms: f32, input_buffer: &InputBuffer)
	{
		let velocity = self.movement_speed * delta_time_ms;

		if input_buffer.is_pressed(KeyCode::W as u8)
		{
			self.position += &(&self.front * &velocity)
		}
		if input_buffer.is_pressed(KeyCode::A as u8)
		{
			self.position -= &(&self.right * &velocity)
		}
		if input_buffer.is_pressed(KeyCode::S as u8)
		{
			self.position -= &(&self.front * &velocity)
		}
		if input_buffer.is_pressed(KeyCode::D as u8)
		{
			self.position += &(&self.right * &velocity)
		}
	}

	pub fn get_view_matrix(&self) -> Mat4x4
	{
		return Mat4x4::new_lookat(&self.position, &(&self.position + &self.front), &self.up);
	}

	pub fn update_camera_vectors(&mut self)
	{
		self.front = Vec3{
			x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			y: self.pitch.to_radians().sin(),
			z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
		}.normalize();

		self.right = self.front.cross(&self.world_up).normalize();
		self.up = self.right.cross(&self.front).normalize();
	}

	pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32)
	{
		let x_offset = x_offset * self.mouse_sensitivity;
		let y_offset = y_offset * self.mouse_sensitivity;

		let modulo_shit = self.yaw + x_offset;
		self.yaw = modulo_shit - (360.0f32 * (modulo_shit / 360.0f32).floor());

		self.pitch += y_offset;
		
		if self.pitch > 89.0f32
		{
			self.pitch = 89.0f32;
		}
		if self.pitch < -89.0f32
		{
			self.pitch = -89.0f32;
		}

		self.update_camera_vectors();
	}
}
