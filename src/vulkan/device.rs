use crate::vulkan::vk_bindgen::*;
use std::ptr::null_mut as nullptr;

use super::handle::VkHandle;

#[derive(Default)]
pub struct QueueFamilyIndices
{
	pub presentation_family: Option<u32>,
	pub graphics_family: Option<u32>
}

pub unsafe fn get_physical_device_queue_family_indices(vk_handle: &VkHandle) -> QueueFamilyIndices
{
	let mut indices_out = QueueFamilyIndices{..Default::default()};

	let mut queue_family_count = 0u32;
	vkGetPhysicalDeviceQueueFamilyProperties(vk_handle.physical_device, &mut queue_family_count, nullptr());
	let mut queue_family_vec = vec![ std::mem::zeroed(); queue_family_count as usize ];
	vkGetPhysicalDeviceQueueFamilyProperties(vk_handle.physical_device, &mut queue_family_count, queue_family_vec.as_mut_ptr());

	for (i, queue_family) in queue_family_vec.iter().enumerate()
	{
		let mut present_support = VK_FALSE;
		vkGetPhysicalDeviceSurfaceSupportKHR(vk_handle.physical_device, i as u32, vk_handle.window_surface, &mut present_support);

		println!("queue [{}] flags : 0b{:08b} count - {} - presentation support : {}", i, queue_family.queueFlags, queue_family.queueCount,present_support)

		// if (present_support > 0) && ((queue_family.queueFlags & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) > 0)
		// {
		// 	indices_out.graphics_family = Some(i as u32);
		// 	indices_out.presentation_family = Some(i as u32);
		// }
	}

	indices_out
}

pub unsafe fn get_physical_device_queue_flags(physical_device: VkPhysicalDevice) -> Option<u32>
{
	if physical_device as u32 == 0
	{
		return None;
	}

	let mut queue_family_count = 0u32;
	vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut queue_family_count, nullptr());
	let mut queue_family_vec = vec![ std::mem::zeroed(); queue_family_count as usize ];
	vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut queue_family_count, queue_family_vec.as_mut_ptr());

	let queue_flags = 
			queue_family_vec
			.iter()
			.map(|queue| queue.queueFlags)
			.reduce(std::ops::BitOr::bitor);
			
	queue_flags
}

pub unsafe fn is_device_suitable(physical_device: VkPhysicalDevice) -> bool
{
	match get_physical_device_queue_flags(physical_device)
	{
		None => { return false }
		Some(flags) => { return (flags & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0 }
	}
}

pub unsafe fn pick_best_device(physical_devices: Vec<*mut VkPhysicalDevice_T>) -> Option<VkPhysicalDevice>
{
	let mut suitable_devices_vec = physical_devices
		.iter()
		.copied()
		.filter(
			|physical_device|
			is_device_suitable(*physical_device)
		)
		.map(
			|physical_device|
			{
				let mut device_properties = std::mem::zeroed();
				vkGetPhysicalDeviceProperties(physical_device, &mut device_properties);

				let mut device_memory_properties = std::mem::zeroed();
				vkGetPhysicalDeviceMemoryProperties(physical_device, &mut device_memory_properties);

				(physical_device, device_properties.deviceType, device_memory_properties.memoryHeaps[0].size)
			}
		)
		.collect::<Vec<_>>();

	if suitable_devices_vec.len() == 0
	{
		return None
	}
	
	suitable_devices_vec
	.sort_by(
		|a, b|
		{
			if a.1 == VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU 
			&& b.1 == VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU
			{
				return std::cmp::Ordering::Greater;
			}
			if a.2 > b.2
			{
				return std::cmp::Ordering::Greater;
			}
			return std::cmp::Ordering::Less;
		}
	);
	
	return Some(suitable_devices_vec.last().unwrap().0);
}