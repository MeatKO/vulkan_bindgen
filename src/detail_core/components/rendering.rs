use decs::component_derive::component;
use decs::component::Component;

use std::fmt::Debug;
use std::rc::Weak;

use crate::{detail_core::model::model::{Model, VulkanModel}, vulkan::{uniform_buffer::{UniformBufferObject, create_uniform_buffers}, vk_bindgen::{VkDeviceMemory, VkBuffer}, handle::VkHandle}};

#[component]
pub struct ModelComponent
{
	pub model_asset: Weak<Model<VulkanModel>>
}

#[component]
pub struct UniformBufferComponent
{
	// pub ubo: Vec<UniformBufferObject>
	pub uniform_buffers: Vec<VkBuffer>,
	pub uniform_buffers_memory: Vec<VkDeviceMemory>,
	pub uniform_buffers_mapped: Vec<*mut UniformBufferObject>,
}

impl UniformBufferComponent
{
	pub fn new(vk_handle: &VkHandle) -> Result<Self, String>
	{
		let uniform_buffers = unsafe{ create_uniform_buffers(&vk_handle, vk_handle.frames_in_flight)? };
		Ok(UniformBufferComponent{
			uniform_buffers: uniform_buffers.0,
			uniform_buffers_memory: uniform_buffers.1,
			uniform_buffers_mapped: uniform_buffers.2, 
		})
	}
}