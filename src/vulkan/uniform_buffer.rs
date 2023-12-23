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
#[derive(Debug, Clone)]
pub struct UniformBufferObject
{
	pub light_pos: Vec3,
	pub view_pos: Vec3,
	pub model: Mat4x4,
	pub view: Mat4x4,
	pub proj: Mat4x4,
}

impl UniformBufferObject
{
	pub fn new_empty() -> Self
	{
		UniformBufferObject{
			light_pos: Vec3::new(0.0f32),
			view_pos: Vec3::new(0.0f32),
			model: Mat4x4::new_identity(1.0f32),
			view: Mat4x4::new_identity(1.0f32),
			proj: Mat4x4::new_identity(1.0f32),
		}
	}
}

pub unsafe fn create_uniform_buffers<T>(
	vk_handle: &VkHandle,
	count: usize,
) -> Result<(Vec<VkBuffer>, Vec<VkDeviceMemory>, Vec<*mut T>), String>
{
	let buffer_size = size_of::<T>() as u64;

	let mut out_uniform_buffers = vec![nullptr(); count];
	let mut out_uniform_buffers_memory = vec![nullptr(); count];
	let mut out_uniform_buffers_mapped = vec![nullptr(); count];

	for i in 0..out_uniform_buffers.len()
	{
		let (buffer, buffer_memory) = 
			create_buffer(
				vk_handle, 
				buffer_size,
				VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32, 
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 | 
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
			)?;
		
		out_uniform_buffers[i] = buffer;
		out_uniform_buffers_memory[i] = buffer_memory;

		// persistent mapping
		let mut uniform_buffer_map : *mut c_void = nullptr();
		vkMapMemory(
			vk_handle.logical_device, 
			out_uniform_buffers_memory[i], 
			0, 
			buffer_size, 
			0, 
			&mut uniform_buffer_map, 
		);
		out_uniform_buffers_mapped[i] = uniform_buffer_map as _;
	}

	Ok((out_uniform_buffers, out_uniform_buffers_memory, out_uniform_buffers_mapped))
}

pub unsafe fn update_uniform_buffer(
	vk_handle: &VkHandle,
	// mesh_data: &VulkanMeshData,
	// ubo_vec: &Vec<UniformBufferObject>,
	ubo_mapped: *mut UniformBufferObject,
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
	
	// std::ptr::copy_nonoverlapping(&ubo, ubo_mapped as _, 1);
	std::ptr::copy_nonoverlapping(&ubo, ubo_mapped, 1);
}	