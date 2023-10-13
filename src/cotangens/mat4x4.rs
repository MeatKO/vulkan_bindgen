use std::ops::{
	Mul,
	Add,
};

use crate::cotangens::vec3::*;

#[repr(align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Mat4x4
{
	pub data: [[f32; 4]; 4]
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

		out_mat.data[0][0] = 1.0f32;
		out_mat.data[1][1] = 1.0f32;
		out_mat.data[2][2] = 1.0f32;
		out_mat.data[3][3] = 1.0f32;

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
		let mut out_mat = Mat4x4::new();

		let f = 1.0f32 / (fov / 2.0f32).tan();
		let range = near - far;

		out_mat.data[0][0] = f / aspect_ratio;
		out_mat.data[1][1] = f * -1.0f32; // building it for vk
		out_mat.data[2][2] = (near + far) / range;
		out_mat.data[2][3] = (1.0f32 * near * far) / range;
		out_mat.data[3][2] = -1.0f32;
		out_mat.data[3][3] = 0.0f32;

		out_mat.transpose()
	}

	pub fn new_lookat(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4x4
	{
		let mut z_axis = Vec3{
			x: (center.x - eye.x),
			y: (center.y - eye.y),
			z: (center.z - eye.z)
		}.normalize();
		let x_axis = z_axis.cross(&up).normalize();
		let y_axis = x_axis.cross(&z_axis);

		z_axis = z_axis.negate();

		return Mat4x4 { 
			data: [
				[x_axis.x, x_axis.y, x_axis.z, -(x_axis.dot(&eye))],
				[y_axis.x, y_axis.y, y_axis.z, -(y_axis.dot(&eye))],
				[z_axis.x, z_axis.y, z_axis.z, -(z_axis.dot(&eye))],
				[0.0f32, 0.0f32, 0.0f32, 1.0f32]
			]
		}.transpose()
	}
}

// members
impl Mat4x4
{
	pub fn translate(&self, translation: Vec3) -> Mat4x4
	{
		let mut trans_mat = Mat4x4::new_identity(1.0f32);

		trans_mat.data[0][3] += translation.x;
		trans_mat.data[1][3] += translation.y;
		trans_mat.data[2][3] += translation.z;

		return *self + &trans_mat.transpose();
		// return *self * &rot_mat;
	}

	pub fn scale(&self, scale: Vec3) -> Mat4x4
	{
		let mut scale_mat = Mat4x4::new_identity(1.0f32);

		scale_mat.data[0][0] = scale.x;
		scale_mat.data[1][1] = scale.y;
		scale_mat.data[2][2] = scale.z;

		return *self * &scale_mat;
	}

	pub fn rotate_x(&self, degrees: f32) -> Mat4x4
	{
		let mut rot_mat = Mat4x4::new_identity(1.0f32);

		rot_mat.data[1][1] = degrees.cos();
		rot_mat.data[2][2] = degrees.cos();
		rot_mat.data[1][2] = degrees.sin();
		rot_mat.data[2][1] = -degrees.sin();

		return *self * &rot_mat;
	}

	pub fn rotate_y(&self, degrees: f32) -> Mat4x4
	{
		let mut rot_mat = Mat4x4::new_identity(1.0f32);

		rot_mat.data[0][0] = degrees.cos();
		rot_mat.data[2][2] = degrees.cos();
		rot_mat.data[0][2] = -degrees.sin();
		rot_mat.data[2][0] = degrees.sin();

		return *self * &rot_mat;
	}

	pub fn rotate_z(&self, degrees: f32) -> Mat4x4
	{
		let mut rot_mat = Mat4x4::new_identity(1.0f32);

		rot_mat.data[0][0] = degrees.cos();
		rot_mat.data[1][1] = degrees.cos();
		rot_mat.data[0][1] = degrees.sin();
		rot_mat.data[1][0] = -degrees.sin();

		return *self * &rot_mat;
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
			x: (self.data[0][0] * vec.x) + (self.data[1][0] * vec.y) + (self.data[2][0] * vec.z) + self.data[3][0],
			y: (self.data[0][1] * vec.x) + (self.data[1][1] * vec.y) + (self.data[2][1] * vec.z) + self.data[3][1],
			z: (self.data[0][2] * vec.x) + (self.data[1][2] * vec.y) + (self.data[2][2] * vec.z) + self.data[3][2],
		};

		let w = (self.data[0][3] * vec.x) + (self.data[1][3] * vec.y) + (self.data[2][3] * vec.z) + self.data[3][3];

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
		out_mat.data = [[0.0f32; 4]; 4];

