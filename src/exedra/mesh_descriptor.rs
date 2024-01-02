use super::material_descriptor::MaterialDescriptor;
use crate::cotangens::{vec3::Vec3, vecn::VecN};

#[derive(Default)]
pub struct MeshDescriptor
{
	pub name: String,
	pub mtl_name: String,
	pub material: MaterialDescriptor,
	// pub face_vec: Vec<Vec3>, //		v / vt / vn
	pub face_vtn_vec: Vec<VecN<usize, 3>>, //		v / vt / vn
}
