use super::material::RenderingMaterial;
use crate::vulkan::vertex::*;
use crate::vulkan::uniform_buffer::UniformBufferObject;
use crate::vulkan::vk_bindgen::{
	VkBuffer, 
	VkDeviceMemory,
	vkDestroyBuffer,
	vkFreeMemory,
	VkImage,
	VkSampler,
	VkImageView, 
	VkDescriptorPool, 
	VkDescriptorSet, 
	vkDestroySampler, 
	vkDestroyImageView, 
	vkDestroyImage,
	vkDestroyDescriptorPool, 
	vkDestroyDescriptorSetLayout,
};

use std::ptr::null_mut as nullptr;
use std::time;

pub struct VulkanMeshData
{
	pub vertex_buffer: VkBuffer,
	pub vertex_buffer_memory: VkDeviceMemory,

	pub index_buffer: VkBuffer,
	pub index_buffer_memory: VkDeviceMemory,

	pub uniform_buffers: Vec<VkBuffer>,
	pub uniform_buffers_memory: Vec<VkDeviceMemory>,
	pub uniform_buffers_mapped: Vec<*mut UniformBufferObject>,

	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_sets: Vec<VkDescriptorSet>,
}

#[derive(Default)]
pub struct Mesh
{
	pub name: String,

	pub material: RenderingMaterial,

	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,

	pub index_count: u32,

	pub vulkan_data: Option<VulkanMeshData>,
}

impl Mesh
{
	pub fn new(mesh_name: String) -> Mesh
	{
		Mesh { 
			name: mesh_name,
			..Default::default()
		}
	}
}