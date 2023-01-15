#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr::null_mut as nullptr;

mod vulkan;
use vulkan::{vk_bindgen::*, c_macros::*, debugger::*, device::*, extension::*, layers::*, handle::VkHandle};

mod loseit;
use loseit::window::*;

mod ffi;
use ffi::strings::*;

// mod smth;

fn main() 
{
	unsafe
	{
		let enable_validation_layers = true;

		let application_info = VkApplicationInfo{
			sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pApplicationName: to_c_string("deta:l vulkan\0"),
			applicationVersion: vk_make_version(1, 0, 0),
			pEngineName: to_c_string("deta:l alpha\0"),
			engineVersion: vk_make_version(1, 0, 0),
			apiVersion: vk_make_api_version(0, 1, 2, 0),
			pNext: nullptr()
		};

		// Extensions
		let needed_extensions = vec![
			"VK_EXT_debug_utils",
			"VK_KHR_surface"
		];

		let mut extension_count = 0u32;
		vkEnumerateInstanceExtensionProperties(nullptr(), &mut extension_count as _, nullptr());
		let mut extension_vec = vec![ std::mem::zeroed(); extension_count as usize ];
		vkEnumerateInstanceExtensionProperties(nullptr(), &mut extension_count as _, extension_vec.as_mut_ptr());
		
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

		check_extension_availability(&needed_extensions, &extension_vec);
		println!("✔️ Required extensions are available"); 

		// Validation layers
		let needed_layers = vec![
			"VK_LAYER_KHRONOS_validation"
		];

		let disabled_layers = vec![
			"VK_LAYER_LUNARG_gfxreconstruct",
			"VK_LAYER_LUNARG_api_dump",
			"VK_LAYER_LUNARG_device_simulation",
			"VK_LAYER_VALVE_steam_fossilize",
			"VK_LAYER_VALVE_steam_fossilize_32",
            "VK_LAYER_VALVE_steam_fossilize_64"
		];

		let mut layer_count = 0u32;
		vkEnumerateInstanceLayerProperties(&mut layer_count as _, nullptr());
		let mut layer_vec = vec![ std::mem::zeroed(); layer_count as usize ];
		vkEnumerateInstanceLayerProperties(&mut layer_count as _, layer_vec.as_mut_ptr());

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
			check_layer_availability(&needed_layers, &layer_vec);
			println!("✔️ Required layers are available"); 
		}

		// Debug messenger
		let mut debug_create_info = VkDebugUtilsMessengerCreateInfoEXT{
			sType: VkStructureType_VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
			messageSeverity: 
				VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT |
				VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT |
				VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
			messageType: 
				VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT |
				VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT |
				VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
			pfnUserCallback: Some(debug_callback),
			pUserData: nullptr(),
			flags: 0,
			pNext: nullptr()
		};

		// Instance
		let mut instance_create_info = VkInstanceCreateInfo{
			sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pApplicationInfo: &application_info,
			enabledExtensionCount: extension_names.len() as u32,
			ppEnabledExtensionNames: extension_names.as_ptr(),
			enabledLayerCount: 0,
			ppEnabledLayerNames: nullptr(),
			flags: 0,
			pNext: nullptr()
		};

		if enable_validation_layers
		{
			instance_create_info.enabledLayerCount = layer_names.len() as u32;
			instance_create_info.ppEnabledLayerNames = layer_names.as_ptr();
			instance_create_info.pNext = (&mut debug_create_info as *mut VkDebugUtilsMessengerCreateInfoEXT) as *mut c_void;
		}

		let mut vk_instance = std::mem::zeroed();
		match vkCreateInstance(&instance_create_info, nullptr(), &mut vk_instance)
		{
			VkResult_VK_SUCCESS => { println!("✔️ vkCreateInstance()"); }
			err => { panic!("vkCreateInstance() failed with code {}.", err); }
		}

		// Debug messenger again...
		let mut debug_messenger = std::mem::zeroed();

		if enable_validation_layers
		{
			match create_debug_utils_messenger_ext( &vk_instance, &debug_create_info, nullptr(), &mut debug_messenger as _)
			{
				VkResult_VK_SUCCESS => { println!("✔️ Debugger creation"); }
				err => { panic!("Debugger creation failed with code {}.", err); }
			}
		}

		// Picking physical device
		let mut physical_device_count = 0u32;
		vkEnumeratePhysicalDevices(vk_instance, &mut physical_device_count, nullptr());
		if physical_device_count == 0
		{
			panic!("No vk compatible Physical Devices found!");
		}
		let mut physical_device_vec = vec![ std::mem::zeroed(); layer_count as usize ];
		vkEnumeratePhysicalDevices(vk_instance, &mut physical_device_count, physical_device_vec.as_mut_ptr());

		let physical_device = pick_best_device(physical_device_vec).expect("Failed to find a suitable GPU!");

		
		// Queue creation
		let queue_flags = get_physical_device_queue_flags(physical_device).expect("no supported queues found!");
		let queue_priorities = 1.0f32;

		let queue_create_info = VkDeviceQueueCreateInfo{
			sType: VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
			queueFamilyIndex: queue_flags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT,
			queueCount: 1,	
			pQueuePriorities: &queue_priorities,
			flags: 0,
			pNext: nullptr()
		};

		// Device creation
		let device_features : VkPhysicalDeviceFeatures = std::mem::zeroed(); // essentially putting everything to VkFalse

		let mut device_create_info = VkDeviceCreateInfo{
			sType: VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
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

		let mut vk_device = std::mem::zeroed();
		match vkCreateDevice(physical_device, &device_create_info, nullptr(), &mut vk_device)
		{
			VkResult_VK_SUCCESS => { println!("✔️ vkCreateDevice()"); }
			err => { panic!("✗ vkCreateDevice() failed with code {}.", err); }
		}

		// Queue handling
		let mut graphics_queue : VkQueue = nullptr();
		let _device_queue_handle = 
			vkGetDeviceQueue(
				vk_device, 
				VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT, 
				0, 
				&mut graphics_queue
			);

		let mut vk_handle = VkHandle {
			instance: vk_instance,
			physical_device: physical_device,
			available_extensions: extension_vec,
			window_surface: nullptr(),
			window_image_format: 0
		};
		
		let window = 
			Window::new()
			.with_title("deta:l")
			.with_dimensions(150, 150)
			.build_vulkan(&mut vk_handle);
		
		std::thread::sleep(std::time::Duration::from_secs(10));

		// Cleanup
		println!("Destroying vk objects...");
		vkDestroySurfaceKHR(vk_instance, vk_handle.window_surface, nullptr());
		vkDestroyDevice(vk_device, nullptr());
		destroy_debug_utils_messenger_ext(&vk_instance, &debug_messenger, nullptr());
		vkDestroyInstance(vk_instance, nullptr());
	}
}
