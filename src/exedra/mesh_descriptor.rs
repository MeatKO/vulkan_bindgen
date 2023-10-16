use super::material_descriptor::MaterialDescriptor;
use crate::cotangens::vec3::Vec3;

#[derive(Default)]
pub struct MeshDescriptor
{
	pub name: String,
	pub mtl_name: String,
	pub material: MaterialDescriptor,
	pub face_vec: Vec<Vec3>, //		v / vt / vn
}

// impl MeshDescriptor
// {
// 	pub fn new(mesh_name: String) -> MeshDescriptor
// 	{
// 		MeshDescriptor { 
// 			name: mesh_name,
// 			..Default::default()
// 		}
// 	}
// }