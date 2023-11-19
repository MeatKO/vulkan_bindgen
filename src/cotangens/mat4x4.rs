use std::ops::{
	Mul,
	Add,
};

use crate::cotangens::vec3::*;

#[repr(align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Mat4x4
{
	pub data: [[f32; 4]; 4] // Coooooooolumn majoooooooooor
}

// constructors
impl Mat4x4
{
	// identity matrix
	pub fn new() -> Mat4x4
	{
		let mut out_mat = Mat4x4{
			data: [[0f32; 4]; 4]
		};

		// out_mat.data[0][0] = 1.0f32;
		// out_mat.data[1][1] = 1.0f32;
		// out_mat.data[2][2] = 1.0f32;
		// out_mat.data[3][3] = 1.0f32;

		out_mat
	}

	// identity matrix
	pub fn new_identity(diagonal_value: f32) -> Mat4x4
	{
		let mut out_mat = Mat4x4{
			data: [[0f32; 4]; 4]
		};

		out_mat.data[0][0] = diagonal_value;
		out_mat.data[1][1] = diagonal_value;
		out_mat.data[2][2] = diagonal_value;
		out_mat.data[3][3] = diagonal_value;

		out_mat
	}

	pub fn new_perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4x4
	{
		let mut out_mat = Mat4x4::new_identity(1.0f32);

		let f = 1.0f32 / (fov / 2.0f32).tan();
		let range = near - far;

		out_mat.data[0][0] = f / aspect_ratio;
		out_mat.data[1][1] = -f; // building it for vk
		out_mat.data[2][2] = far / range;
		out_mat.data[3][2] = (near * far) / range;
		out_mat.data[2][3] = -1.0f32;
		out_mat.data[3][3] = 0.0f32;


		out_mat
	}

	pub fn new_orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4x4 
	{
		let mut out_mat = Mat4x4::new_identity(1.0f32);
	
		out_mat.data[0][0] = 2.0f32 / (right - left);
		out_mat.data[1][1] = 2.0f32 / (top - bottom);
		out_mat.data[2][2] = -2.0f32 / (far - near);
		
		out_mat.data[3][0] = -(right + left) / (right - left);
		out_mat.data[3][1] = -(top + bottom) / (top - bottom);
		out_mat.data[3][2] = -(far + near) / (far - near);
		out_mat.data[3][3] = 1.0f32;
	
		out_mat
	}

	pub fn new_lookat(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4x4
	{
		let mut z_axis = 
			Vec3{
				x: (center.x - eye.x),
				y: (center.y - eye.y),
				z: (center.z - eye.z)
			}.normalize();
		let x_axis = z_axis.cross(&up.normalize()).normalize();
		let y_axis = x_axis.cross(&z_axis);

		z_axis = z_axis.negate();

		return Mat4x4 { 
			data: [
				[x_axis.x, x_axis.y, x_axis.z, -(x_axis.dot(&eye))],
				[y_axis.x, y_axis.y, y_axis.z, -(y_axis.dot(&eye))],
				[z_axis.x, z_axis.y, z_axis.z, -(z_axis.dot(&eye))],
				[0.0f32, 0.0f32, 0.0f32, 1.0f32]
			]
		// }
		}.transpose()
	}
}

// members
impl Mat4x4
{
	pub fn translate(&self, translation: Vec3) -> Mat4x4
	{
		let mut trans_mat = Mat4x4::new_identity(0.0f32);

		trans_mat.data[3][0] = translation.x;
		trans_mat.data[3][1] = translation.y;
		trans_mat.data[3][2] = translation.z;

		// return trans_mat + self;
		return *self + &trans_mat;
	}

	pub fn scale(&self, scale: Vec3) -> Mat4x4
	{
		let mut scale_mat = Mat4x4::new_identity(1.0f32);

		scale_mat.data[0][0] = scale.x;
		scale_mat.data[1][1] = scale.y;
		scale_mat.data[2][2] = scale.z;

		// return scale_mat * self;
		return *self * &scale_mat;
	}

