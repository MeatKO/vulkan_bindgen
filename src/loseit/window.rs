#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::handle::*;
use crate::loseit::window_traits::*;
use crate::loseit::xcb_window::*;
use crate::loseit::xcb_events;
use crate::loseit::win32_window::*;
use crate::loseit::window_events::WindowEvent;

use std::ptr::null_mut as nullptr;

use super::window_events::KeyValues;

enum WindowHandle
{
	Xcb(XcbHandle),
	Win32(Win32Handle)
}

pub struct Window
{
	window_handle: Box<Option<WindowHandle>>,
	window_title: Option<String>,
	pub width: u32,
	pub height: u32,
	pub is_focused: bool,
}

impl Window
{
	pub fn new() -> Self
	{
		return Window { 
			window_handle: Box::new(None),
			window_title: None,
			width: 150,
			height: 150,
			is_focused: true
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

		self.window_handle = Box::new(Some(WindowHandle::Xcb(xcb_handle)));

		self
	}

	pub fn get_event(&self) -> Option<WindowEvent> 
	{
		// I need to get better at Rust, this clone doesn't make much sense tbh
		// I shouldn't have to clone everything
		let handle = &self.window_handle;
		
		match handle.as_ref()
		{
			Some(WindowHandle::Xcb(handle)) => { handle.get_event() }
			_ => { None }
		}
	}

	pub fn convert_key_code(&self, key_code: u8) -> KeyValues
	{
		let handle = &self.window_handle;
		
		match handle.as_ref()
		{
			Some(WindowHandle::Xcb(handle)) => { xcb_events::convert_key_code(key_code) }
			_ => { KeyValues::UNKNOWN }
			// _ => { KeyValues::UNKNOWN(key_code) }
		}
	}

	pub fn lock_pointer(&self)
	{
		let handle = &self.window_handle;

		match handle.as_ref()
		{
			Some(WindowHandle::Xcb(handle)) => { handle.lock_pointer() }
			_ => { }
		}
	}

	pub fn unlock_pointer(&self)
	{
		let handle = &self.window_handle;

		match handle.as_ref()
		{
			Some(WindowHandle::Xcb(handle)) => { handle.unlock_pointer() }
			_ => { }
		}
	}

	pub fn center_pointer(&self)
	{
		let handle = &self.window_handle;

		match handle.as_ref()
		{
			Some(WindowHandle::Xcb(handle)) => { handle.center_pointer() }
			_ => { }
		}
	}

	pub fn hide_cursor(&self)
	{
		let handle = &self.window_handle;

		match handle.as_ref()
		{
			Some(WindowHandle::Xcb(handle)) => { handle.hide_cursor() }
			_ => { }
		}
	}
}