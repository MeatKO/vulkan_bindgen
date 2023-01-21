use super::vk_bindgen::*;
use super::handle::VkHandle;

use std::ptr::null_mut as nullptr;

pub fn create_shader_module(vk_handle: &VkHandle , shader_source: &[u8]) -> VkShaderModule
{
	let shader_create_info = VkShaderModuleCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
		codeSize: shader_source.len(),
		pCode: shader_source.as_ptr() as _,
		flags: 0,
		pNext: nullptr(),
	};

	unsafe
	{
		let mut shader_module = std::mem::zeroed();
		match vkCreateShaderModule(vk_handle.logical_device, &shader_create_info, nullptr(), &mut shader_module)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateShaderModule()"); }
			err => { panic!("✗ vkCreateShaderModule() failed with code {:?}.", err); }
		}

		shader_module
	}
}