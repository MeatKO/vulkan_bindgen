use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::command_buffer::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::uniform_buffer::*;
use std::ptr::null_mut as nullptr;

pub fn draw_frame(vk_handle: &mut VkHandle)
{
	unsafe
	{
		vkWaitForFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame], VK_TRUE, u64::MAX);
		
		let mut image_index = 0u32;
		match vkAcquireNextImageKHR(vk_handle.logical_device, vk_handle.swapchain, u64::MAX, vk_handle.image_available_semaphore_vec[vk_handle.current_frame], nullptr(), &mut image_index)
		{
			VkResult::VK_SUCCESS => {}
			VkResult::VK_ERROR_OUT_OF_DATE_KHR => { recreate_swapchain(vk_handle); return; }
			e => { panic!("vkAcquireNextImageKHR() resulted in {:?}", e) }
		}

		update_uniform_buffer(vk_handle);

		vkResetFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame]);

		vkResetCommandBuffer(vk_handle.command_buffer_vec[vk_handle.current_frame], 0);
		record_command_buffer(vk_handle, image_index);

		let wait_semaphore_vec = vec![vk_handle.image_available_semaphore_vec[vk_handle.current_frame]];
		let wait_stages_vec : Vec<VkPipelineStageFlags> = vec![VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32];
		let signal_semaphores_vec = vec![vk_handle.rendering_finished_semaphore_vec[vk_handle.current_frame]];

		let submit_info = VkSubmitInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
			waitSemaphoreCount: 1,
			pWaitSemaphores: wait_semaphore_vec.as_ptr(),
			pWaitDstStageMask: wait_stages_vec.as_ptr(),
			commandBufferCount: 1,
			pCommandBuffers: &vk_handle.command_buffer_vec[vk_handle.current_frame],
			signalSemaphoreCount: 1,
			pSignalSemaphores: signal_semaphores_vec.as_ptr(),
			pNext: nullptr()	
		};

		match vkQueueSubmit( vk_handle.graphics_queue, 1, &submit_info, vk_handle.in_flight_fence_vec[vk_handle.current_frame])
		{
			// VkResult::VK_SUCCESS => { println!("✔️ vkQueueSubmit()"); }
			VkResult::VK_SUCCESS => { }
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

		match vkQueuePresentKHR(vk_handle.presentation_queue, &present_info)
		{
			// VkResult::VK_SUCCESS => { println!("✔️ vkQueuePresentKHR()"); }
			VkResult::VK_SUCCESS => {  }
			VkResult::VK_ERROR_OUT_OF_DATE_KHR => { println!("vkQueuePresentKHR() out of date - recreating"); recreate_swapchain(vk_handle) }
			e => { panic!("vkQueuePresentKHR() resulted in {:?}", e) }
		}

		vk_handle.current_frame = (vk_handle.current_frame + 1) % vk_handle.frames_in_flight;
	}
}