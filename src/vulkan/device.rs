use crate::vulkan::vk_bindgen::*;
use crate::vulkan::extension::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::handle::*;
use crate::vulkan::queue::*;
use crate::ffi::strings::*;
use std::ptr::null_mut as nullptr;

#[derive(Default)]
pub struct QueueFamilyIndices
{
	pub presentation_family: Option<u32>,
	pub graphics_family: Option<u32>
}

pub unsafe fn create_logical_device(vk_handle: &mut VkHandle)
{
	// a stupid hack, fix later !
	let needed_extensions_c = 
		vk_handle.needed_device_extensions.iter()
		.map(|extension|  
			to_c_string(extension)
		)
		.collect::<Vec<*const i8>>();

	vk_handle.queue_handle = 
			QueueHandle::new()
			.with_graphics_support()
			.with_presentation_support()
			.build(&vk_handle)
			.expect("No suitable queues found.");

	let queue_priorities = 1.0f32;
	let queue_create_info = VkDeviceQueueCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
		queueFamilyIndex: vk_handle.queue_handle.graphics_queue.as_ref().unwrap().family_index,
		queueCount: 1,	
		pQueuePriorities: &queue_priorities,
		flags: 0,
		pNext: nullptr()
	};

	// Device creation
	let mut device_features : VkPhysicalDeviceFeatures = std::mem::zeroed(); // essentially putting everything to VkFalse
	device_features.samplerAnisotropy = VK_TRUE;

	let mut device_create_info = VkDeviceCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
		pQueueCreateInfos: &queue_create_info,
		queueCreateInfoCount: 1,
		pEnabledFeatures: &device_features,
		enabledExtensionCount: needed_extensions_c.len() as u32,
		ppEnabledExtensionNames: needed_extensions_c.as_ptr(),
		enabledLayerCount: 0,
		ppEnabledLayerNames: nullptr(),
		flags: 0,
		pNext: nullptr()
	};
	if vk_handle.enable_validation_layers
	{
		device_create_info.enabledLayerCount = vk_handle.layer_names.len() as u32;
		device_create_info.ppEnabledLayerNames = vk_handle.layer_names.as_ptr();
	}

	match vkCreateDevice(vk_handle.physical_device, &device_create_info, nullptr(), &mut vk_handle.logical_device)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateDevice()"); }
		err => { panic!("✗ vkCreateDevice() failed with code {:?}.", err); }
	}

	// Get VkQueue objects after the Logical Device creation
	// These queues must be created with create infos first!
	// Check device creation above
	vkGetDeviceQueue( 
		vk_handle.logical_device, 
		vk_handle.queue_handle.graphics_queue.as_ref().unwrap().family_index, 
		vk_handle.queue_handle.graphics_queue.as_ref().unwrap().queue_index, 
		&mut vk_handle.graphics_queue
	);
	vkGetDeviceQueue( 
		vk_handle.logical_device, 
		vk_handle.queue_handle.presentation_queue.as_ref().unwrap().family_index, 
		vk_handle.queue_handle.presentation_queue.as_ref().unwrap().queue_index, 
		&mut vk_handle.presentation_queue
	);
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
	let mut device_properties = std::mem::zeroed();
	vkGetPhysicalDeviceProperties(physical_device, &mut device_properties);
	println!("checking if {} is compatible", from_c_string(&device_properties.deviceName).unwrap());

	let mut extension_count = 0u32;
	vkEnumerateDeviceExtensionProperties(physical_device, nullptr(), &mut extension_count, nullptr());
	let mut extension_vec = vec![ std::mem::zeroed(); extension_count as usize ];
	vkEnumerateDeviceExtensionProperties(physical_device, nullptr(), &mut extension_count, extension_vec.as_mut_ptr());

	// optionally we can print the missing extensions here...
	match get_missing_extensions(&required_extensions, &extension_vec)
	{
		Some(missing_extensions) =>
		{
			println!("incompatible due to missing extensions {:?}", missing_extensions);
			return false
		}
		None => {}
	}

	// if get_missing_extensions(&required_extensions, &extension_vec).is_some()
	// {
	// 	return false;
	// }

	let swapchain_support_details = query_swapchain_support(physical_device, vk_handle.window_surface);

	if !(swapchain_support_details.formats.len() > 0) || !(swapchain_support_details.present_modes.len() > 0)
	{
		println!("incompatible due to lack of swapchain support details");
		return false
	}

	let mut physical_device_features: VkPhysicalDeviceFeatures = std::mem::zeroed();
	vkGetPhysicalDeviceFeatures(physical_device, &mut physical_device_features);

	match get_physical_device_queue_flags(physical_device)
	{
		None => 
		{ 
			println!("incompatible due to missing physical queue flags");
			return false 
		}
		Some(flags) => 
		{ 
			// println!("physical queue flags are available, checking for graphics bit");
			// println!("available flags are {:#04b}", flags);
			// println!("required are {:#04b}", VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32);
			return (flags & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0 &&
					physical_device_features.samplerAnisotropy != 0
					
			// return (flags & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0
		}
	}
}

pub unsafe fn pick_best_device(vk_handle: &VkHandle, physical_devices: Vec<*mut VkPhysicalDevice_T>) -> Option<VkPhysicalDevice>
{
	let mut suitable_devices_vec = 
		physical_devices
		.iter()
		.copied()
		.filter(
			|physical_device|
			is_device_suitable(vk_handle, *physical_device, &vk_handle.needed_device_extensions)
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
		.collect::<Vec<(VkPhysicalDevice, VkPhysicalDeviceType, u64)>>();

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
	
	return Some(suitable_devices_vec.last().expect("couldn't pick a device, the suitable_devices_vec was empty.").0);
}