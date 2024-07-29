use crate::vulkan::vk_bindgen::{vkCreateDescriptorPool, VkDescriptorPool, VkDescriptorPoolCreateInfo, VkDescriptorPoolSize, VkDescriptorType, VkDevice, VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineShaderStageCreateInfo, VkRenderPass, VkResult, VkShaderModule, VkShaderStageFlagBits, VkStructureType};
use crate::vulkan::vk_bindgen::vkCreateGraphicsPipelines;
use std::ptr::null_mut as nullptr;

pub struct VkPipelineBuilder
{
	pub shader_stages_create_info_vec: Vec<VkPipelineShaderStageCreateInfo>,
	pub render_pass: VkRenderPass,
}

impl VkPipelineBuilder
{
	pub fn new() -> Self 
	{
		return 
			VkPipelineBuilder {
				shader_stages_create_info_vec: vec![],
				render_pass: nullptr(),
			}
	}

	pub fn with_vertex_shader(self, vertex_shader_module: VkShaderModule) -> Self
	{
		self.with_shader(vertex_shader_module, VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT)
	}

	pub fn with_fragment_shader(self, fragment_shader_module: VkShaderModule) -> Self
	{
		self.with_shader(fragment_shader_module, VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT)
	}

	pub fn with_shader(mut self, shader_module: VkShaderModule, shader_stage_flag_bits: VkShaderStageFlagBits) -> Self
	{
		let shader_stage_create_info =
			VkPipelineShaderStageCreateInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
				stage: shader_stage_flag_bits,
				module: shader_module,
				pName: "main\0".as_ptr() as _,
				pSpecializationInfo: nullptr(),
				flags: 0,	
				pNext: nullptr(),
			};

		self.shader_stages_create_info_vec.push(shader_stage_create_info);
		self
	}

	pub fn with_render_pass(mut self, render_pass: VkRenderPass) -> Self 
	{
		self.render_pass = render_pass;
		self
	}

	pub unsafe fn build(self, logical_device: &VkDevice) -> Result<VkPipeline, String>
	{
		let pipeline_create_info = std::mem::zeroed();
		// let pipeline_create_info = 
		// 	VkGraphicsPipelineCreateInfo{
		// 		sType: VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
		// 		stageCount: shader_stages_create_info_vec.len() as _,
		// 		pStages: shader_stages_create_info_vec.as_ptr(),
		// 		pVertexInputState: &vertex_input_create_info,
		// 		pInputAssemblyState: &input_assembly_state_create_info,
		// 		pViewportState: &viewport_state_create_info,
		// 		pRasterizationState: &rasterizer_create_info,
		// 		pMultisampleState: &multisampling_create_info,
		// 		pDepthStencilState: &depth_stencil_create_info,
		// 		pColorBlendState: &color_blend_create_info,
		// 		pDynamicState: &dynamic_state_create_info,
		// 		layout: pipeline_layout,
		// 		renderPass: self.render_pass,
		// 		subpass: 0,
		// 		basePipelineHandle: nullptr(),
		// 		basePipelineIndex: -1,
		// 		pTessellationState: nullptr(),
		// 		flags: 0,	
		// 		pNext: nullptr(),
		// 	};

		let mut pipeline: VkPipeline = nullptr();

		match unsafe { vkCreateGraphicsPipelines(*logical_device, nullptr(), 1, &pipeline_create_info, nullptr(), &mut pipeline) }
		{
			VkResult::VK_SUCCESS => { Ok(pipeline) }
			err_code => { Err(format!("vkCreateGraphicsPipelines() failed with code {:?}.", err_code)) }
		}
	}
}