use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::command_buffer::*;
use std::ptr::null_mut as nullptr;

pub fn draw_frame(vk_handle: &VkHandle)
{
	println!("drawing...");
	unsafe
	{
		vkWaitForFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence, VK_TRUE, u64::MAX);

		let mut image_index = 0u32;
		vkAcquireNextImageKHR(vk_handle.logical_device, vk_handle.swapchain, u64::MAX, vk_handle.image_available_semaphore, nullptr(), &mut image_index);

		vkResetCommandBuffer(vk_handle.command_buffer, 0);
		record_command_buffer(vk_handle, image_index);

		let wait_semaphore_vec = vec![vk_handle.image_available_semaphore];
		let wait_stages_vec : Vec<VkPipelineStageFlags> = vec![VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32];
		let signal_semaphores_vec = vec![vk_handle.rendering_finished_semaphore];

		let submit_info = VkSubmitInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
			waitSemaphoreCount: 1,
			pWaitSemaphores: wait_semaphore_vec.as_ptr(),
			pWaitDstStageMask: wait_stages_vec.as_ptr(),
			commandBufferCount: 1,
			pCommandBuffers: &vk_handle.command_buffer,
			signalSemaphoreCount: 1,
			pSignalSemaphores: signal_semaphores_vec.as_ptr(),
			pNext: nullptr()	
		};

		match vkQueueSubmit( vk_handle.graphics_queue, 1, &submit_info, vk_handle.in_flight_fence)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkQueueSubmit()"); }
			err => { panic!("✗ vkQueueSubmit() failed with code {:?}.", err); }
		}

		let swapchain_vec = vec![vk_handle.swapchain];

		let present_info = VkPresentInfoKHR{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
			waitSemaphoreCount: 1,
			pWaitSemaphores: signal_semaphores_vec.as_ptr(),
			swapchainCount: 1,
			pSwapchains: swapchain_vec.as_ptr(),
			pImageIndices: &image_index,
			pResults: nullptr(),
			pNext: nullptr()
		};

		vkQueuePresentKHR(vk_handle.presentation_queue, &present_info);

		vkResetFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence);
	}
}