use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;


pub unsafe fn create_command_buffer(vk_handle: &mut VkHandle)
{
	let command_buffer_create_info = VkCommandBufferAllocateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
		commandPool: vk_handle.command_pool,
		level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
		commandBufferCount: vk_handle.frames_in_flight as u32,
		pNext: nullptr(),
	};

	vk_handle.command_buffer_vec.resize(vk_handle.frames_in_flight, nullptr());
	match vkAllocateCommandBuffers(vk_handle.logical_device, &command_buffer_create_info, vk_handle.command_buffer_vec.as_mut_ptr())
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateCommandBuffers()"); }
		err => { panic!("✗ vkAllocateCommandBuffers() failed with code {:?}.", err); }
	}
}

pub fn record_command_buffer(vk_handle: &VkHandle, image_index: u32)
{
	let command_buffer_begin_info = VkCommandBufferBeginInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
		flags: 0,
		pInheritanceInfo: nullptr(),
		pNext: nullptr()
	};

	unsafe 
	{
		match vkBeginCommandBuffer(vk_handle.command_buffer_vec[vk_handle.current_frame], &command_buffer_begin_info)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkBeginCommandBuffer()"); }
			err => { panic!("✗ vkBeginCommandBuffer() failed with code {:?}.", err); }
		}
	}

	let clear_value = 
		VkClearValue{
			color: VkClearColorValue{
				float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32]
			}
		};

	let render_pass_begin_info = VkRenderPassBeginInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
		renderPass: vk_handle.render_pass,
		framebuffer: vk_handle.swapchain_framebuffers[image_index as usize],
		renderArea: VkRect2D { 
			offset: VkOffset2D { x: 0, y: 0 }, 
			extent: vk_handle.extent
		},
		clearValueCount: 1,
		pClearValues: &clear_value,
		pNext: nullptr()
	};

	unsafe
	{
		vkCmdBeginRenderPass(vk_handle.command_buffer_vec[vk_handle.current_frame], &render_pass_begin_info, VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE);
		vkCmdBindPipeline(vk_handle.command_buffer_vec[vk_handle.current_frame], VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS , vk_handle.graphics_pipeline);
	}

	let viewport = VkViewport{
		x: 0.0f32,
		y: 0.0f32,
		width: vk_handle.extent.width as f32,
		height: vk_handle.extent.height as f32,
		minDepth: 0.0f32,
		maxDepth: 1.0f32,
	};
	unsafe
	{
		vkCmdSetViewport(vk_handle.command_buffer_vec[vk_handle.current_frame], 0, 1, &viewport);
	}

	let scissor = VkRect2D{
		offset: VkOffset2D { x: 0, y: 0 },
		extent: vk_handle.extent
	};
	unsafe
	{
		vkCmdSetScissor(vk_handle.command_buffer_vec[vk_handle.current_frame], 0, 1, &scissor);
	
		let vertex_buffers: Vec<VkBuffer> = vec![vk_handle.vertex_buffer];
		let offsets = vec![0];

		vkCmdBindVertexBuffers(vk_handle.command_buffer_vec[vk_handle.current_frame], 0, 1, vertex_buffers.as_ptr(), offsets.as_ptr());

		vkCmdDraw(vk_handle.command_buffer_vec[vk_handle.current_frame], vk_handle.vertices.len() as u32, 1, 0, 0);
		// vkCmdDraw(vk_handle.command_buffer_vec[vk_handle.current_frame], 3, 1, 0, 0);
		vkCmdEndRenderPass(vk_handle.command_buffer_vec[vk_handle.current_frame]);

		match vkEndCommandBuffer(vk_handle.command_buffer_vec[vk_handle.current_frame])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkEndCommandBuffer()"); }
			err => { panic!("✗ vkEndCommandBuffer() failed with code {:?}.", err); }
		}	
	}
}

/*
void recordCommandBuffer(VkCommandBuffer commandBuffer, uint32_t imageIndex) {
        VkCommandBufferBeginInfo beginInfo{};
        beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;

        if (vkBeginCommandBuffer(commandBuffer, &beginInfo) != VK_SUCCESS) {
            throw std::runtime_error("failed to begin recording command buffer!");
        }

        VkRenderPassBeginInfo renderPassInfo{};
        renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
        renderPassInfo.renderPass = renderPass;
        renderPassInfo.framebuffer = swapChainFramebuffers[imageIndex];
        renderPassInfo.renderArea.offset = {0, 0};
        renderPassInfo.renderArea.extent = swapChainExtent;

        VkClearValue clearColor = {{{0.0f, 0.0f, 0.0f, 1.0f}}};
        renderPassInfo.clearValueCount = 1;
        renderPassInfo.pClearValues = &clearColor;

        vkCmdBeginRenderPass(commandBuffer, &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);

		vkCmdBindPipeline(commandBuffer, VK_PIPELINE_BIND_POINT_GRAPHICS, graphicsPipeline);

		VkViewport viewport{};
		viewport.x = 0.0f;
		viewport.y = 0.0f;
		viewport.width = (float) swapChainExtent.width;
		viewport.height = (float) swapChainExtent.height;
		viewport.minDepth = 0.0f;
		viewport.maxDepth = 1.0f;
		vkCmdSetViewport(commandBuffer, 0, 1, &viewport);

		VkRect2D scissor{};
		scissor.offset = {0, 0};
		scissor.extent = swapChainExtent;
		vkCmdSetScissor(commandBuffer, 0, 1, &scissor);            

		VkBuffer vertexBuffers[] = {vertexBuffer};
		VkDeviceSize offsets[] = {0};
		vkCmdBindVertexBuffers(commandBuffer, 0, 1, vertexBuffers, offsets);

		vkCmdDraw(commandBuffer, static_cast<uint32_t>(vertices.size()), 1, 0, 0);

        vkCmdEndRenderPass(commandBuffer);

        if (vkEndCommandBuffer(commandBuffer) != VK_SUCCESS) {
            throw std::runtime_error("failed to record command buffer!");
        }
    }
*/