#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr::null_mut as nullptr;

mod vulkan;
use vulkan::{vk_bindgen::*, c_macros::*, debugger::*, device::*, extension::*, layers::*, handle::VkHandle, queue::*};

mod loseit;
use loseit::window::*;

mod ffi;
use ffi::strings::*;

fn main() 
{
	unsafe
	{
		let enable_validation_layers = true;

		let application_info = VkApplicationInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
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
			"VK_KHR_surface",
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

		let extension_names = 
			extension_vec
			.iter()
			.map(|extension_property|  extension_property.extensionName.as_ptr())
			.collect::<Vec<*const i8>>();

		match get_missing_extensions(&needed_extensions, &extension_vec)
		{
			Some(missing_extensions) => { panic!("Missing required extensions : {:?}", missing_extensions) }
			_ => {}
		}

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
			sType: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
			messageSeverity: 
				VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT as u32 |
				VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT as u32 |
				VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT as u32 ,
			messageType: 
				VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT as u32 |
				VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT as u32 |
				VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT as u32 ,
			pfnUserCallback: Some(debug_callback),
			pUserData: nullptr(),
			flags: 0,
			pNext: nullptr()
		};

		// Instance
		let mut instance_create_info = VkInstanceCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
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
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateInstance()"); }
			err => { panic!("vkCreateInstance() failed with code {:?}.", err); }
		}

		// Debug messenger again...
		let mut debug_messenger = std::mem::zeroed();

		if enable_validation_layers
		{
			match create_debug_utils_messenger_ext( &vk_instance, &debug_create_info, nullptr(), &mut debug_messenger as _)
			{
				VkResult::VK_SUCCESS => { println!("✔️ Debugger creation"); }
				err => { panic!("Debugger creation failed with code {:?}.", err); }
			}
		}

		// Creating a window
		let mut vk_handle = VkHandle {
			instance: vk_instance,
			physical_device: nullptr(),
			logical_device: nullptr(),
			available_extensions: extension_vec,
			window_surface: nullptr(),
			window_image_format: VkFormat::VK_FORMAT_UNDEFINED
		};

		let _window = 
			Window::new()
			.with_title("deta:l")
			.with_dimensions(150, 150)
			.build_vulkan(&mut vk_handle);

		println!("Window built successfully.");

		// Picking physical device
		let mut physical_device_count = 0u32;
		vkEnumeratePhysicalDevices(vk_instance, &mut physical_device_count, nullptr());
		if physical_device_count == 0
		{
			panic!("No vk compatible Physical Devices found!");
		}
		println!("{} Physical devices were found", physical_device_count);
		let mut physical_device_vec = vec![ std::mem::zeroed(); physical_device_count as usize ];
		vkEnumeratePhysicalDevices(vk_instance, &mut physical_device_count, physical_device_vec.as_mut_ptr());

		let needed_extensions = vec![
			"VK_KHR_swapchain"
		];

		// a stupid hack, fix later !
		let needed_extensions_c = 
			needed_extensions
			.iter()
			.map(|extension|  
				to_c_string(extension)
			)
			.collect::<Vec<*const i8>>();

		let physical_device = pick_best_device(&vk_handle, physical_device_vec, &needed_extensions).expect("failed to find a suitable GPU!");

		vk_handle.physical_device = physical_device;

		let mut device_properties = std::mem::zeroed();
		vkGetPhysicalDeviceProperties(physical_device, &mut device_properties);

		println!("picked device {}", from_c_string(&device_properties.deviceName).unwrap());
			
		// Queue creation
		let queue_handle = 
			QueueHandle::new()
			.with_graphics_support()
			.with_presentation_support()
			.build(&vk_handle)
			.expect("No suitable queues found.");

		let queue_priorities = 1.0f32;

		let queue_create_info = VkDeviceQueueCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
			queueFamilyIndex: queue_handle.graphics_queue.as_ref().unwrap().family_index,
			queueCount: 1,	
			pQueuePriorities: &queue_priorities,
			flags: 0,
			pNext: nullptr()
		};

		// Device creation
		let device_features : VkPhysicalDeviceFeatures = std::mem::zeroed(); // essentially putting everything to VkFalse

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
		if enable_validation_layers
		{
			device_create_info.enabledLayerCount = layer_names.len() as u32;
			device_create_info.ppEnabledLayerNames = layer_names.as_ptr();
		}

		vk_handle.logical_device = std::mem::zeroed();
		match vkCreateDevice(physical_device, &device_create_info, nullptr(), &mut vk_handle.logical_device)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateDevice()"); }
			err => { panic!("✗ vkCreateDevice() failed with code {:?}.", err); }
		}

		// Get VkQueue objects after the Logical Device creation
		// These queues must be created with create infos first!
		// Check device creation above
		let mut graphics_queue = std::mem::zeroed();
		let mut presentation_queue = std::mem::zeroed();
		vkGetDeviceQueue( 
			vk_handle.logical_device, 
			queue_handle.graphics_queue.as_ref().unwrap().family_index, 
			queue_handle.graphics_queue.as_ref().unwrap().queue_index, 
			&mut graphics_queue
		);
		vkGetDeviceQueue( 
			vk_handle.logical_device, 
			queue_handle.presentation_queue.as_ref().unwrap().family_index, 
			queue_handle.presentation_queue.as_ref().unwrap().queue_index, 
			&mut presentation_queue
		);

		std::thread::sleep(std::time::Duration::from_secs(2));

		// Cleanup
		println!("Destroying vk objects...");
		vkDestroySurfaceKHR(vk_instance, vk_handle.window_surface, nullptr());
		vkDestroyDevice(vk_handle.logical_device, nullptr());
		destroy_debug_utils_messenger_ext(&vk_instance, &debug_messenger, nullptr());
		vkDestroyInstance(vk_instance, nullptr());
	}
}
