#![allow(drop_bounds)]

use crate::vulkan::handle::VkHandle;
use crate::loseit::window_events::WindowEvent;

pub trait VulkanWindowHandle : Sized + Drop
{
	fn new(window_title: &Option<String>, width: u32, height: u32, vk_handle: &mut VkHandle) -> Option<Self>;
	fn lock_pointer(&self);
	fn center_pointer(&self);
	fn unlock_pointer(&self);
	fn hide_cursor(&self);
	fn show_cursor(&self);
	fn get_event(&self) -> Option<WindowEvent>;
	fn destroy(&mut self);
}