use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_framebuffers(
	// vk_handle: &mut VkHandle,
	device: &VkDevice,
	swapchain_image_views_vec: &Vec<VkImageView>,
	depth_image_view: &VkImageView,
	render_pass: &VkRenderPass,
	swapchain_extent: &VkExtent2D,
) -> Vec<VkFramebuffer>
{
	// Framebuffers
	// vk_handle.swapchain_framebuffers = vec![nullptr();  vk_handle.swapchain_image_views_vec.len()];
	let mut swapchain_framebuffers = vec![nullptr(); swapchain_image_views_vec.len()];

	for i in 0..swapchain_image_views_vec.len()
	{
		let attachments = 
			vec![
				swapchain_image_views_vec[i],
				depth_image_view.clone(),
			];

		let framebuffer_create_info = 
			VkFramebufferCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
				renderPass: *render_pass, // I have no fucking idea why this is here and there is a debate @ https://github.com/KhronosGroup/Vulkan-Docs/issues/1147
				attachmentCount: attachments.len() as _,
				pAttachments: attachments.as_ptr(),
				width: swapchain_extent.width,
				height: swapchain_extent.height,
				layers: 1,
				flags: 0,	
				pNext: nullptr(),
			};

		match vkCreateFramebuffer(*device, &framebuffer_create_info, nullptr(), &mut swapchain_framebuffers[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateFramebuffer()"); }
			err => { panic!("✗ vkCreateFramebuffer() failed with code {:?}.", err); }
		}	
	}

	swapchain_framebuffers
}