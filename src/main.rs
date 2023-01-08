use std::ffi::{c_void};
use std::ptr::null_mut as nullptr;

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
			if needed_extensions.contains(&elem.get_extension_name())
			{
				println!("\t\t\x1B[92m{} - {:?}\x1B[0m", elem.get_extension_name(), elem.extensionName.as_ptr());
			}
			else
			{
				println!("\t\t{} - {:?}", elem.get_extension_name(), elem.extensionName.as_ptr());
			}
		}

		let extension_names = extension_vec
			.iter()
			.map(|extension_property|  extension_property.extensionName.as_ptr())
			.collect::<Vec<*const i8>>();

		vulkan::check_extension_availability(&needed_extensions, &extension_vec);
		println!("✔️ Required extensions are available"); 

		// Validation layers
		let needed_layers = vec![
			"VK_LAYER_KHRONOS_validation"
		];

		let disabled_layers = vec![
			"VK_LAYER_LUNARG_gfxreconstruct",
			"VK_LAYER_LUNARG_api_dump",
			"VK_LAYER_LUNARG_device_simulation",
			"VK_LAYER_VALVE_steam_fossilize_64",
            "VK_LAYER_VALVE_steam_fossilize_32"
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
			if disabled_layers.contains(&property.get_layer_name())
			{
				println!("\t\t\x1B[31m{} - {:?}\x1B[0m", property.get_layer_name(), property.layerName.as_ptr());
			}
			else
			{
				println!("\t\t\x1B[36m{} - {:?}\x1B[0m", property.get_layer_name(), property.layerName.as_ptr());
			}
		}

		let layer_names = layer_vec
			.iter()
			.filter(|layer| 
					!disabled_layers.contains(&layer.get_layer_name())
			)
			.map(|layer|  layer.layerName.as_ptr())
			.collect::<Vec<*const i8>>();

		if enable_validation_layers
		{
			vulkan::check_layer_availability(&needed_layers, &layer_vec);
			println!("✔️ Required layers are available"); 
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
		};

		let mut vulkan_instance = std::mem::zeroed();
		match vulkan::vkCreateInstance(&instance_create_info, nullptr(), &mut vulkan_instance)
		{
			vulkan::VkResult_VK_SUCCESS => { println!("✔️ vkCreateInstance()"); }
			err => { panic!("vkCreateInstance() failed with code {}.", err); }
		}

		// Debug messenger again...
		let mut debug_messenger = std::mem::zeroed();
		match vulkan::create_debug_utils_messenger_ext( &vulkan_instance, &debug_create_info, nullptr(), &mut debug_messenger as _)
		{
			vulkan::VkResult_VK_SUCCESS => { println!("✔️ Debugger creation"); }
			err => { panic!("Debugger creation failed with code {}.", err); }
		}

		// Picking physical device
		let mut physical_device_count = 0u32;
		vulkan::vkEnumeratePhysicalDevices(vulkan_instance, &mut physical_device_count, nullptr());
		if physical_device_count == 0
		{
			panic!("No Vulkan compatible Physical Devices found!");
		}
		let mut physical_device_vec = vec![ std::mem::zeroed(); layer_count as usize ];
		vulkan::vkEnumeratePhysicalDevices(vulkan_instance, &mut physical_device_count, physical_device_vec.as_mut_ptr());

		let mut physical_device = nullptr();
		for device in physical_device_vec.iter().cloned()
		{
			if vulkan::is_device_suitable(device)
			{
				physical_device = device;
				break;
			}
		}
		match physical_device.is_null()
		{
			true => { panic!("Failed to find a suitable GPU!"); }
			false => { println!("Using device {:?}", physical_device); }
		}

		// Queue creation
		let queue_flags = vulkan::get_physical_device_queue_flags(physical_device).expect("no supported queues found!");
		let queue_priorities = 1.0f32;

		let queue_create_info = vulkan::VkDeviceQueueCreateInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
			queueFamilyIndex: queue_flags & vulkan::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT,
			queueCount: 1,	
			pQueuePriorities: &queue_priorities,
			flags: 0,
			pNext: nullptr()
		};

		// Device creation
		let device_features : vulkan::VkPhysicalDeviceFeatures = std::mem::zeroed(); // essentially putting everything to VkFalse

		let mut device_create_info = vulkan::VkDeviceCreateInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
			pQueueCreateInfos: &queue_create_info,
			queueCreateInfoCount: 1,
			pEnabledFeatures: &device_features,
			enabledExtensionCount: 0,
			ppEnabledExtensionNames: nullptr(),
			enabledLayerCount: 0,
			ppEnabledLayerNames: nullptr(),
			flags: 0,
			pNext: nullptr()
		};
		if enable_validation_layers
		{
			device_create_info.enabledLayerCount = layer_names.len() as u32;
			device_create_info.ppEnabledLayerNames = layer_names.as_ptr();
		}

		let mut vulkan_device = std::mem::zeroed();
		match vulkan::vkCreateDevice(physical_device, &device_create_info, nullptr(), &mut vulkan_device)
		{
			vulkan::VkResult_VK_SUCCESS => { println!("✔️ vkCreateDevice()"); }
			err => { panic!("✗ vkCreateDevice() failed with code {}.", err); }
		}

		// Queue handling
		let mut graphics_queue : vulkan::VkQueue = nullptr();
		let _device_queue_handle = 
			vulkan::vkGetDeviceQueue(
				vulkan_device, 
				vulkan::VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT, 
				0, 
				&mut graphics_queue
			);

		
		
		// Cleanup
		println!("Destroying vulkan objects...");
		vulkan::vkDestroyDevice(vulkan_device, nullptr());
		vulkan::destroy_debug_utils_messenger_ext(&vulkan_instance, &debug_messenger, nullptr());
		vulkan::vkDestroyInstance(vulkan_instance, nullptr());
	}
}
