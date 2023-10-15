use crate::cotangens::{
	vec2::*,
	vec3::*,
};
use crate::vulkan::descriptor_pool::create_descriptor_pool;
use crate::vulkan::descriptor_set::create_descriptor_sets;
use crate::vulkan::index::create_index_buffer;
use crate::vulkan::texture::create_texture_image;
use crate::vulkan::texture_view::{create_texture_image_view, create_texture_sampler};
use crate::vulkan::uniform_buffer::{UniformBufferObject, create_uniform_buffers};
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

use super::material::VulkanMaterialData;
use super::mesh::{Mesh, VulkanMeshData};
use super::obj_loader::{ModelLoadError, load_obj};

pub struct Model
{
	pub name: String,
	pub meshes: Vec<Mesh>,
	pub scale: Vec3,
	pub translation: Vec3,
	pub rotation: Vec3,
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

	pub unsafe fn process_meshes(&mut self, vk_handle: &mut VkHandle)
	{
		for mesh in &mut self.meshes
		{
			let (texture_image, texture_image_memory) = create_texture_image(&vk_handle, mesh.material.diffuse_map_rel_path.clone());
				let texture_image_view = create_texture_image_view(&vk_handle, &texture_image);
				let texture_sampler = create_texture_sampler(&vk_handle).unwrap();
				mesh.material.vulkan_data = Some(
					VulkanMaterialData{
						texture_image: texture_image,
						texture_image_memory: texture_image_memory,
						texture_image_view: texture_image_view,
						texture_sampler: texture_sampler,
					}
				);

				let (vertex_buffer, vertex_buffer_memory) =
					create_vertex_buffer(&vk_handle, &mut mesh.vertices)
					.unwrap();
		
				let (index_buffer, index_buffer_memory) =
					create_index_buffer(&vk_handle, &mut mesh.indices)
					.unwrap();

				mesh.vulkan_data = Some(
					VulkanMeshData{
						vertex_buffer: vertex_buffer,
						vertex_buffer_memory: vertex_buffer_memory,
						index_buffer: index_buffer,
						index_buffer_memory: index_buffer_memory,
						uniform_buffers: vec![],
						uniform_buffers_memory: vec![],
						uniform_buffers_mapped: vec![],
						descriptor_pool: nullptr(),
						descriptor_sets: vec![],
					}
				);

				let vulkan_mesh_data = mesh.vulkan_data.as_mut().unwrap();
				let vulkan_material_data = mesh.material.vulkan_data.as_mut().unwrap();

				create_uniform_buffers(&vk_handle, vulkan_mesh_data);

				let mut descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				create_descriptor_sets(&vk_handle, vulkan_mesh_data, vulkan_material_data, &descriptor_pool);
				vulkan_mesh_data.descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
		}
	}
}