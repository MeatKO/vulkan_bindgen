use crate::cotangens::{
	vec2::*,
	vec3::*,
};
use crate::vulkan::uniform_buffer::UniformBufferObject;
use crate::vulkan::vertex::*;
use crate::vulkan::handle::{
	VkHandle,
};
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

use std::path::Path;
use std::ptr::null_mut as nullptr;
use std::time;

use super::mesh::Mesh;
use super::obj_loader::{ModelLoadError, load_obj};

pub struct Model
{
	pub name: String,
	pub meshes: Vec<Mesh>
}

impl Model
{
	pub fn load<P>(model_path: P) -> Result<Model, ModelLoadError>
	where P : AsRef<Path>
	{
		match model_path.as_ref().extension().ok_or(ModelLoadError::UnsupportedFileType)?.to_str()
		{
			Some("obj") => load_obj(model_path),
			_ => Err(ModelLoadError::UnsupportedFileType),
		}
	}

	pub fn destroy(&self, vk_handle: &mut VkHandle)
	{
		// unsafe
		// {
		// 	vkDestroyBuffer(vk_handle.logical_device, self.vertex_buffer, nullptr());
		// 	vkFreeMemory(vk_handle.logical_device, self.vertex_buffer_memory, nullptr());
		
		// 	vkDestroyBuffer(vk_handle.logical_device, self.index_buffer, nullptr());
		// 	vkFreeMemory(vk_handle.logical_device, self.index_buffer_memory, nullptr());

		// 	vkDestroySampler(vk_handle.logical_device, self.texture_sampler, nullptr());
		// 	vkDestroyImageView(vk_handle.logical_device, self.texture_image_view, nullptr());
			
		// 	vkDestroyImage(vk_handle.logical_device, self.texture_image, nullptr());
		// 	vkFreeMemory(vk_handle.logical_device, self.texture_image_memory, nullptr());

		// 	for i in 0..vk_handle.frames_in_flight
		// 	{
		// 		vkDestroyBuffer(vk_handle.logical_device, self.uniform_buffers[i], nullptr());
		// 		vkFreeMemory(vk_handle.logical_device, self.uniform_buffers_memory[i], nullptr());
		// 	}

		// 	// vkDestroyDescriptorPool(vk_handle.logical_device, self.descriptor_pool, nullptr());
		// 	// vkDestroyDescriptorSetLayout(vk_handle.logical_device, vk_handle.descriptor_set_layout, nullptr());
		// }
	}

	pub fn process_meshes(&mut self, vk_handle: &mut VkHandle)
	{
		for mesh in &self.meshes
		{
			
		}
	}
}