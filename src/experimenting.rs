use std::{cmp::min, ptr::null_mut as nullptr, thread::sleep, time::Duration};

use crate::{detail_core::window::create_vulkan_surface, ffi::strings::{from_c_string, from_c_string_ptr, to_c_string}, main, vulkan::{c_macros::{vk_make_api_version, vk_make_version}, command_pool, physical_device, pipeline::create_pipeline, swapchain::{choose_swap_extent, choose_swap_present_mode, choose_swap_surface_format, query_swapchain_support}, texture_view::create_image_view, vertex::Vertex, vk_bindgen::{vkCreateDevice, vkCreateSwapchainKHR, vkEnumeratePhysicalDevices, vkGetDeviceQueue, vkGetPhysicalDeviceMemoryProperties, vkGetPhysicalDeviceProperties, vkGetPhysicalDeviceQueueFamilyProperties, vkGetSwapchainImagesKHR, PFN_vkCmdSetLogicOpEnableEXT, VkApplicationInfo, VkCommandPoolCreateFlagBits, VkCompositeAlphaFlagBitsKHR, VkDescriptorType, VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkImage, VkImageAspectFlagBits, VkImageUsageFlagBits, VkImageView, VkInstance, VkInstanceCreateFlagBits, VkInstanceCreateInfo, VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties, VkPhysicalDeviceType, VkPolygonMode, VkQueueFlagBits, VkSharingMode, VkStructureType, VkSwapchainCreateInfoKHR, VK_TRUE}, wrappers::{vk_command_pool::CommandPoolBuilder, vk_descriptor_layout::VkDescriptorLayoutBuilder, vk_descriptor_pool::VkDescriptorPoolBuilder}}};

use crate::vulkan::shader::create_shader_module;
use crate::vulkan::vk_bindgen::vkCreateInstance;
use crate::vulkan::vk_bindgen::VkResult;

pub struct QueueFamilies
{
	graphics : u32,
	compute : u32,
	presentation: u32,
}

