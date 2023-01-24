use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::device::*;
use crate::ffi::strings::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_physical_device(vk_handle: &mut VkHandle)
{
	// Picking physical device
	let mut physical_device_count = 0u32;
	vkEnumeratePhysicalDevices(vk_handle.instance, &mut physical_device_count, nullptr());
	if physical_device_count == 0
	{
		panic!("No vk compatible Physical Devices found!");
	}
	println!("{} Physical devices were found", physical_device_count);
	let mut physical_device_vec = vec![ std::mem::zeroed(); physical_device_count as usize ];
	vkEnumeratePhysicalDevices(vk_handle.instance, &mut physical_device_count, physical_device_vec.as_mut_ptr());

	vk_handle.needed_device_extensions = vec![
		"VK_KHR_swapchain",
	];
	if vk_handle.enable_validation_layers
	{
		vk_handle.needed_device_extensions.push("VK_KHR_portability_subset");
	}

	vk_handle.physical_device = pick_best_device(&vk_handle, physical_device_vec).expect("failed to find a suitable GPU!");
	let mut device_properties = std::mem::zeroed();
	vkGetPhysicalDeviceProperties(vk_handle.physical_device, &mut device_properties);
	println!("picked device {}", from_c_string(&device_properties.deviceName).unwrap());
}