#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr::null_mut as nullptr;

mod vulkan;
use vulkan::{
	vk_bindgen::*, c_macros::*, debugger::*, device::*, extension::*, 
	layers::*, handle::VkHandle, queue::*, swapchain::*, shader::*,
	draw::*, framebuffer::*, command_pool::*, command_buffer::*,
	pipeline::*,
};

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
			window_image_format: VkFormat::VK_FORMAT_UNDEFINED,
			surface_format: VkSurfaceFormatKHR { 
				format: VkFormat::VK_FORMAT_UNDEFINED, 
				colorSpace: VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR 
			},
			present_mode: VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR,
			extent: VkExtent2D { width: 0, height: 0 },
			swapchain_framebuffers: vec![],
			render_pass: nullptr(),
			queue_handle: QueueHandle::default(),
			graphics_pipeline: nullptr(),
			pipeline_layout: nullptr(),
			queue_family_indices: vec![],
			graphics_queue: nullptr(),
			presentation_queue: nullptr(),
			command_pool: nullptr(),
			command_buffer_vec: vec![],
			image_available_semaphore_vec: vec![],
			rendering_finished_semaphore_vec: vec![],
			in_flight_fence_vec: vec![],
			swapchain: nullptr(),
			swapchain_image_views_vec: vec![],
			swapchain_support_details: SwapchainSupportDetails {
				capabilities: VkSurfaceCapabilitiesKHR{ ..Default::default() },
				formats: vec![],
				present_modes: vec![]
			},
			frames_in_flight: 3,
			current_frame: 0,
			needed_device_extensions: vec![],
			layer_names: layer_names,
			enable_validation_layers: enable_validation_layers,
			vertex_shader_module: nullptr(),
			fragment_shader_module: nullptr()
		};

		let _window = 
			Window::new()
			.with_title("deta:l")
			.with_dimensions(400, 400)
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
	
		create_logical_device(&mut vk_handle);

		vk_handle.swapchain_support_details = query_swapchain_support(vk_handle.physical_device, vk_handle.window_surface);
		
		create_swapchain(&mut vk_handle);

		create_swapchain_image_views(&mut vk_handle);

		create_pipeline(&mut vk_handle);

		create_framebuffer(&mut vk_handle);

		// Command pool creation
		create_command_pool(&mut vk_handle);

		// Command buffer
		create_command_buffer(&mut vk_handle);

		// creating semaphores & fence
		let semaphore_create_info = VkSemaphoreCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
			flags: 0,	
			pNext: nullptr(),
		};

		let fence_create_info = VkFenceCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
			flags: VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as u32,	
			pNext: nullptr(),
		};

		vk_handle.image_available_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
		vk_handle.rendering_finished_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
		vk_handle.in_flight_fence_vec.resize(vk_handle.frames_in_flight, nullptr());

		for i in 0..vk_handle.frames_in_flight
		{
			match vkCreateSemaphore(vk_handle.logical_device, &semaphore_create_info, nullptr(), &mut vk_handle.image_available_semaphore_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
				err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
			}	
			match vkCreateSemaphore(vk_handle.logical_device, &semaphore_create_info, nullptr(), &mut vk_handle.rendering_finished_semaphore_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
				err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
			}
			match vkCreateFence(vk_handle.logical_device, &fence_create_info, nullptr(), &mut vk_handle.in_flight_fence_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateFence()"); }
				err => { panic!("✗ vkCreateFence() failed with code {:?}.", err); }
			}
		}

		loop 
		{
			draw_frame(&mut vk_handle);
		}
		
		vkDeviceWaitIdle(vk_handle.logical_device);

		std::thread::sleep(std::time::Duration::from_secs(2));

		// Cleanup
		println!("Destroying vk objects...");
		for i in 0..vk_handle.frames_in_flight
		{
			vkDestroyFence(vk_handle.logical_device, vk_handle.in_flight_fence_vec[i], nullptr());
			vkDestroySemaphore(vk_handle.logical_device, vk_handle.rendering_finished_semaphore_vec[i], nullptr());
			vkDestroySemaphore(vk_handle.logical_device, vk_handle.image_available_semaphore_vec[i], nullptr());
		}
		
		vkDestroyCommandPool(vk_handle.logical_device, vk_handle.command_pool, nullptr());
		for framebuffer in vk_handle.swapchain_framebuffers
		{
			vkDestroyFramebuffer(vk_handle.logical_device, framebuffer, nullptr());
		}
		vkDestroyPipelineLayout(vk_handle.logical_device, vk_handle.pipeline_layout, nullptr());
		vkDestroyPipeline(vk_handle.logical_device, vk_handle.graphics_pipeline, nullptr());
		vkDestroyRenderPass(vk_handle.logical_device, vk_handle.render_pass, nullptr());
		vkDestroyShaderModule(vk_handle.logical_device, vk_handle.fragment_shader_module, nullptr());
		vkDestroyShaderModule(vk_handle.logical_device, vk_handle.vertex_shader_module, nullptr());
		for image_view in vk_handle.swapchain_image_views_vec
		{
			vkDestroyImageView(vk_handle.logical_device, image_view, nullptr());
		}
		vkDestroySwapchainKHR(vk_handle.logical_device, vk_handle.swapchain, nullptr());
		vkDestroySurfaceKHR(vk_instance, vk_handle.window_surface, nullptr());
		vkDestroyDevice(vk_handle.logical_device, nullptr());
		destroy_debug_utils_messenger_ext(&vk_instance, &debug_messenger, nullptr());
		vkDestroyInstance(vk_instance, nullptr());
	}
}
