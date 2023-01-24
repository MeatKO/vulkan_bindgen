#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::handle::*;
use crate::loseit::window_traits::*;
use crate::loseit::xcb_window::*;
use crate::loseit::win32_window::*;

use std::ptr::null_mut as nullptr;

enum WindowHandle
{
	Xcb(XcbHandle),
	Win32(Win32Handle)
}

pub struct Window
{
	window_handle: Option<WindowHandle>,
	window_title: Option<String>,
	pub width: u32,
	height: u32
}

impl Window
{
	pub fn new() -> Self
	{
		return Window { 
			window_handle: None,
			window_title: None,
			width: 150,
			height: 150
		}
	}

	pub fn with_title<T>(mut self, title: T) -> Self
	where T: ToString
	{
		self.window_title = Some(title.to_string());
		self
	}

	pub fn with_dimensions(mut self, width: u32, height: u32) -> Self
	{
		self.width = width;
		self.height = height;
		self
	}

	pub fn build_vulkan(mut self, vk_handle: &mut VkHandle) -> Self
	{
		if vk_handle.instance == nullptr()
		{
			panic!("Window requires a valid VkInstance pointer. Consider moving the Window creation after Instance creation.")
		}

		let xcb_handle = 
			match XcbHandle::new(&self.window_title, self.width, self.height, vk_handle)
			{
				Some(handle) => { handle },
				None => { panic!("couldn't initialize xcb") }
			};

		self.window_handle = Some(WindowHandle::Xcb(xcb_handle));

		self
	}
}