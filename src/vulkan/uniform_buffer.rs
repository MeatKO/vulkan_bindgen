use crate::cotangens::mat4x4::Mat4x4;
use crate::cotangens::vec2::Vec2;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::buffer::*;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;

#[repr(C)]
// #[repr(align(16))]
pub struct UniformBufferObject
{
	pub foo: Vec2,
	pub model: Mat4x4,
	pub view: Mat4x4,
	pub proj: Mat4x4,
}

pub unsafe fn create_uniform_buffers(vk_handle: &mut VkHandle) 
{
	let buffer_size = size_of::<UniformBufferObject>() as u64;

	vk_handle.uniform_buffers.resize(vk_handle.frames_in_flight, nullptr());
	vk_handle.uniform_buffers_memory.resize(vk_handle.frames_in_flight, nullptr());
	vk_handle.uniform_buffers_mapped.resize(vk_handle.frames_in_flight, nullptr());

	for i in 0..vk_handle.frames_in_flight
	{
		let (buffer, buffer_memory) = 
			match create_buffer(
				vk_handle, 
				buffer_size,
				VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32, 
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 | 
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
			)
			{
				Ok(tuple) => { tuple }
				Err(e) => { panic!("create_uniform_buffers, create_buffer failed {}", e) }
			};
		
		vk_handle.uniform_buffers[i] = buffer;
		vk_handle.uniform_buffers_memory[i] = buffer_memory;

		// persistent mapping
		let mut uniform_buffer_map : *mut c_void = nullptr();
		vkMapMemory(
			vk_handle.logical_device, 
			vk_handle.uniform_buffers_memory[i], 
			0, 
			buffer_size, 
			0, 
			&mut uniform_buffer_map, 
		);
		vk_handle.uniform_buffers_mapped[i] = uniform_buffer_map as _;
	}
}

pub unsafe fn update_uniform_buffer(vk_handle: &mut VkHandle) 
{
	let time: f32 = std::time::Instant::now().duration_since(vk_handle.start_time).as_secs_f32();
	// let time: f32 = 0.0f32;

	let time = time / 2.0f32;

	let ubo = 
		UniformBufferObject{
			foo: Vec2 { x: 0.0f32, y: 0.0f32 },
			model: Mat4x4::new_identity(1.0f32)
				.rotate_x(time * 90.0f32.to_radians())
				.rotate_y(time * 90.0f32.to_radians())
				.rotate_z(time * 90.0f32.to_radians()),
			view: vk_handle.camera.get_view_matrix(),
			proj: Mat4x4::new_perspective(
				45.0f32.to_radians(), 
				vk_handle.swapchain_extent.width as f32 / vk_handle.swapchain_extent.height as f32, 
				0.1f32, 
				100.0f32
			)
		};
	
	std::ptr::copy_nonoverlapping(&ubo, vk_handle.uniform_buffers_mapped[vk_handle.current_frame] as _, 1);
}	