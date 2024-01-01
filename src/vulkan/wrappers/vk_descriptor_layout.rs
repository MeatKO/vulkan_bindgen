use crate::vulkan::vk_bindgen::{VkDescriptorSetLayoutBinding, VkDescriptorType, VkDescriptorSetLayout, vkCreateDescriptorSetLayout, VkResult, VkDevice, VkDescriptorSetLayoutCreateInfo, VkStructureType, VkShaderStageFlagBits};
use std::ptr::null_mut as nullptr;
pub struct VkDescriptorLayoutBuilder 
{
	builder_descriptor_types: Vec<VkDescriptorType>
}

impl VkDescriptorLayoutBuilder 
{
	pub fn new() -> VkDescriptorLayoutBuilder
	{
		VkDescriptorLayoutBuilder
		{
			builder_descriptor_types: Vec::new()
		}
	}

	pub fn add_binding(mut self, binding: VkDescriptorType) -> VkDescriptorLayoutBuilder
	{
		self.builder_descriptor_types.push(binding);
		self
	}

	pub fn build(self, logical_device: VkDevice) -> Result<VkDescriptorSetLayout, String>
	{
		let mut bindings_vec: Vec<VkDescriptorSetLayoutBinding> = vec![];
		for descriptor_type in self.builder_descriptor_types.iter()
		{
			bindings_vec.push(
				VkDescriptorSetLayoutBinding
				{
					binding: bindings_vec.len() as u32,
					descriptorType: *descriptor_type,
					descriptorCount: 1,
					stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32 |
								VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32,
					pImmutableSamplers: std::ptr::null()
				}
			);
		}

		let descriptor_set_layout_create_info = 
		VkDescriptorSetLayoutCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			bindingCount: bindings_vec.len() as u32,
			pBindings: bindings_vec.as_ptr(),
			flags: 0,	
			pNext: nullptr(),
		};

		let mut descriptor_set_layout = nullptr();
		match unsafe{ vkCreateDescriptorSetLayout(logical_device, &descriptor_set_layout_create_info, nullptr(), &mut descriptor_set_layout) }
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateDescriptorSetLayout()"); }
			err => { panic!("✗ vkCreateDescriptorSetLayout() failed with code {:?}.", err); }
		}

		Ok(descriptor_set_layout)
	}
}