use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::VkHandle;
use std::ptr::null_mut as nullptr;

pub struct QueueDescriptor
{
	pub family_index: u32,
	pub queue_index: u32
}

pub struct QueueHandle
{
	pub presentation_queue: Option<QueueDescriptor>,
	pub graphics_queue: Option<QueueDescriptor>,
	pub compute_queue: Option<QueueDescriptor>,
	pub transfer_queue: Option<QueueDescriptor>,

	presentation_support: bool,

	family_mask: u32
}

impl QueueHandle
{
	pub fn new() -> Self
	{
		return QueueHandle{
			presentation_queue: None,
			graphics_queue: None,
			compute_queue: None,
			transfer_queue: None,
			presentation_support: false,
			family_mask: 0u32
		};
	}

	pub fn with_presentation_support(mut self) -> Self
	{
		self.presentation_support = true;
		self
	}
	pub fn with_graphics_support(mut self) -> Self
	{
		self.family_mask |= VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32;
		self
	}

	pub fn build(mut self, vk_handle: &VkHandle) -> Option<Self>
	{
		if self.family_mask == 0u32
		{
			panic!("Cannot build QueueHandle with no family flags.\nConsider requesting a queue bit support.\nEx : with_graphics_support().");
		}

		let queue_family_vec = 
			get_physical_device_queue_families(vk_handle)?
			.iter()
			.copied()	
			.filter( 
				|family_properties|
				(family_properties.queueFlags & self.family_mask) > 0
			)
			.collect::<Vec<_>>();

		if queue_family_vec.is_empty()
		{
			return None
		}

		// actually there was no need for the family variable because we filter it above...
		// judge a man not by the quality of his code but by the silly jokes in his comments
		for (family_index, _family) in queue_family_vec.iter().enumerate()
		{
			if self.presentation_support
			{
				let mut present_support = VK_FALSE;
				unsafe 
				{
					vkGetPhysicalDeviceSurfaceSupportKHR(
						vk_handle.physical_device, 
						family_index as u32, 
						vk_handle.window_surface, 
						&mut present_support
					);
				}
				
				if present_support == VK_TRUE
				{
					self.graphics_queue = Some(
						QueueDescriptor { 
							family_index: family_index as u32, 
							queue_index: 0
						}
					);
					self.presentation_queue = Some(
						QueueDescriptor { 
							family_index: family_index as u32, 
							queue_index: 0
						}
					);
					return Some(self)
				}
			}
			else 
			{
				self.graphics_queue = Some(
					QueueDescriptor { 
						family_index: family_index as u32, 
						queue_index: 0
					}
				);
				return Some(self)
			}
		}

		None
	}
}

fn get_physical_device_queue_families(vk_handle: &VkHandle) -> Option<Vec<VkQueueFamilyProperties>>
{
	unsafe
	{
		let mut queue_family_count = 0u32;
		vkGetPhysicalDeviceQueueFamilyProperties(vk_handle.physical_device, &mut queue_family_count, nullptr());
		let mut queue_family_vec = vec![ std::mem::zeroed(); queue_family_count as usize ];
		vkGetPhysicalDeviceQueueFamilyProperties(vk_handle.physical_device, &mut queue_family_count, queue_family_vec.as_mut_ptr());

		if queue_family_vec.is_empty()
		{
			return None
		}
		Some(queue_family_vec)
	}
}