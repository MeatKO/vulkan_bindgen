use crate::vulkan::vk_bindgen::*;
use crate::vulkan::extension::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

#[derive(Default)]
pub struct QueueFamilyIndices
{
	pub presentation_family: Option<u32>,
	pub graphics_family: Option<u32>
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

pub unsafe fn is_device_suitable(vk_handle: &VkHandle, physical_device: VkPhysicalDevice, required_extensions: &Vec<&str>) -> bool
{
	let mut extension_count = 0u32;
	vkEnumerateDeviceExtensionProperties(physical_device, nullptr(), &mut extension_count, nullptr());
	let mut extension_vec = vec![ std::mem::zeroed(); extension_count as usize ];
	vkEnumerateDeviceExtensionProperties(physical_device, nullptr(), &mut extension_count, extension_vec.as_mut_ptr());

	match get_missing_extensions(&required_extensions, &extension_vec)
	{
		Some(_) => { return false }
		_ => {}
	}

	let swapchain_support_details = query_swapchain_support(vk_handle);

	match get_physical_device_queue_flags(physical_device)
	{
		None => { return false }
		Some(flags) => { return (flags & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0 }
	}
}

pub unsafe fn pick_best_device(vk_handle: &VkHandle, physical_devices: Vec<*mut VkPhysicalDevice_T>, required_extensions: &Vec<&str>) -> Option<VkPhysicalDevice>
{
	println!("physical device count : {}", physical_devices.len());

	let mut suitable_devices_vec = physical_devices
		.iter()
		.copied()
		.filter(
			|physical_device|
			is_device_suitable(vk_handle, *physical_device, required_extensions)
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