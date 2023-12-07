use super::material::Material;
use crate::vulkan::uniform_buffer::UniformBufferObject;
use crate::vulkan::vk_bindgen::{
	VkBuffer, 
	VkDeviceMemory,
	VkDescriptorPool, 
	VkDescriptorSet,
};

#[derive(Debug)]
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

#[derive(Default, Debug)]
pub struct Mesh
{
	pub name: String,
	pub material: Material,
	pub index_count: u32,
	pub vulkan_data: Option<VulkanMeshData>,
}
