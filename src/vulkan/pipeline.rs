use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::shader::*;
use crate::vulkan::vertex::*;
use crate::vulkan::depth_buffer::*;
use crate::ffi::strings::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_pipeline(vk_handle: &mut VkHandle)
{
	let binding_description = Vertex::get_binding_description();
	let attribute_descriptions_vec = Vertex::get_attribute_descriptions();

	// Vertex input
	let vertex_input_create_info = 
		VkPipelineVertexInputStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
			vertexBindingDescriptionCount: 1,
			pVertexBindingDescriptions: &binding_description,
			vertexAttributeDescriptionCount: attribute_descriptions_vec.len() as u32,
			pVertexAttributeDescriptions: attribute_descriptions_vec.as_ptr(),
			flags: 0,	
			pNext: nullptr(),
		};

	// Input assembly
	let input_assembly_state_create_info = 
		VkPipelineInputAssemblyStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
			topology: VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
			primitiveRestartEnable: VK_FALSE,
			flags: 0,	
			pNext: nullptr(),
		};

	// Viewport
	let viewport = 
		VkViewport{
			x: 0.0f32,
			y: 0.0f32,
			width: vk_handle.swapchain_extent.width as f32,
			height: vk_handle.swapchain_extent.height as f32,
			minDepth: 0.0f32,
			maxDepth: 1.0f32,
		};

	// Scissor
	let scissor = 
		VkRect2D{
			offset: VkOffset2D { x: 0, y: 0 },
			extent: vk_handle.swapchain_extent
		};

	// Dynamic states
	let dynamic_states_vec =
		vec![
			VkDynamicState::VK_DYNAMIC_STATE_VIEWPORT,
			VkDynamicState::VK_DYNAMIC_STATE_SCISSOR
		];

	let dynamic_state_create_info = 
		VkPipelineDynamicStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
			dynamicStateCount: dynamic_states_vec.len() as u32,
			pDynamicStates: dynamic_states_vec.as_ptr(),
			flags: 0,	
			pNext: nullptr(),
		};

	// Viewport state
	let viewport_state_create_info = 
		VkPipelineViewportStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
			viewportCount: 1,
			pViewports: &viewport,
			scissorCount: 1,
			pScissors: &scissor,
			flags: 0,	
			pNext: nullptr(),
		};

	// Rasterizer
	let rasterizer_create_info = 
		VkPipelineRasterizationStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
			depthClampEnable: VK_FALSE,
			rasterizerDiscardEnable: VK_FALSE,
			polygonMode: VkPolygonMode::VK_POLYGON_MODE_FILL,
			lineWidth: 1.0f32,
			// cullMode: VkCullModeFlagBits::VK_CULL_MODE_BACK_BIT as u32,
			cullMode: VkCullModeFlagBits::VK_CULL_MODE_NONE as u32,
			frontFace: VkFrontFace::VK_FRONT_FACE_CLOCKWISE,
			// frontFace: VkFrontFace::VK_FRONT_FACE_COUNTER_CLOCKWISE,
			depthBiasEnable: VK_FALSE,
			depthBiasConstantFactor: 0.0f32,
			depthBiasClamp: 0.0f32,
			depthBiasSlopeFactor: 0.0f32,
			flags: 0,	
			pNext: nullptr(),
		};

	// Multisampling
	let multisampling_create_info = 
		VkPipelineMultisampleStateCreateInfo{
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
	let color_blend_attachment_state = 
		VkPipelineColorBlendAttachmentState{
			// blendEnable: VK_FALSE,
			blendEnable: VK_TRUE,
			// srcColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ONE,
			srcColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_SRC_ALPHA,
			// dstColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
			dstColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
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

	let color_blend_create_info = 
		VkPipelineColorBlendStateCreateInfo{
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
	let pipeline_layout_create_info = 
		VkPipelineLayoutCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			setLayoutCount: 1,
			pSetLayouts: &vk_handle.descriptor_set_layout,
			pushConstantRangeCount: 0,
			pPushConstantRanges: nullptr(),
			flags: 0,	
			pNext: nullptr(),
		};

	match vkCreatePipelineLayout(vk_handle.logical_device, &pipeline_layout_create_info, nullptr(), &mut vk_handle.pipeline_layout)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreatePipelineLayout()"); }
		err => { panic!("✗ vkCreatePipelineLayout() failed with code {:?}.", err); }
	}		

	//// Render pass creation
	let color_attachment_descriptor = 
		VkAttachmentDescription{
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

	let depth_attachment_descriptor = 
		VkAttachmentDescription{
			format: find_depth_format(vk_handle),
			samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
			loadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
			stencilLoadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			stencilStoreOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
			initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
			finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			flags: 0
		};

	// Attachment references
	let color_attachment_reference = 
		VkAttachmentReference{
			attachment: 0,
			layout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
		};

	let depth_attachment_reference = 
		VkAttachmentReference{
			attachment: 1,
			layout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
		};

	// Subpass
	let subpass = 
		VkSubpassDescription{
			pipelineBindPoint: VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
			colorAttachmentCount: 1,
			pColorAttachments: &color_attachment_reference,

			inputAttachmentCount: 0,
			pInputAttachments: nullptr(),
			preserveAttachmentCount: 0,
			pPreserveAttachments: nullptr(),
			pDepthStencilAttachment: &depth_attachment_reference,
			pResolveAttachments: nullptr(),
			flags: 0
		};

	// render pass dependencies
	let subpass_dependency = 
		VkSubpassDependency{
			srcSubpass: VK_SUBPASS_EXTERNAL as u32,
			dstSubpass: 0,
			srcStageMask: 
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32 |
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT as u32,
			srcAccessMask: 0,
			dstStageMask: 
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32 |
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT as u32,
			dstAccessMask: 
				VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32 |
				VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT as u32,
			dependencyFlags: 0,
		};

	let attachment_descriptors = vec![color_attachment_descriptor, depth_attachment_descriptor];

	// the actual Render pass creation
	let render_pass_create_info = 
		VkRenderPassCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
			attachmentCount: attachment_descriptors.len() as _,
			pAttachments: attachment_descriptors.as_ptr(),
			subpassCount: 1,
			pSubpasses: &subpass,
			dependencyCount: 1,
			pDependencies: &subpass_dependency,
			flags: 0,	
			pNext: nullptr(),
		};

	match vkCreateRenderPass(vk_handle.logical_device, &render_pass_create_info, nullptr(), &mut vk_handle.render_pass)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateRenderPass()"); }
		err => { panic!("✗ vkCreateRenderPass() failed with code {:?}.", err); }
	}		

	let depth_stencil_create_info = 
		VkPipelineDepthStencilStateCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
			depthTestEnable: VK_TRUE,
			depthWriteEnable: VK_TRUE,
			depthCompareOp: VkCompareOp::VK_COMPARE_OP_LESS,
			depthBoundsTestEnable: VK_FALSE,
			minDepthBounds: 0.0f32,
			maxDepthBounds: 1.0f32,
			stencilTestEnable: VK_FALSE,
			front: VkStencilOpState{ ..Default::default() },
			back: VkStencilOpState{ ..Default::default() },
			flags: 0,	
			pNext: nullptr(),
		};


	// Shader creation
	// let vertex_shader_source = include_bytes!("../../shaders/basic_triangle/vert.spv");
	// let fragment_shader_source = include_bytes!("../../shaders/basic_triangle/frag.spv");

	// let vertex_shader_source = include_bytes!("../../shaders/uniform_buffer/vert.spv");
	// let fragment_shader_source = include_bytes!("../../shaders/uniform_buffer/frag.spv");

	// let vertex_shader_source = include_bytes!("../../shaders/textured/vert.spv");
	// let fragment_shader_source = include_bytes!("../../shaders/textured/frag.spv");

	let vertex_shader_source = include_bytes!("../../shaders/test/vert.spv");
	let fragment_shader_source = include_bytes!("../../shaders/test/frag.spv");

	vk_handle.vertex_shader_module = create_shader_module(&vk_handle, vertex_shader_source);
	vk_handle.fragment_shader_module = create_shader_module(&vk_handle, fragment_shader_source);

	let shader_stages_create_info_vec = 
		vec![
			VkPipelineShaderStageCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
				stage: VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
				module: vk_handle.vertex_shader_module,
				pName: to_c_string("main"),
				pSpecializationInfo: nullptr(),
				flags: 0,	
				pNext: nullptr(),
			},
			VkPipelineShaderStageCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
				stage: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
				module: vk_handle.fragment_shader_module,
				pName: to_c_string("main"),
				pSpecializationInfo: nullptr(),
				flags: 0,	
				pNext: nullptr(),
			}
		];

	//// Pipeline creation
	let pipeline_create_info = 
		VkGraphicsPipelineCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
			stageCount: 2,
			pStages: shader_stages_create_info_vec.as_ptr(),
			pVertexInputState: &vertex_input_create_info,
			pInputAssemblyState: &input_assembly_state_create_info,
			pViewportState: &viewport_state_create_info,
			pRasterizationState: &rasterizer_create_info,
			pMultisampleState: &multisampling_create_info,
			pDepthStencilState: &depth_stencil_create_info,
			// pDepthStencilState:  nullptr(),
			pColorBlendState: &color_blend_create_info,
			pDynamicState: &dynamic_state_create_info,
			layout: vk_handle.pipeline_layout,
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
}