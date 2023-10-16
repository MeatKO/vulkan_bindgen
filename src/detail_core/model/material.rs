use crate::detail_core::texture::texture::{VulkanTexture, Texture};


#[derive(Default)]
pub struct Material
{
	pub name: String,

	pub albedo_map: Option<Texture<VulkanTexture>>,
	pub normal_map: Option<Texture<VulkanTexture>>
}

// impl Material
// {
// 	pub fn new(material_name: String) -> Material
// 	{
// 		Material { 
// 			name: material_name, 
// 			..Default::default()
// 		}
// 	}
// }