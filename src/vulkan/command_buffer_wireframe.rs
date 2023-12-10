use crate::detail_core::model::model::Model;
use crate::detail_core::model::model::VulkanModel;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

// pub unsafe fn create_command_buffers(vk_handle: &mut VkHandle)
// {
// 	let command_buffer_count = vk_handle.frames_in_flight;

// 	let command_buffer_create_info = 
// 		VkCommandBufferAllocateInfo {
// 			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
// 			commandPool: vk_handle.command_pool,
// 			level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
// 			commandBufferCount: command_buffer_count as u32,
// 			pNext: nullptr(),
// 		};

// 	vk_handle.command_buffer_vec.resize(command_buffer_count, nullptr());
// 	match vkAllocateCommandBuffers(vk_handle.logical_device, &command_buffer_create_info, vk_handle.command_buffer_vec.as_mut_ptr())
// 	{
// 		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateCommandBuffers()"); }
// 		err => { panic!("✗ vkAllocateCommandBuffers() failed with code {:?}.", err); }
// 	}
// }


pub unsafe fn record_command_buffer_wireframe(
	vk_handle: &VkHandle, 
	image_index: u32, 
	models: &Vec<Model<VulkanModel>>
)
{
	let current_command_buffer = vk_handle.command_buffer_wireframe_vec[vk_handle.current_frame].get_command_buffer_ptr();

	let command_buffer_begin_info = 
		VkCommandBufferBeginInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			flags: 0,
			pInheritanceInfo: nullptr(),
			pNext: nullptr()
		};

	match vkBeginCommandBuffer(current_command_buffer, &command_buffer_begin_info)
	{
		VkResult::VK_SUCCESS => {  }
		err => { panic!("✗ vkBeginCommandBuffer() failed with code {:?}.", err); }
	}

	// order here should be identical to the order of attachments...
	let clear_values = 
		vec![
			VkClearValue{
				color: VkClearColorValue{
					float32: [0.0f32, 0.0f32, 0.0f32, 0.0f32]
				}
			},
			VkClearValue{
				depthStencil: VkClearDepthStencilValue { 
					depth: 1.0f32, 
					stencil: 0,
				}
			},
		];

	let render_pass_begin_info = 
		VkRenderPassBeginInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
			renderPass: vk_handle.render_pass_wireframe,
			framebuffer: vk_handle.swapchain_framebuffers[image_index as usize],
			renderArea: VkRect2D { 
					offset: VkOffset2D { x: 0, y: 0 }, 
					extent: vk_handle.swapchain_extent
				},
			clearValueCount: clear_values.len() as _,
			pClearValues: clear_values.as_ptr(),
			pNext: nullptr()
		};

	vkCmdBeginRenderPass(
		current_command_buffer, 
		&render_pass_begin_info, 
		VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE
	);

	vkCmdBindPipeline(
		current_command_buffer, 
		VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
		vk_handle.graphics_pipeline_wireframe
	);

	let viewport = 
		VkViewport{
			x: 0.0f32,
			y: 0.0f32,
			width: vk_handle.swapchain_extent.width as f32,
			height: vk_handle.swapchain_extent.height as f32,
			minDepth: 0.0f32,
			maxDepth: 1.0f32,
		};

	vkCmdSetViewport(
		current_command_buffer, 
		0, 
		1,
		&viewport
	);

	let scissor = 
		VkRect2D{
			offset: VkOffset2D { 
				x: 0, 
				y: 0 
			},
			extent: vk_handle.swapchain_extent,
		};

	vkCmdSetScissor(
		current_command_buffer, 
		0, 
		1, 
		&scissor
	);

	for model in models.iter()
	{
		let vulkan_data = 
			match &model.aabb_vulkan_data
			{
				Some(vd) => vd,
				None => continue
			};

		let vertex_buffers: Vec<VkBuffer> = 
		vec![
			vulkan_data.vertex_buffer,
		];

		let offsets = vec![0];

		vkCmdBindVertexBuffers(
			current_command_buffer, 
			0, 
			1, 
			vertex_buffers.as_ptr(), 
			offsets.as_ptr()
		);
		
		// add a type constraint on the index buffer later, it must be equal to the type of the buffer !
		vkCmdBindIndexBuffer(
			current_command_buffer,
			vulkan_data.index_buffer, 
			0,
			VkIndexType::VK_INDEX_TYPE_UINT32,
		);

		vkCmdBindDescriptorSets(
			current_command_buffer, 
			VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
			vk_handle.pipeline_layout_wireframe, 
			0, 
			1, 
			&vulkan_data.descriptor_sets[vk_handle.current_frame],
			0, 
			nullptr()
		);

		vkCmdDrawIndexed(
			current_command_buffer,
			model.aabb_index_count,
			1, 
			0, 
			0, 
			0
		);
	}
	
	// vkCmdDraw(current_command_buffer, vk_handle.vertices.len() as u32, 1, 0, 0);
	// vkCmdDraw(current_command_buffer, model.vertices.len() as u32, 1, 0, 0);

	vkCmdEndRenderPass(current_command_buffer);

	match vkEndCommandBuffer(current_command_buffer)
	{
		// VkResult::VK_SUCCESS => { println!("✔️ vkEndCommandBuffer()"); }
		VkResult::VK_SUCCESS => {  }
		err => { panic!("✗ vkEndCommandBuffer() failed with code {:?}.", err); }
	}
}


