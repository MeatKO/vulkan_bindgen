use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::debugger::*;
use crate::vulkan::c_macros::*;
use crate::vulkan::extension::*;
use crate::vulkan::layers::*;
use crate::ffi::strings::*;
use std::ffi::c_void;

use std::ptr::null_mut as nullptr;

pub unsafe fn create_instance(vk_handle: &mut VkHandle)
{
	let application_info = VkApplicationInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
		pApplicationName: to_c_string("deta:l vulkan"),
		applicationVersion: vk_make_version(1, 0, 0),
		pEngineName: to_c_string("deta:l alpha"),
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
		"VK_LAYER_VALVE_steam_fossilize_64",
		"VK_LAYER_VALVE_steam_overlay_32",
		"VK_LAYER_VALVE_steam_overlay_64",
		"VK_LAYER_INTEL_nullhw",
		"VK_LAYER_MESA_overlay",
	];

	let mut layer_count = 0u32;
	vkEnumerateInstanceLayerProperties(&mut layer_count, nullptr());
	let mut layer_vec = vec![ std::mem::zeroed(); layer_count as usize ];
	vkEnumerateInstanceLayerProperties(&mut layer_count, layer_vec.as_mut_ptr());

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

	let layer_names = 
		layer_vec
		.iter()
		.filter(|layer| 
			!disabled_layers.contains(&layer.get_layer_name())
		)
		.map(|layer|  
			layer.layerName.as_ptr())
		.collect::<Vec<*const i8>>();

	if vk_handle.enable_validation_layers
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
	if vk_handle.enable_validation_layers
	{
		instance_create_info.enabledLayerCount = layer_names.len() as u32;
		instance_create_info.ppEnabledLayerNames = layer_names.as_ptr();
		instance_create_info.pNext = (&mut debug_create_info as *mut VkDebugUtilsMessengerCreateInfoEXT) as *mut c_void;
	}

	match vkCreateInstance(&instance_create_info, nullptr(), &mut vk_handle.instance)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateInstance()"); }
		err => { panic!("vkCreateInstance() failed with code {:?}.", err); }
	}

	// Debug messenger again...
	if vk_handle.enable_validation_layers
	{
		match create_debug_utils_messenger_ext( &vk_handle.instance, &debug_create_info, nullptr(), &mut vk_handle.debug_messenger)
		{
			VkResult::VK_SUCCESS => { println!("✔️ Debugger creation"); }
			err => { panic!("Debugger creation failed with code {:?}.", err); }
		}
	}
}