		for i in 0..4 {
            for j in 0..4 {
                out_mat.data[i][j] = 0.0;
                for k in 0..4 {
                    out_mat.data[i][j] += self.data[i][k] * mat_in.data[k][j];
                }
            }
        }

		// out_mat.data[0][0] = (self.data[0][0] * mat_in.data[0][0]) + (self.data[1][0] * mat_in.data[0][1]) + (self.data[2][0] * mat_in.data[0][2]) + (self.data[3][0] * mat_in.data[0][3]);
		// out_mat.data[0][1] = (self.data[0][1] * mat_in.data[0][0]) + (self.data[1][1] * mat_in.data[0][1]) + (self.data[2][1] * mat_in.data[0][2]) + (self.data[3][1] * mat_in.data[0][3]);
		// out_mat.data[0][2] = (self.data[0][2] * mat_in.data[0][0]) + (self.data[1][2] * mat_in.data[0][1]) + (self.data[2][2] * mat_in.data[0][2]) + (self.data[3][2] * mat_in.data[0][3]);
		// out_mat.data[0][3] = (self.data[0][3] * mat_in.data[0][0]) + (self.data[1][3] * mat_in.data[0][1]) + (self.data[2][3] * mat_in.data[0][2]) + (self.data[3][3] * mat_in.data[0][3]);

		// out_mat.data[1][0] = (self.data[0][0] * mat_in.data[1][0]) + (self.data[1][0] * mat_in.data[1][1]) + (self.data[2][0] * mat_in.data[1][2]) + (self.data[3][0] * mat_in.data[1][3]);
		// out_mat.data[1][1] = (self.data[0][1] * mat_in.data[1][0]) + (self.data[1][1] * mat_in.data[1][1]) + (self.data[2][1] * mat_in.data[1][2]) + (self.data[3][1] * mat_in.data[1][3]);
		// out_mat.data[1][2] = (self.data[0][2] * mat_in.data[1][0]) + (self.data[1][2] * mat_in.data[1][1]) + (self.data[2][2] * mat_in.data[1][2]) + (self.data[3][2] * mat_in.data[1][3]);
		// out_mat.data[1][3] = (self.data[0][3] * mat_in.data[1][0]) + (self.data[1][3] * mat_in.data[1][1]) + (self.data[2][3] * mat_in.data[1][2]) + (self.data[3][3] * mat_in.data[1][3]);
		
		// out_mat.data[2][0] = (self.data[0][0] * mat_in.data[2][0]) + (self.data[1][0] * mat_in.data[2][1]) + (self.data[2][0] * mat_in.data[2][2]) + (self.data[3][0] * mat_in.data[2][3]);
		// out_mat.data[2][1] = (self.data[0][1] * mat_in.data[2][0]) + (self.data[1][1] * mat_in.data[2][1]) + (self.data[2][1] * mat_in.data[2][2]) + (self.data[3][1] * mat_in.data[2][3]);
		// out_mat.data[2][2] = (self.data[0][2] * mat_in.data[2][0]) + (self.data[1][2] * mat_in.data[2][1]) + (self.data[2][2] * mat_in.data[2][2]) + (self.data[3][2] * mat_in.data[2][3]);
		// out_mat.data[2][3] = (self.data[0][3] * mat_in.data[2][0]) + (self.data[1][3] * mat_in.data[2][1]) + (self.data[2][3] * mat_in.data[2][2]) + (self.data[3][3] * mat_in.data[2][3]);

		// out_mat.data[3][0] = (self.data[0][0] * mat_in.data[3][0]) + (self.data[1][0] * mat_in.data[3][1]) + (self.data[2][0] * mat_in.data[3][2]) + (self.data[3][0] * mat_in.data[3][3]);
		// out_mat.data[3][1] = (self.data[0][1] * mat_in.data[3][0]) + (self.data[1][1] * mat_in.data[3][1]) + (self.data[2][1] * mat_in.data[3][2]) + (self.data[3][1] * mat_in.data[3][3]);
		// out_mat.data[3][2] = (self.data[0][2] * mat_in.data[3][0]) + (self.data[1][2] * mat_in.data[3][1]) + (self.data[2][2] * mat_in.data[3][2]) + (self.data[3][2] * mat_in.data[3][3]);
		// out_mat.data[3][3] = (self.data[0][3] * mat_in.data[3][0]) + (self.data[1][3] * mat_in.data[3][1]) + (self.data[2][3] * mat_in.data[3][2]) + (self.data[3][3] * mat_in.data[3][3]);

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