use decs::component_derive::component;
use decs::component::Component;

use crate::vulkan::vk_bindgen::{VkDescriptorPool, VkDescriptorSet};

#[component]
pub struct VulkanModelComponent
{
	pub model_asset_name: String,
	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_sets: Vec<VkDescriptorSet>,
}

impl VulkanModelComponent
{
	pub fn new(model_asset_name: String, ) -> Self
	{
		Self
		{
			model_asset_name,
			descriptor_pool: std::ptr::null_mut(),
			descriptor_sets: Vec::new(),
		}
	}
}