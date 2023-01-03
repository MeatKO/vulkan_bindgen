use std::ffi::{c_void};
use std::ptr::null_mut as nullptr;
use crate::vulkan::VkResult_VK_SUCCESS;

mod vulkan;

// only takes static string slices because I have trust issues 
// (I don't know how the C code will later use this memory)
unsafe fn c_string(input_str: &'static str) -> *const i8
{
	input_str.as_ptr() as *const i8
}

fn main() 
{
	unsafe
	{
		let enable_validation_layers = true;

		let application_info = vulkan::VkApplicationInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pApplicationName: c_string("deta:l vulkan\0"),
			applicationVersion: vulkan::VK_MAKE_VERSION(1, 0, 0),
			pEngineName: c_string("deta:l alpha\0"),
			engineVersion: vulkan::VK_MAKE_VERSION(1, 0, 0),
			apiVersion: vulkan::VK_MAKE_API_VERSION(0, 1, 2, 0),
			pNext: nullptr()
		};

		// Extensions
		let needed_extensions = vec![
			"VK_EXT_debug_utils",
			"VK_KHR_surface"
		];

		let mut extension_count = 0u32;
		vulkan::vkEnumerateInstanceExtensionProperties(std::ptr::null_mut(), &mut extension_count as _, nullptr());
		let mut extension_vec = vec![ std::mem::zeroed(); extension_count as usize ];
		vulkan::vkEnumerateInstanceExtensionProperties(nullptr(), &mut extension_count as _, extension_vec.as_mut_ptr());
		
		println!("Instance Extensions :");
		println!("\tSupported extension count : {}", extension_count);
		println!("\tSupported extensions : ");
		for elem in &extension_vec 
		{
			println!("\t\t{} - {:?}", elem.get_extension_name(), elem.extensionName.as_ptr());
		}

		let extension_names = extension_vec
			.iter()
			.map(|extension_property|  extension_property.extensionName.as_ptr())
			.collect::<Vec<*const i8>>();

		println!("\nChecking extension availability...");
		vulkan::check_extension_availability(&needed_extensions, &extension_vec);
		println!("success!");

		// Validation layers
		let needed_layers = vec![
			"VK_LAYER_KHRONOS_validation"
		];

		let mut layer_count = 0u32;
		vulkan::vkEnumerateInstanceLayerProperties(&mut layer_count as _, nullptr());
		let mut layer_vec = vec![ std::mem::zeroed(); layer_count as usize ];
		vulkan::vkEnumerateInstanceLayerProperties(&mut layer_count as _, layer_vec.as_mut_ptr());

		println!("\nIntance Layers :");
		println!("\tSupported layer count: {}", layer_count);
		println!("\tSupported layers : ");
		for property in &layer_vec 
		{
			println!("\t\t{} - {:?}", property.get_layer_name(), property.layerName.as_ptr());
		}

		let layer_names = layer_vec
			.iter()
			.filter(|layer|
				// ignore this one because it creates a bunch of snapshots
				layer.get_layer_name() != "VK_LAYER_LUNARG_gfxreconstruct" 
			)
			.map(|layer|  layer.layerName.as_ptr())
			.collect::<Vec<*const i8>>();

		if enable_validation_layers
		{
			println!("\nChecking layer availability...");
			vulkan::check_layer_availability(&needed_layers, &layer_vec);
			println!("success!");
		}

		// Debug messenger
		let mut debug_create_info = vulkan::VkDebugUtilsMessengerCreateInfoEXT{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
			messageSeverity: 
				vulkan::VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT |
				vulkan::VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT |
				vulkan::VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
			messageType: 
				vulkan::VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT |
				vulkan::VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT |
				vulkan::VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
			pfnUserCallback: Some(vulkan::debug_callback),
			pUserData: nullptr(),
			flags: 0,
			pNext: nullptr()
		};

		// Instance
		let instance_create_info = vulkan::VkInstanceCreateInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pApplicationInfo: &application_info,
			enabledExtensionCount: extension_names.len() as u32,
			ppEnabledExtensionNames: extension_names.as_ptr(),
			enabledLayerCount: layer_names.len() as u32,
			ppEnabledLayerNames: layer_names.as_ptr(),
			flags: 0,
			pNext: (&mut debug_create_info as *mut vulkan::VkDebugUtilsMessengerCreateInfoEXT) as *mut c_void
			// pNext: nullptr()
		};

		let mut vulkan_instance = 0 as vulkan::VkInstance;
		let create_info_result = vulkan::vkCreateInstance(&instance_create_info, nullptr(), &mut vulkan_instance);

		println!("\nCreating vulkan instance...");
		if create_info_result != vulkan::VkResult_VK_SUCCESS
		{
			panic!("failed with code {}.", create_info_result);
		}
		println!("success!");

		// Debug messenger again...
		let mut debug_messenger = std::mem::zeroed();

		let create_debugger_result = 
			vulkan::create_debug_utils_messenger_ext(
				&vulkan_instance, 
				&debug_create_info, 
				nullptr(), 
				&mut debug_messenger as _
			);
		
		if create_debugger_result != VkResult_VK_SUCCESS
		{
			panic!("couldn't create debugger. failed with code {}", create_debugger_result);
		}

		// Picking physical device
		let physical_device = vulkan::VK_NULL_HANDLE();

		let mut physical_device_count = 0u32;
		vulkan::vkEnumeratePhysicalDevices(vulkan_instance, &mut physical_device_count, nullptr());
		if physical_device_count == 0
		{
			panic!("No Vulkan compatible Physical Devices found!");
		}
		let mut physical_device_vec = vec![ std::mem::zeroed(); layer_count as usize ];
		vulkan::vkEnumeratePhysicalDevices(vulkan_instance, &mut physical_device_count, physical_device_vec.as_mut_ptr());

		let mut device_properties = std::mem::zeroed();
		for (index, device) in physical_device_vec.iter().cloned().enumerate()
		{
			if device as u32 == 0
			{
				continue;
			}

			vulkan::vkGetPhysicalDeviceProperties(device, &mut device_properties);
			// println!("device [{}] with name {}", index, vulkan::c_string(&device_properties.deviceName));
		}

		// Cleanup
		println!("Destroying instances...");
		vulkan::destroy_debug_utils_messenger_ext(&vulkan_instance, &debug_messenger, nullptr());
		vulkan::vkDestroyInstance(vulkan_instance, nullptr());
	}
}
