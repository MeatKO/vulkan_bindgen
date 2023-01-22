use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_framebuffer(vk_handle: &mut VkHandle)
{
	// Framebuffers
	vk_handle.swapchain_framebuffers = vec![std::mem::zeroed();  vk_handle.swapchain_image_views_vec.len()];

	for i in 0.. vk_handle.swapchain_image_views_vec.len()
	{
		let attachments = vec![ vk_handle.swapchain_image_views_vec[i]];

		let framebuffer_create_info = VkFramebufferCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
			renderPass: vk_handle.render_pass,
			attachmentCount: 1,
			pAttachments: attachments.as_ptr(),
			width: vk_handle.extent.width,
			height: vk_handle.extent.height,
			layers: 1,
			flags: 0,	
			pNext: nullptr(),
		};

		match vkCreateFramebuffer(vk_handle.logical_device, &framebuffer_create_info, nullptr(), &mut vk_handle.swapchain_framebuffers[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateFramebuffer()"); }
			err => { panic!("✗ vkCreateFramebuffer() failed with code {:?}.", err); }
		}	
	}
}