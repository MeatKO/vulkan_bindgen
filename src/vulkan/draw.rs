use crate::cotangens::vec3::Vec3;
use crate::detail_core::model::model::Model;
use crate::detail_core::model::model::VulkanModel;
use crate::detail_core::ui::traits::HUDElement;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::command_buffer::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::uniform_buffer::*;
use std::ptr::null_mut as nullptr;

use super::command_buffer_hud::record_command_buffer_hud;
use super::command_buffer_wireframe::record_command_buffer_wireframe;
use super::uniform_buffer_hud::update_uniform_buffer_hud;
use super::uniform_buffer_wireframe::update_uniform_buffer_wireframe;

pub fn 	draw_frame(
	vk_handle: &mut VkHandle, 
	models: &mut Vec<Model<VulkanModel>>,
	light_pos: &Vec3,
	hud_elements: &Vec<Box<dyn HUDElement>>,
)
{
	unsafe
	{
		vkWaitForFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame], VK_TRUE, u64::MAX);
		vkResetFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame]);

		let mut image_index = 0u32;
		match vkAcquireNextImageKHR(vk_handle.logical_device, vk_handle.swapchain, u64::MAX, vk_handle.image_available_semaphore_vec[vk_handle.current_frame], nullptr(), &mut image_index)
		{
			VkResult::VK_SUCCESS => {}
			VkResult::VK_ERROR_OUT_OF_DATE_KHR => { recreate_swapchain(vk_handle); }
			e => { panic!("vkAcquireNextImageKHR() resulted in {:?}", e) }
		}

		for (index, model) in models.iter_mut().enumerate()
		{
			// model.aabb.size = model.scale.clone();

			update_uniform_buffer_wireframe(
				vk_handle, 
				model.aabb_vulkan_data.as_ref().unwrap(), 
				&model.aabb.size,
				// &model.translation, 
				&model.aabb.position, 
				&Vec3::new(0.0f32), 
				&model.aabb.color,
			);

			for mesh in &model.meshes
			{
				let vulkan_data = 
					match &mesh.vulkan_data
					{
						Some(vd) => vd,
						None => continue
					};

				update_uniform_buffer(vk_handle, vulkan_data, index, &model.scale, &model.aabb.position, &model.rotation, light_pos);
			}
		}
		for hud_element in hud_elements.iter()
		{
			let vulkan_data = 
				match hud_element.get_vulkan_data()
				{
					Some(vd) => vd,
					None => continue
				};

			update_uniform_buffer_hud(vk_handle, &vulkan_data);
		}

		// vkResetCommandBuffer(vk_handle.command_buffer_vec[vk_handle.current_frame], 0);
		vk_handle.command_buffer_vec[vk_handle.current_frame].reset(None);

		record_command_buffer(vk_handle, image_index, models);
		record_command_buffer_wireframe(vk_handle, image_index, models);
		record_command_buffer_hud(vk_handle, image_index, hud_elements);

		let wait_semaphore_vec = vec![vk_handle.image_available_semaphore_vec[vk_handle.current_frame]];
		let wait_stages_vec : Vec<VkPipelineStageFlags> = vec![VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32];
		let signal_semaphores_vec = vec![vk_handle.rendering_finished_semaphore_vec[vk_handle.current_frame]];

		let command_buffers = 
			vec![
				vk_handle.command_buffer_vec[vk_handle.current_frame].get_command_buffer_ptr(),
				vk_handle.command_buffer_wireframe_vec[vk_handle.current_frame].get_command_buffer_ptr(),
				vk_handle.command_buffer_hud_vec[vk_handle.current_frame].get_command_buffer_ptr(),
			];

		let submit_info = 
			VkSubmitInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
				waitSemaphoreCount: 1,
				pWaitSemaphores: wait_semaphore_vec.as_ptr(),
				pWaitDstStageMask: wait_stages_vec.as_ptr(),
				commandBufferCount: command_buffers.len() as _,
				pCommandBuffers: command_buffers.as_ptr(),
				signalSemaphoreCount: 1,
				pSignalSemaphores: signal_semaphores_vec.as_ptr(),
				pNext: nullptr(),
			};

		match vkQueueSubmit(vk_handle.graphics_queue, 1, &submit_info, vk_handle.in_flight_fence_vec[vk_handle.current_frame])
		{
			VkResult::VK_SUCCESS => {}
			err => { panic!("✗ vkQueueSubmit() failed with code {:?}.", err); }
		}

		let swapchain_vec = vec![vk_handle.swapchain];

		let present_info = 
			VkPresentInfoKHR{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
				waitSemaphoreCount: 1,
				pWaitSemaphores: signal_semaphores_vec.as_ptr(),
				swapchainCount: 1,
				pSwapchains: swapchain_vec.as_ptr(),
				pImageIndices: &image_index,
				pResults: nullptr(),
				pNext: nullptr()
			};


		match vkDeviceWaitIdle(vk_handle.logical_device)
		{
			VkResult::VK_SUCCESS => {} 
			// VkResult::VK_ERROR_DEVICE_LOST => 
			_ => { panic!("sheeeit") }
		}
		
		// let start_time = std::time::Instant::now();

		match vkQueuePresentKHR(vk_handle.presentation_queue, &present_info)
		{
			VkResult::VK_SUCCESS => {}
			VkResult::VK_ERROR_OUT_OF_DATE_KHR => { println!("vkQueuePresentKHR() out of date - recreating"); recreate_swapchain(vk_handle) }
			e => { panic!("vkQueuePresentKHR() resulted in {:?}", e) }
		}

		// println!("vkQueuePresentKHR() time : {:?}ms", std::time::Instant::now().duration_since(start_time).as_secs_f64() * 1000.0f64);

		vk_handle.current_frame = (vk_handle.current_frame + 1) % vk_handle.frames_in_flight;
	}
}