use crate::detail_core::components::rendering::UniformBufferComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::Model;
use crate::detail_core::model::model::VulkanModel;
use crate::detail_core::phys::aabb::AABB;
use crate::detail_core::texture::texture::Texture;
use crate::detail_core::texture::texture::VulkanTexture;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;
use std::rc::Rc;

use super::descriptor_set::update_descriptor_sets;
use super::uniform_buffer::UniformBufferObject;

pub unsafe fn record_command_buffer_ref(
	vk_handle: &VkHandle, 
	image_index: u32, 
	models: &Vec<(&AABB, Rc<Model<VulkanModel>>, &UniformBufferComponent)>,
	default_material: Rc<Material>,
)
{
	let current_command_buffer = vk_handle.command_buffer_vec[vk_handle.current_frame].get_command_buffer_ptr();

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
					float32: [0.3f32, 0.5f32, 0.4f32, 1.0f32]
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
			renderPass: vk_handle.render_pass,
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
		vk_handle.graphics_pipeline
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
		for mesh in model.1.meshes.iter()
		{
			let vulkan_data = 
				match &mesh.vulkan_data
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

			{
				let default_albedo_map = default_material.albedo_map.clone().unwrap();
				let default_normal_map = default_material.normal_map.clone().unwrap();

				let albedo_map = mesh.material.albedo_map.clone().unwrap_or(default_albedo_map);
				let normal_map = mesh.material.albedo_map.clone().unwrap_or(default_normal_map);

				let albedo_image_info = 
					VkDescriptorImageInfo {
						imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
						imageView: albedo_map.texture_image_view,
						sampler: albedo_map.texture_sampler
					};

				let normal_image_info = 
					VkDescriptorImageInfo {
						imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
						imageView: normal_map.texture_image_view,
						sampler: normal_map.texture_sampler
				};

				let buffer_info = 
					VkDescriptorBufferInfo {
						// buffer: vulkan_data.uniform_buffers[vk_handle.current_frame],
						buffer: model.2.uniform_buffers[vk_handle.current_frame],
						offset: 0,
						range: size_of::<UniformBufferObject>() as u64
					};

				let descriptor_writes = 
					vec![
						VkWriteDescriptorSet {
							sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
							dstSet: vk_handle.global_mesh_descriptor_sets[vk_handle.current_frame],
							dstBinding: 0,
							dstArrayElement: 0,
							descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
							descriptorCount: 1,
							pBufferInfo: &buffer_info,
							pImageInfo: nullptr(),
							pTexelBufferView: nullptr(),
							pNext: nullptr()
						},
						VkWriteDescriptorSet {
							sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
							dstSet: vk_handle.global_mesh_descriptor_sets[vk_handle.current_frame],
							dstBinding: 1,
							dstArrayElement: 0,
							descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
							descriptorCount: 1,
							pBufferInfo: nullptr(),
							pImageInfo: &albedo_image_info,
							pTexelBufferView: nullptr(),
							pNext: nullptr()
						},
						VkWriteDescriptorSet {
							sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
							dstSet: vk_handle.global_mesh_descriptor_sets[vk_handle.current_frame],
							dstBinding: 2,
							dstArrayElement: 0,
							descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
							descriptorCount: 1,
							pBufferInfo: nullptr(),
							pImageInfo: &normal_image_info,
							pTexelBufferView: nullptr(),
							pNext: nullptr()
						},
					];

				vkUpdateDescriptorSets(vk_handle.logical_device, descriptor_writes.len() as _, descriptor_writes.as_ptr(), 0, nullptr());
			
				vkCmdBindDescriptorSets(
					current_command_buffer, 
					VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
					vk_handle.pipeline_layout, 
					0, 
					1, 
					&vk_handle.global_mesh_descriptor_sets[vk_handle.current_frame],
					0, 
					nullptr()
				);
			}
	
			vkCmdDrawIndexed(
				current_command_buffer,
				mesh.index_count,
				1, 
				0, 
				0, 
				0
			);
		}
	}
	
	vkCmdEndRenderPass(current_command_buffer);

	match vkEndCommandBuffer(current_command_buffer)
	{
		VkResult::VK_SUCCESS => {  }
		err => { panic!("✗ vkEndCommandBuffer() failed with code {:?}.", err); }
	}
}