pub unsafe fn init_everything()
{
	// instance
	// physical device - capabilities, query queues, extensions VK_KHR_surface
	// logical device - create queues, extensions VK_KHR_swapchain
	// window - window surface
	// swapchain - format, present mode, extent, 

	let vk_application_info = 
		VkApplicationInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pApplicationName: to_c_string("deta:l vulkan"),
			applicationVersion: vk_make_version(1, 0, 0),
			pEngineName: to_c_string("deta:l alpha"),
			engineVersion: vk_make_version(1, 0, 0),
			apiVersion: vk_make_api_version(0, 1, 2, 0),
			pNext: nullptr()
		};

	let extensions = 
			vec![
				"VK_KHR_surface\0".as_ptr(),
				"VK_KHR_xcb_surface\0".as_ptr(),
			];

	let vk_instance_create_info = 
		VkInstanceCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pNext: nullptr(),
			flags: 0u32,
			pApplicationInfo: &vk_application_info,
			enabledLayerCount: 0,
			ppEnabledLayerNames: nullptr(),
			enabledExtensionCount: extensions.len() as u32,
			ppEnabledExtensionNames: extensions.as_ptr() as _,
		};

	let mut instance = nullptr();
	match vkCreateInstance(&vk_instance_create_info, nullptr(), &mut instance)
	{
		VkResult::VK_SUCCESS => { println!("vkCreateInstance() OK") }
		err => { panic!("vkCreateInstance() failed with code {:?}.", err); }
	}

	let picked_device = pick_best_device(instance).unwrap();

	let mut device_properties = std::mem::zeroed::<VkPhysicalDeviceProperties>();
	vkGetPhysicalDeviceProperties(picked_device, &mut device_properties);

	println!("Picked device '{}'", from_c_string_ptr(device_properties.deviceName.as_ptr()).unwrap());

	let queue_families = get_queue_family_indices(&picked_device).expect("device doesn't have graphics &| compute queues");

	println!("queue families :\nGraphics - {}\nCompute - {}", queue_families.graphics, queue_families.compute);

	let priority = 1.0f32;

	let queue_create_infos = 
		{
			if queue_families.graphics == queue_families.compute
			{
				vec![
					VkDeviceQueueCreateInfo{
						sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
						pNext: nullptr(),
						flags: 0u32,
						queueFamilyIndex: queue_families.graphics,
						queueCount: 2,
						pQueuePriorities: &priority,
					},
				]
			}
			else 
			{
				vec![
					VkDeviceQueueCreateInfo{
						sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
						pNext: nullptr(),
						flags: 0u32,
						queueFamilyIndex: queue_families.graphics,
						queueCount: 1,
						pQueuePriorities: &priority,
					},
					VkDeviceQueueCreateInfo{
						sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
						pNext: nullptr(),
						flags: 0u32,
						queueFamilyIndex: queue_families.compute,
						queueCount: 1,
						pQueuePriorities: &priority,
					}
				]
			}
		};

	let mut device_features : VkPhysicalDeviceFeatures = std::mem::zeroed(); // essentially putting everything to VkFalse
	device_features.samplerAnisotropy = VK_TRUE;

	let needed_device_extensions = 
			vec![
				"VK_KHR_swapchain\0".as_ptr()
			];

	let device_create_info = 
		VkDeviceCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
			pNext: nullptr(),
			flags: 0u32,
			queueCreateInfoCount: queue_create_infos.len() as _,
			pQueueCreateInfos: queue_create_infos.as_ptr(),
			enabledLayerCount: 0,
			ppEnabledLayerNames: nullptr(),
			enabledExtensionCount: needed_device_extensions.len() as _,
			ppEnabledExtensionNames: needed_device_extensions.as_ptr() as _,
			pEnabledFeatures: &device_features,
		};

	let mut device = nullptr(); 
	vkCreateDevice(picked_device, &device_create_info, nullptr(), &mut device).unwrap("vkCreateDevice");

	let mut graphics_queue = nullptr();
	vkGetDeviceQueue(device, queue_families.graphics, 0, &mut graphics_queue);
	// this is potentially bullshit, write a better presentation queue finder later
	let mut presentation_queue = nullptr();
	vkGetDeviceQueue(device, queue_families.presentation, 0, &mut presentation_queue);

	// gotta have a window before the swapchain bs

	let window = 
		parmack::window::WindowBuilder::new()
		.with_title("experimental")
		.with_dimensions(800, 600)
		.build()
		.unwrap();

	// sleep(Duration::from_secs_f32(2.0f32));

	let surface = create_vulkan_surface(&window, &instance).expect("uhhhh couldn't make a surface ?? ?");

	// swapchain creation

	let queue_family_indices = 
		vec![
			queue_families.graphics,
			queue_families.presentation,
		];

	let swapchain_support_details = query_swapchain_support(picked_device, surface);

	let surface_format = choose_swap_surface_format(&swapchain_support_details.formats).expect("Couldn't find suitable window surface format.");
	let present_mode = choose_swap_present_mode(&swapchain_support_details.present_modes);
	let swapchain_extent = choose_swap_extent(&swapchain_support_details.capabilities);

	let image_count =
		min(
			swapchain_support_details.capabilities.minImageCount + 1, 
			swapchain_support_details.capabilities.maxImageCount
		);

	let swapchain_create_info = 
		VkSwapchainCreateInfoKHR{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
			pNext: nullptr(),
			flags: 0u32,
			surface: surface,
			minImageCount: image_count,
			imageFormat: surface_format.format,
			imageColorSpace: surface_format.colorSpace,
			imageExtent: swapchain_extent,
			imageArrayLayers: 1,
			imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32,
			imageSharingMode: VkSharingMode::VK_SHARING_MODE_CONCURRENT,
			queueFamilyIndexCount: queue_family_indices.len() as _,
			pQueueFamilyIndices: queue_family_indices.as_ptr(),
			preTransform: swapchain_support_details.capabilities.currentTransform,
			compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
			presentMode: present_mode,
			clipped: VK_TRUE,
			oldSwapchain: nullptr(),
		};

	let mut swapchain = nullptr();
	vkCreateSwapchainKHR(device, &swapchain_create_info, nullptr(), &mut swapchain).unwrap("vkCreateSwapchainKHR");

	// swapchain image views
	let mut swapchain_image_count = 0u32;
	vkGetSwapchainImagesKHR(device, swapchain, &mut swapchain_image_count, nullptr());
	let mut swapchain_image_vec: Vec<VkImage> = vec![std::mem::zeroed(); swapchain_image_count as _];
	vkGetSwapchainImagesKHR(device, swapchain, &mut swapchain_image_count, swapchain_image_vec.as_mut_ptr());

	let swapchain_image_views = 
		swapchain_image_vec.iter()
		.map(
			|current_swapchain_image|
			{
				create_image_view(&device, current_swapchain_image, surface_format.format, VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32)
			}
		)
		.collect::<Vec<VkImageView>>();

	println!("Image views created...");

	let command_pool_graphics = 
		CommandPoolBuilder::new()
		.with_flag(VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT)
		.with_queue_family_index(queue_families.graphics)
		.build(&device)
		.unwrap();

	let descriptor_set_layout_ubo = 
		VkDescriptorLayoutBuilder::new()
		.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
		.build(device)
		.unwrap();

	let descriptor_pool_ubo =
		VkDescriptorPoolBuilder::new()
		.add_pool_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 10000)
		.build(device, 10000)
		.unwrap();

	let descriptor_set_layout = 
		VkDescriptorLayoutBuilder::new()
		.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
		.build(device)
		.unwrap();

	let vertex_shader_source = include_bytes!("../detail/shaders/normal_new_layout/vert.spv");
	let fragment_shader_source = include_bytes!("../detail/shaders/normal_new_layout/frag.spv");
	//
	let vertex_shader_module = create_shader_module(device, vertex_shader_source);
	let fragment_shader_module = create_shader_module(device, fragment_shader_source);
	//
	let binding_descriptions = Vertex::get_binding_descriptions();
	let attribute_descriptions_vec = Vertex::get_attribute_descriptions();
	
	let (pipeline_layout, render_pass, pipeline) = 
		create_pipeline(
			picked_device,
			device,
			swapchain_extent,
			surface_format,
			vertex_shader_module, 
			fragment_shader_module, 
			binding_descriptions, 
			attribute_descriptions_vec,
			VkPolygonMode::VK_POLYGON_MODE_FILL, 
			true,
			vec![
				descriptor_set_layout
			]
		);

	
}

