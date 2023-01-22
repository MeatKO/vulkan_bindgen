#![allow(non_upper_case_globals)]

use std::{ffi::c_void, cmp::min};
use std::ptr::null_mut as nullptr;

mod vulkan;
use vulkan::{
	vk_bindgen::*, c_macros::*, debugger::*, device::*, extension::*, 
	layers::*, handle::VkHandle, queue::*, swapchain::*, shader::*,
	command_buffer::*,
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
            "VK_LAYER_VALVE_steam_overlay_64"
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
			graphics_pipeline: nullptr()
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

		let swapchain_support_details = query_swapchain_support(vk_handle.physical_device, vk_handle.window_surface);
		
		// Swapchain creation
		vk_handle.surface_format = choose_swap_surface_format(&swapchain_support_details.formats).expect("Couldn't find suitable window surface format.");
		vk_handle.present_mode = choose_swap_present_mode(&swapchain_support_details.present_modes);
		vk_handle.extent = choose_swap_extent(&swapchain_support_details.capabilities);

		let image_count =
			min(
				swapchain_support_details.capabilities.minImageCount + 1, 
				swapchain_support_details.capabilities.maxImageCount
			);

		// queue stuff for the swapchain creation : 
		let queue_family_indices = 
			vec![
				queue_handle.graphics_queue.as_ref().unwrap().family_index, 
				queue_handle.presentation_queue.as_ref().unwrap().family_index
			];

		let mut swapchain_create_info = VkSwapchainCreateInfoKHR{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
			surface: vk_handle.window_surface,
			minImageCount: image_count,
			imageFormat: vk_handle.surface_format.format,
			imageColorSpace: vk_handle.surface_format.colorSpace,
			imageExtent: vk_handle.extent,
			imageArrayLayers: 1,
			imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices: nullptr(),
			imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32,
			preTransform: swapchain_support_details.capabilities.currentTransform,
			compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
			presentMode: vk_handle.present_mode,
			clipped: VK_TRUE,
			oldSwapchain: nullptr(),
			flags: 0,
			pNext: nullptr(),
		};
		if graphics_queue != presentation_queue
		{
			swapchain_create_info.imageSharingMode = VkSharingMode::VK_SHARING_MODE_CONCURRENT;
			swapchain_create_info.queueFamilyIndexCount = 2;
			swapchain_create_info.pQueueFamilyIndices = queue_family_indices.as_ptr();
		}

		let mut swapchain = std::mem::zeroed();
		match vkCreateSwapchainKHR(vk_handle.logical_device, &swapchain_create_info, nullptr(), &mut swapchain)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateSwapchainKHR()"); }
			err => { panic!("✗ vkCreateSwapchainKHR() failed with code {:?}.", err); }
		}

		// Swapchain images
		let mut swapchain_images_count = 0u32;
		vkGetSwapchainImagesKHR(vk_handle.logical_device, swapchain, &mut swapchain_images_count, nullptr());
		let mut swapchain_images_vec = vec![ std::mem::zeroed(); swapchain_images_count as usize ];
		vkGetSwapchainImagesKHR(vk_handle.logical_device, swapchain, &mut swapchain_images_count, swapchain_images_vec.as_mut_ptr());

		// Swapchain image views
		let mut swapchain_image_views_vec: Vec<VkImageView> = vec![nullptr(); swapchain_images_count as usize];
	
		for i in 0..swapchain_images_vec.len()
		{
			let swapchain_image_view_create_info = VkImageViewCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
				image: swapchain_images_vec[i],
				viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
				format: vk_handle.surface_format.format,
				components: VkComponentMapping { 
						r: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
						g: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
						b: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
						a: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY
					},
				subresourceRange: VkImageSubresourceRange { 
						aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32, 
						baseMipLevel: 0, 
						levelCount: 1, 
						baseArrayLayer: 0, 
						layerCount: 1 
					},
				flags: 0,
				pNext: nullptr(),
			};

			match vkCreateImageView(vk_handle.logical_device, &swapchain_image_view_create_info, nullptr(), &mut swapchain_image_views_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateImageView()"); }
				err => { panic!("✗ vkCreateImageView() failed with code {:?}.", err); }
			}
		}

		// Shader creation
		let vertex_shader_source = include_bytes!("../shaders/basic_triangle/vert.spv");
		let fragment_shader_source = include_bytes!("../shaders/basic_triangle/frag.spv");

		let vertex_shader_module = create_shader_module(&vk_handle, vertex_shader_source);
		let fragment_shader_module = create_shader_module(&vk_handle, fragment_shader_source);

		let shader_stages_create_info_vec = vec![
			VkPipelineShaderStageCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
				stage: VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
				module: vertex_shader_module,
				pName: to_c_string("main"),
				pSpecializationInfo: nullptr(),
				flags: 0,	
				pNext: nullptr(),
			},
			VkPipelineShaderStageCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
				stage: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
				module: fragment_shader_module,
				pName: to_c_string("main"),
				pSpecializationInfo: nullptr(),
				flags: 0,	
				pNext: nullptr(),
			}
		];

		// Vertex input
		let vertex_input_create_info = VkPipelineVertexInputStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
			vertexBindingDescriptionCount: 0,
			pVertexBindingDescriptions: nullptr(),
			vertexAttributeDescriptionCount: 0,
			pVertexAttributeDescriptions: nullptr(),
			flags: 0,	
			pNext: nullptr(),
		};

		// Input assembly
		let input_assembly_state_create_info = VkPipelineInputAssemblyStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
			topology: VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
			primitiveRestartEnable: VK_FALSE,
			flags: 0,	
			pNext: nullptr(),
		};

		// Viewport
		let viewport = VkViewport{
			x: 0.0f32,
			y: 0.0f32,
			width: vk_handle.extent.width as f32,
			height: vk_handle.extent.height as f32,
			minDepth: 0.0f32,
			maxDepth: 1.0f32,
		};

		// Scissor
		let scissor = VkRect2D{
			offset: VkOffset2D { x: 0, y: 0 },
			extent: vk_handle.extent
		};

		// Dynamic states
		let dynamic_states_vec = vec![
			VkDynamicState::VK_DYNAMIC_STATE_VIEWPORT,
			VkDynamicState::VK_DYNAMIC_STATE_SCISSOR
		];

		let dynamic_state_create_info = VkPipelineDynamicStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
			dynamicStateCount: dynamic_states_vec.len() as u32,
			pDynamicStates: dynamic_states_vec.as_ptr(),
			flags: 0,	
			pNext: nullptr(),
		};

		// Viewport state
		let viewport_state_create_info = VkPipelineViewportStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
			viewportCount: 1,
			pViewports: &viewport,
			scissorCount: 1,
			pScissors: &scissor,
			flags: 0,	
			pNext: nullptr(),
		};

		// Rasterizer
		let rasterizer_create_info = VkPipelineRasterizationStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
			depthClampEnable: VK_FALSE,
			rasterizerDiscardEnable: VK_FALSE,
			polygonMode: VkPolygonMode::VK_POLYGON_MODE_FILL,
			lineWidth: 1.0f32,
			cullMode: VkCullModeFlagBits::VK_CULL_MODE_BACK_BIT as u32,
			frontFace: VkFrontFace::VK_FRONT_FACE_CLOCKWISE,
			depthBiasEnable: VK_FALSE,
			depthBiasConstantFactor: 0.0f32,
			depthBiasClamp: 0.0f32,
			depthBiasSlopeFactor: 0.0f32,
			flags: 0,	
			pNext: nullptr(),
		};

		// Multisampling
		let multisampling_create_info = VkPipelineMultisampleStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
			sampleShadingEnable: VK_FALSE,
			rasterizationSamples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
			minSampleShading: 1.0f32,
			pSampleMask: nullptr(),
			alphaToCoverageEnable: VK_FALSE,
			alphaToOneEnable: VK_FALSE,
			flags: 0,	
			pNext: nullptr(),
		};

		// Color blending
		let color_blend_attachment_state = VkPipelineColorBlendAttachmentState{
			blendEnable: VK_FALSE,
			srcColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ONE,
			dstColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
			colorBlendOp: VkBlendOp::VK_BLEND_OP_ADD,
			srcAlphaBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ONE,
			dstAlphaBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
			alphaBlendOp: VkBlendOp::VK_BLEND_OP_ADD,
			colorWriteMask: {
				VkColorComponentFlagBits::VK_COLOR_COMPONENT_R_BIT as u32 | 
				VkColorComponentFlagBits::VK_COLOR_COMPONENT_G_BIT as u32 | 
				VkColorComponentFlagBits::VK_COLOR_COMPONENT_B_BIT as u32 | 
				VkColorComponentFlagBits::VK_COLOR_COMPONENT_A_BIT as u32
			}
		};

		let color_blend_create_info = VkPipelineColorBlendStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
			logicOpEnable: VK_FALSE,
			logicOp: VkLogicOp::VK_LOGIC_OP_COPY,
			attachmentCount: 1,
			pAttachments: &color_blend_attachment_state,
			blendConstants: [0.0f32, 0.0f32, 0.0f32, 0.0f32],
			flags: 0,	
			pNext: nullptr(),
		};

		// Pipeline
		let pipeline_layout_create_info = VkPipelineLayoutCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			setLayoutCount: 0,
			pSetLayouts: nullptr(),
			pushConstantRangeCount: 0,
			pPushConstantRanges: nullptr(),
			flags: 0,	
			pNext: nullptr(),
		};

		let mut pipeline_layout = std::mem::zeroed();
		match vkCreatePipelineLayout(vk_handle.logical_device, &pipeline_layout_create_info, nullptr(), &mut pipeline_layout)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreatePipelineLayout()"); }
			err => { panic!("✗ vkCreatePipelineLayout() failed with code {:?}.", err); }
		}		

		//// Render pass creation
		let color_attachment_descriptor = VkAttachmentDescription{
			format: vk_handle.surface_format.format,
			samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
			loadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE,
			stencilLoadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			stencilStoreOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
			initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
			finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
			flags: 0
		};

		// Attachment references
		let color_attachment_reference = VkAttachmentReference{
			attachment: 0,
			layout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
		};

		// Subpass
		let subpass = VkSubpassDescription{
			pipelineBindPoint: VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
			colorAttachmentCount: 1,
			pColorAttachments: &color_attachment_reference,

			inputAttachmentCount: 0,
			pInputAttachments: nullptr(),
			preserveAttachmentCount: 0,
			pPreserveAttachments: nullptr(),
			pDepthStencilAttachment: nullptr(),
			pResolveAttachments: nullptr(),
			flags: 0
		};

		// the actual Render pass creation
		let render_pass_create_info = VkRenderPassCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
			attachmentCount: 1,
			pAttachments: &color_attachment_descriptor,
			subpassCount: 1,
			pSubpasses: &subpass,
			dependencyCount: 0,
			pDependencies: nullptr(),
			flags: 0,	
			pNext: nullptr(),
		};

		// let mut render_pass = std::mem::zeroed();
		match vkCreateRenderPass(vk_handle.logical_device, &render_pass_create_info, nullptr(), &mut vk_handle.render_pass)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateRenderPass()"); }
			err => { panic!("✗ vkCreateRenderPass() failed with code {:?}.", err); }
		}		

		//// Pipeline creation
		let pipeline_create_info = VkGraphicsPipelineCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
			stageCount: 2,
			pStages: shader_stages_create_info_vec.as_ptr(),
			pVertexInputState: &vertex_input_create_info,
			pInputAssemblyState: &input_assembly_state_create_info,
			pViewportState: &viewport_state_create_info,
			pRasterizationState: &rasterizer_create_info,
			pMultisampleState: &multisampling_create_info,
			pDepthStencilState: nullptr(),
			pColorBlendState: &color_blend_create_info,
			pDynamicState: &dynamic_state_create_info,
			layout: pipeline_layout,
			renderPass: vk_handle.render_pass,
			subpass: 0,
			basePipelineHandle: nullptr(),
			basePipelineIndex: -1,
			pTessellationState: nullptr(),
			flags: 0,	
			pNext: nullptr(),
		};

		match vkCreateGraphicsPipelines(vk_handle.logical_device, nullptr(), 1, &pipeline_create_info, nullptr(), &mut vk_handle.graphics_pipeline)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateGraphicsPipelines()"); }
			err => { panic!("✗ vkCreateGraphicsPipelines() failed with code {:?}.", err); }
		}	

		// Framebuffers
		vk_handle.swapchain_framebuffers = vec![std::mem::zeroed(); swapchain_image_views_vec.len()];

		for i in 0..swapchain_image_views_vec.len()
		{
			let attachments = vec![swapchain_image_views_vec[i]];

			let framebuffer_create_info = VkFramebufferCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
				renderPass: vk_handle.render_pass,
				attachmentCount: 1,
				pAttachments: attachments.as_ptr(),
				width: vk_handle.extent.width,
				height: vk_handle.extent.height,
				layers: 1,
				flags: 0,	
				pNext: nullptr(),
			};

			match vkCreateFramebuffer(vk_handle.logical_device, &framebuffer_create_info, nullptr(), &mut vk_handle.swapchain_framebuffers[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateFramebuffer()"); }
				err => { panic!("✗ vkCreateFramebuffer() failed with code {:?}.", err); }
			}	
		}

		// Command pool creation
		let command_pool_create_info = VkCommandPoolCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
			queueFamilyIndex: queue_family_indices[0],
			flags: VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32,	
			pNext: nullptr(),
		};

		let mut command_pool = std::mem::zeroed();
		match vkCreateCommandPool(vk_handle.logical_device, &command_pool_create_info, nullptr(), &mut command_pool)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateCommandPool()"); }
			err => { panic!("✗ vkCreateCommandPool() failed with code {:?}.", err); }
		}	

		// Command buffer
		let command_buffer_create_info = VkCommandBufferAllocateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
			commandPool: command_pool,
			level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
			commandBufferCount: 1,
			pNext: nullptr(),
		};

		let mut command_buffer = std::mem::zeroed();
		match vkAllocateCommandBuffers(vk_handle.logical_device, &command_buffer_create_info, &mut command_buffer)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkAllocateCommandBuffers()"); }
			err => { panic!("✗ vkAllocateCommandBuffers() failed with code {:?}.", err); }
		}	

		// Command buffer recording 
		record_command_buffer(&vk_handle, command_buffer, 0);

		std::thread::sleep(std::time::Duration::from_secs(2));

		// Cleanup
		println!("Destroying vk objects...");
		vkDestroyCommandPool(vk_handle.logical_device, command_pool, nullptr());
		for framebuffer in vk_handle.swapchain_framebuffers
		{
			vkDestroyFramebuffer(vk_handle.logical_device, framebuffer, nullptr());
		}
		vkDestroyPipelineLayout(vk_handle.logical_device, pipeline_layout, nullptr());
		vkDestroyPipeline(vk_handle.logical_device, vk_handle.graphics_pipeline, nullptr());
		vkDestroyRenderPass(vk_handle.logical_device, vk_handle.render_pass, nullptr());
		vkDestroyShaderModule(vk_handle.logical_device, fragment_shader_module, nullptr());
		vkDestroyShaderModule(vk_handle.logical_device, vertex_shader_module, nullptr());
		for image_view in swapchain_image_views_vec
		{
			vkDestroyImageView(vk_handle.logical_device, image_view, nullptr());
		}
		vkDestroySwapchainKHR(vk_handle.logical_device, swapchain, nullptr());
		vkDestroySurfaceKHR(vk_instance, vk_handle.window_surface, nullptr());
		vkDestroyDevice(vk_handle.logical_device, nullptr());
		destroy_debug_utils_messenger_ext(&vk_instance, &debug_messenger, nullptr());
		vkDestroyInstance(vk_instance, nullptr());
	}
}