pub unsafe fn record_command_buffer_wireframe_ref(
	vk_handle: &VkHandle, 
	image_index: u32, 
	models: &Vec<&Model<VulkanModel>>
)
{
	let current_command_buffer = vk_handle.command_buffer_wireframe_vec[vk_handle.current_frame].get_command_buffer_ptr();

	let command_buffer_begin_info = 
		VkCommandBufferBeginInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			flags: 0,
			pInheritanceInfo: nullptr(),
			pNext: nullptr()
		};

	match vkBeginCommandBuffer(current_command_buffer, &command_buffer_begin_info)
	{
		VkResult::VK_SUCCESS => {  }
		err => { panic!("✗ vkBeginCommandBuffer() failed with code {:?}.", err); }
	}

	// order here should be identical to the order of attachments...
	let clear_values = 
		vec![
			VkClearValue{
				color: VkClearColorValue{
					float32: [0.0f32, 0.0f32, 0.0f32, 0.0f32]
				}
			},
			VkClearValue{
				depthStencil: VkClearDepthStencilValue { 
					depth: 1.0f32, 
					stencil: 0,
				}
			},
		];

	let render_pass_begin_info = 
		VkRenderPassBeginInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
			renderPass: vk_handle.render_pass_wireframe,
			framebuffer: vk_handle.swapchain_framebuffers[image_index as usize],
			renderArea: VkRect2D { 
					offset: VkOffset2D { x: 0, y: 0 }, 
					extent: vk_handle.swapchain_extent
				},
			clearValueCount: clear_values.len() as _,
			pClearValues: clear_values.as_ptr(),
			pNext: nullptr()
		};

	vkCmdBeginRenderPass(
		current_command_buffer, 
		&render_pass_begin_info, 
		VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE
	);

	vkCmdBindPipeline(
		current_command_buffer, 
		VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
		vk_handle.graphics_pipeline_wireframe
	);

	let viewport = 
		VkViewport{
			x: 0.0f32,
			y: 0.0f32,
			width: vk_handle.swapchain_extent.width as f32,
			height: vk_handle.swapchain_extent.height as f32,
			minDepth: 0.0f32,
			maxDepth: 1.0f32,
		};

	vkCmdSetViewport(
		current_command_buffer, 
		0, 
		1,
		&viewport
	);

	let scissor = 
		VkRect2D{
			offset: VkOffset2D { 
				x: 0, 
				y: 0 
			},
			extent: vk_handle.swapchain_extent,
		};

	vkCmdSetScissor(
		current_command_buffer, 
		0, 
		1, 
		&scissor
	);

	for model in models.iter()
	{
		let vulkan_data = 
			match &model.aabb_vulkan_data
			{
				Some(vd) => vd,
				None => continue
			};

		let vertex_buffers: Vec<VkBuffer> = 
		vec![
			vulkan_data.vertex_buffer,
		];

		let offsets = vec![0];

		vkCmdBindVertexBuffers(
			current_command_buffer, 
			0, 
			1, 
			vertex_buffers.as_ptr(), 
			offsets.as_ptr()
		);
		
		// add a type constraint on the index buffer later, it must be equal to the type of the buffer !
		vkCmdBindIndexBuffer(
			current_command_buffer,
			vulkan_data.index_buffer, 
			0,
			VkIndexType::VK_INDEX_TYPE_UINT32,
		);

		vkCmdBindDescriptorSets(
			current_command_buffer, 
			VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
			vk_handle.pipeline_layout_wireframe, 
			0, 
			1, 
			&vulkan_data.descriptor_sets[vk_handle.current_frame],
			0, 
			nullptr()
		);

		vkCmdDrawIndexed(
			current_command_buffer,
			model.aabb_index_count,
			1, 
			0, 
			0, 
			0
		);
	}
	
	// vkCmdDraw(current_command_buffer, vk_handle.vertices.len() as u32, 1, 0, 0);
	// vkCmdDraw(current_command_buffer, model.vertices.len() as u32, 1, 0, 0);

	vkCmdEndRenderPass(current_command_buffer);

	match vkEndCommandBuffer(current_command_buffer)
	{
		// VkResult::VK_SUCCESS => { println!("✔️ vkEndCommandBuffer()"); }
		VkResult::VK_SUCCESS => {  }
		err => { panic!("✗ vkEndCommandBuffer() failed with code {:?}.", err); }
	}
}