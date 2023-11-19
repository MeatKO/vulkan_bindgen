use crate::cotangens::mat4x4::Mat4x4;
use crate::cotangens::vec3::Vec3;
use crate::detail_core::model::mesh::VulkanMeshData;
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
	pub light_pos: Vec3,
	pub view_pos: Vec3,
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
	mesh_data: &VulkanMeshData,
	index: usize,
	scale: &Vec3,
	translation: &Vec3,
	rotation: &Vec3,
	light_pos: &Vec3,
) 
{
	// let time: f32 = std::time::Instant::now().duration_since(vk_handle.start_time).as_secs_f32();

	let camera_position = vk_handle.camera.get_position();

	let ubo = 
		UniformBufferObject{
			// light_pos: camera_position.clone(),
			// light_pos: Vec3 { x: 5.0f32, y: 5.0f32, z: 5.0f32 },
			light_pos: light_pos.clone(),
			view_pos: camera_position,
			model: Mat4x4::new_identity(1.0f32)
				.scale(scale.clone())
				.rotate_x(rotation.x)
				.rotate_y(rotation.y)
				// .rotate_z(rotation.z)
				.translate(translation.clone()),
			view: vk_handle.camera.get_view_matrix(),
			proj: Mat4x4::new_perspective(
				45.0f32.to_radians(), 
				vk_handle.swapchain_extent.width as f32 / vk_handle.swapchain_extent.height as f32, 
				0.1f32, 
				1000.0f32
			)
		};
	
	std::ptr::copy_nonoverlapping(&ubo, mesh_data.uniform_buffers_mapped[vk_handle.current_frame] as _, 1);
}	