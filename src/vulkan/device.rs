#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;
use std::ptr::null_mut as nullptr;

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

	let out_queue_bitset = 0u32;
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
		Some(flags) => { return flags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT != 0 }
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
			if a.1 == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU 
			&& b.1 == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU
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