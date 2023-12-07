use crate::detail_core::texture::texture::{VulkanTexture, Texture};


#[derive(Default, Clone, Debug)]
pub struct Material
{
	pub name: String,

	pub albedo_path: String,
	pub normal_path: String,

	pub albedo_map: Option<Texture<VulkanTexture>>,
	pub normal_map: Option<Texture<VulkanTexture>>
}