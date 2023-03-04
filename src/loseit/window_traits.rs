#![allow(drop_bounds)]

use crate::vulkan::handle::VkHandle;
use crate::loseit::window_events::WindowEvent;

pub trait VulkanWindowHandle : Sized + Drop
{
	fn new(window_title: &Option<String>, width: u32, height: u32, vk_handle: &mut VkHandle) -> Option<Self>;
	fn get_event(&self) -> Option<WindowEvent>;
	fn destroy(&mut self);
}