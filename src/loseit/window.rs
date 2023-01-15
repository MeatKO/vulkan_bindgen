#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;
use crate::loseit::window_traits::*;
use crate::loseit::xcb_window::*;
use crate::loseit::win32_window::*;

enum WindowHandle
{
	Xcb(XcbHandle),
	Win32(Win32Handle)
}

pub struct Window
{
	window_handle: Option<WindowHandle>
}

impl Window
{
	pub fn new() -> Self
	{
		return Window { 
			window_handle: None 
		}
	}

	pub fn build(&mut self) -> &Self
	{
		let xcb_handle = 
			match XcbHandle::new(None, 150, 150)
			{
				Some(handle) => { handle },
				None => { panic!("couldn't initialize xcb") }
			};

		self.window_handle = Some(WindowHandle::Xcb(xcb_handle));

		self
	}
}