use crate::{detail_core::texture::texture::{VulkanTexture, Texture}, vulkan::vk_bindgen::VkDescriptorSet};

use std::ptr::null_mut as nullptr;

#[derive(Clone, Debug)]
pub struct Material
{
	pub name: String,

	pub descriptor_set: VkDescriptorSet,

	pub albedo_path: String,
	pub normal_path: String,

	pub albedo_map: Option<Texture<VulkanTexture>>,
	pub normal_map: Option<Texture<VulkanTexture>>
}

impl Default for Material
{
	fn default() -> Self
	{
		Material
		{
			name: String::from("default"),
			descriptor_set: nullptr(),
			albedo_path: String::from(""),
			normal_path: String::from(""),
			albedo_map: None,
			normal_map: None,
		}
	}
}