	pub fn rotate_x(&self, degrees: f32) -> Mat4x4
	{
		let mut rot_mat = Mat4x4::new_identity(1.0f32);
		let radians = degrees.to_radians();

		rot_mat.data[1][1] = radians.cos();
		rot_mat.data[1][2] = -radians.sin();
		rot_mat.data[2][1] = radians.sin();
		rot_mat.data[2][2] = radians.cos();

		return *self * &rot_mat;
		// return rot_mat * self;
	}

	pub fn rotate_y(&self, degrees: f32) -> Mat4x4
	{
		let mut rot_mat = Mat4x4::new_identity(1.0f32);
		let radians = degrees.to_radians();

		rot_mat.data[0][0] = radians.cos();
		rot_mat.data[0][2] = radians.sin();
		rot_mat.data[2][0] = -radians.sin();
		rot_mat.data[2][2] = radians.cos();

		return *self * &rot_mat;
		// return rot_mat * self;
	}

	pub fn rotate_z(&self, degrees: f32) -> Mat4x4
	{
		let mut rot_mat = Mat4x4::new_identity(1.0f32);
		let radians = degrees.to_radians();

		rot_mat.data[0][0] = radians.cos();
		rot_mat.data[0][1] = radians.sin();
		rot_mat.data[1][0] = -radians.sin();
		rot_mat.data[1][1] = radians.cos();

		return *self * &rot_mat;
		// return rot_mat * self;
	}

	pub fn transpose(&self) -> Mat4x4
	{
		return Mat4x4 { 
			data: [
				[self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0]],
				[self.data[0][1], self.data[1][1], self.data[2][1], self.data[3][1]],
				[self.data[0][2], self.data[1][2], self.data[2][2], self.data[3][2]],
				[self.data[0][3], self.data[1][3], self.data[2][3], self.data[3][3]]
			]
		}
	}
}

// Multiplying a 4x4 matrix by 3-dimensional vector to confuse the mathematicians 
impl Mul<&Vec3> for &Mat4x4 
{
    type Output = Vec3;

    fn mul(self, vec: &Vec3) -> Vec3 
	{
        let mut out_vec = Vec3{
			x: (self.data[0][0] * vec.x) + (self.data[0][1] * vec.y) + (self.data[0][2] * vec.z) + self.data[0][3],
			y: (self.data[1][0] * vec.x) + (self.data[1][1] * vec.y) + (self.data[1][2] * vec.z) + self.data[1][3],
			z: (self.data[2][0] * vec.x) + (self.data[2][1] * vec.y) + (self.data[2][2] * vec.z) + self.data[2][3],
		};
	
		let w = (self.data[3][0] * vec.x) + (self.data[3][1] * vec.y) + (self.data[3][2] * vec.z) + self.data[3][3];
	
		if w != 1.0f32
		{
			out_vec.x /= w;
			out_vec.y /= w;
			out_vec.z /= w;
		}
	
		out_vec
    }
}

impl Mul<&Mat4x4> for Mat4x4 
{
    type Output = Mat4x4;

    fn mul(self, mat_in: &Mat4x4) -> Mat4x4 
	{
        let mut out_mat: Mat4x4 = Mat4x4::new();

		for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    out_mat.data[i][j] += self.data[i][k] * mat_in.data[k][j];
                }
            }
        }

		out_mat
    }
}

impl Add<&Mat4x4> for Mat4x4 
{
	type Output = Mat4x4;

    fn add(self, mat_in: &Mat4x4) -> Mat4x4 
	{
		let mut out_mat: Mat4x4 = Mat4x4::new();

		for i in 0..4
		{
			for j in 0..4
			{
				out_mat.data[i][j] = self.data[i][j] + mat_in.data[i][j];
			}
		}

		out_mat
	}
}