pub unsafe fn pick_best_device(instance: VkInstance) -> Option<VkPhysicalDevice>
{
	let mut physical_devices: Vec<VkPhysicalDevice> = vec![];
	let mut physical_devices_count = 0u32;
	vkEnumeratePhysicalDevices(instance, &mut physical_devices_count, nullptr());
	physical_devices.resize(physical_devices_count as _, std::mem::zeroed::<VkPhysicalDevice>());
	vkEnumeratePhysicalDevices(instance, &mut physical_devices_count, physical_devices.as_mut_ptr());

	println!("found {} devices", physical_devices_count);

	// device [0] - AMD Radeon Graphics (RADV RENOIR)
	// device [1] - NVIDIA GeForce GTX 1650
	// device [2] - llvmpipe (LLVM 15.0.7, 256 bits)
	for (index, current_physical_device) in physical_devices.iter().copied().enumerate()
	{
		let mut device_properties = std::mem::zeroed::<VkPhysicalDeviceProperties>();
		vkGetPhysicalDeviceProperties(current_physical_device, &mut device_properties);

		println!("device [{}] - {}", index, from_c_string_ptr(device_properties.deviceName.as_ptr()).unwrap());
	}

	let mut useful_devices = 
		physical_devices.iter().copied().filter(
			|current_physical_device|
			{
				get_queue_family_indices(current_physical_device).is_some()
			}
		).collect::<Vec<VkPhysicalDevice>>();

	useful_devices.sort_by(
		|a, b|
		{
			let mut device_properties_a: VkPhysicalDeviceProperties = std::mem::zeroed();
			vkGetPhysicalDeviceProperties(*a, &mut device_properties_a);

			let mut device_properties_b: VkPhysicalDeviceProperties = std::mem::zeroed();
			vkGetPhysicalDeviceProperties(*b, &mut device_properties_b);

			if device_properties_a.deviceType == VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU &&
				device_properties_b.deviceType == VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU
			{
				return std::cmp::Ordering::Less;
			}
			return std::cmp::Ordering::Greater;
		}
	);

	println!("amount of useful devices : {}", useful_devices.len());

	for (index, current_physical_device) in useful_devices.iter().copied().enumerate()
	{
		let mut device_properties = std::mem::zeroed::<VkPhysicalDeviceProperties>();
		vkGetPhysicalDeviceProperties(current_physical_device, &mut device_properties);

		println!("sorted useful devices [{}] - {}", index, from_c_string_ptr(device_properties.deviceName.as_ptr()).unwrap());
	}

	return useful_devices.first().copied();
}

pub unsafe fn get_queue_family_indices(physical_device: &VkPhysicalDevice) -> Option<QueueFamilies>
{
	let mut count = 0;
	vkGetPhysicalDeviceQueueFamilyProperties(*physical_device, &mut count, nullptr());
	let mut device_queue_family_properties = vec![std::mem::zeroed(); count as usize];
	vkGetPhysicalDeviceQueueFamilyProperties(*physical_device, &mut count, device_queue_family_properties.as_mut_ptr());

	let mut graphics_queue: Option<usize> = None;
	let mut compute_queue: Option<usize> = None;

	for (index, queue_family) in device_queue_family_properties.iter().enumerate()
	{
		if queue_family.queueCount > 0
			&& ((queue_family.queueFlags & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) > 0)
		{
			graphics_queue = Some(index);
			break;
		}
	}

	for (index, queue_family) in device_queue_family_properties.iter().enumerate()
	{
		if queue_family.queueCount > 0
			&& ((queue_family.queueFlags & VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32) > 0)
		{
			compute_queue = Some(index);
			break;
		}
	}

	match (graphics_queue, compute_queue)
	{
		(Some(graphics), Some(compute)) => { return Some(QueueFamilies{graphics: graphics as u32, compute: compute as u32, presentation: graphics as u32}) },
		_ => { return None }
	}
}