use crate::cotangens::mat4x4::Mat4x4;
use crate::cotangens::vec2::Vec2;
use crate::cotangens::vec3::Vec3;
use crate::exedra::mesh::VulkanMeshData;
use crate::exedra::model::Model;
// use crate::detail_core::camera::*;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::vk_buffer::*;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;

// don't remember if this should be commented out : EDIT - its *not* needed because inner structs have 16B alignment
#[repr(C)]
#[repr(align(16))]
pub struct UniformBufferObject
{
	pub foo: Vec2,
	pub model: Mat4x4,
	pub view: Mat4x4,
	pub proj: Mat4x4,
}

pub unsafe fn create_uniform_buffers(
	vk_handle: &VkHandle,
	mesh_data: &mut VulkanMeshData
) 
{
	let buffer_size = size_of::<UniformBufferObject>() as u64;

	mesh_data.uniform_buffers.resize(vk_handle.frames_in_flight, nullptr());
	mesh_data.uniform_buffers_memory.resize(vk_handle.frames_in_flight, nullptr());
	mesh_data.uniform_buffers_mapped.resize(vk_handle.frames_in_flight, nullptr());

	for i in 0..mesh_data.uniform_buffers.len()
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
		
		mesh_data.uniform_buffers[i] = buffer;
		mesh_data.uniform_buffers_memory[i] = buffer_memory;

		// persistent mapping
		let mut uniform_buffer_map : *mut c_void = nullptr();
		vkMapMemory(
			vk_handle.logical_device, 
			mesh_data.uniform_buffers_memory[i], 
			0, 
			buffer_size, 
			0, 
			&mut uniform_buffer_map, 
		);
		mesh_data.uniform_buffers_mapped[i] = uniform_buffer_map as _;
	}
}

pub unsafe fn update_uniform_buffer(
	vk_handle: &VkHandle,
	// models: &mut Vec<Model>
	mesh_data: &mut VulkanMeshData,
	index: usize
) 
{
	// for (index, model) in models.iter_mut().enumerate()
	// {
		let time: f32 = std::time::Instant::now().duration_since(vk_handle.start_time).as_secs_f32();
		// let time: f32 = 0.0f32;
	
		let time = time / 2.0f32;
	
		let ubo = 
			UniformBufferObject{
				foo: Vec2 { x: 0.0f32, y: 0.0f32 },
				model: Mat4x4::new_identity(1.0f32)
					// .scale(Vec3 { x: 0.1f32, y: 0.1f32, z: 0.1f32 }),
					// .scale(Vec3 { x: 0.001f32, y: 0.001f32, z: 0.001f32 }),
					.scale(Vec3::new(0.00001f32)),
					// .translate(Vec3 { x: 0.0f32, y: -0.5f32, z: 0.0f32 })
					// .translate(Vec3 { x: 5.0f32 * index as f32, y: -0.5f32, z: 0.0f32 })
					// .rotate_x(-90.0f32.to_radians())
					// .rotate_y(90.0f32.to_radians()),
					// .rotate_z(-90.0f32.to_radians()),
	
					// .rotate_x(time * 90.0f32.to_radians())
					// .rotate_y(time * 90.0f32.to_radians())
					// .rotate_z(time * 90.0f32.to_radians()),
				view: vk_handle.camera.get_view_matrix(),
				// view: Mat4x4::new_lookat(vk_handle.camera.position.clone(), Vec3::new(0.0f32), Vec3 { x: 0.0f32, y: 1.0f32, z: 0.0f32 }),
				proj: Mat4x4::new_perspective(
					45.0f32.to_radians(), 
					vk_handle.swapchain_extent.width as f32 / vk_handle.swapchain_extent.height as f32, 
					0.1f32, 
					1000.0f32
				)
			};
		
		std::ptr::copy_nonoverlapping(&ubo, mesh_data.uniform_buffers_mapped[vk_handle.current_frame] as _, 1);
	// }
}	