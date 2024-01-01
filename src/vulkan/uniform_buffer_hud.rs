// use crate::cotangens::mat4x4::Mat4x4;
// use crate::detail_core::ui::button::VulkanButtonData;
// use crate::vulkan::vk_bindgen::*;
// use crate::vulkan::handle::*;
// use crate::vulkan::vk_buffer::*;
// use std::ffi::c_void;
// use std::mem::size_of;
// use std::ptr::null_mut as nullptr;


// // don't remember if this should be commented out : EDIT - its *not* needed because inner structs have 16B alignment
// #[repr(C)]
// #[repr(align(16))]
// pub struct UniformBufferObjectHUD
// {
// 	// pub model: Mat4x4,
// 	// pub view: Mat4x4,
// 	pub proj: Mat4x4,
// }

// pub unsafe fn create_uniform_buffers_hud(
// 	vk_handle: &VkHandle,
// 	hud_element_data: &mut VulkanButtonData,
// ) 
// {
// 	let buffer_size = size_of::<UniformBufferObjectHUD>() as u64;
// 	// let buffer_size = size_of::<UniformBufferObject>() as u64;

// 	hud_element_data.uniform_buffers.resize(vk_handle.frames_in_flight, nullptr());
// 	hud_element_data.uniform_buffers_memory.resize(vk_handle.frames_in_flight, nullptr());
// 	hud_element_data.uniform_buffers_mapped.resize(vk_handle.frames_in_flight, nullptr());

// 	for i in 0..hud_element_data.uniform_buffers.len()
// 	{
// 		let (buffer, buffer_memory) = 
// 			match create_buffer(
// 				vk_handle, 
// 				buffer_size,
// 				VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32, 
// 				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 | 
// 				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
// 			)
// 			{
// 				Ok(tuple) => { tuple }
// 				Err(e) => { panic!("create_uniform_buffers, create_buffer failed {}", e) }
// 			};
		
// 		hud_element_data.uniform_buffers[i] = buffer;
// 		hud_element_data.uniform_buffers_memory[i] = buffer_memory;

// 		// persistent mapping
// 		let mut uniform_buffer_map : *mut c_void = nullptr();
// 		vkMapMemory(
// 			vk_handle.logical_device, 
// 			hud_element_data.uniform_buffers_memory[i], 
// 			0, 
// 			buffer_size, 
// 			0, 
// 			&mut uniform_buffer_map, 
// 		);
// 		hud_element_data.uniform_buffers_mapped[i] = uniform_buffer_map as _;
// 	}
// }

// pub unsafe fn update_uniform_buffer_hud(
// 	vk_handle: &VkHandle,
// 	hud_element_data: &VulkanButtonData,
// ) 
// {
// 	let ubo = 
// 	UniformBufferObjectHUD{
// 			proj: Mat4x4::new_orthographic(0.0f32, vk_handle.swapchain_extent.width as f32, 0.0f32, vk_handle.swapchain_extent.height as f32, -1.0f32, 1.0f32),
// 		};
	
// 	std::ptr::copy_nonoverlapping(&ubo, hud_element_data.uniform_buffers_mapped[vk_handle.current_frame] as _, 1);
// }	