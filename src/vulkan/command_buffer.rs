use decs::manager::QueryResultMut;

use crate::detail_core::components::rendering::UniformBufferComponent;
use crate::detail_core::model::asset::MaterialAsset;
use crate::detail_core::model::asset::ModelAsset;
use crate::detail_core::model::component::VulkanModelComponent;
use crate::detail_core::model::material::Material;
// use crate::detail_core::model::model::Model;
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::collections::HashMap;
use std::ptr::null_mut as nullptr;
use std::rc::Rc;


pub unsafe fn record_command_buffer(
	vk_handle: &VkHandle, 
	image_index: u32, 
	model_vec: &Vec<(&QueryResultMut<VulkanModelComponent>, &UniformBufferComponent, &AABB)>,
	model_assets_map: &HashMap::<String, Rc<ModelAsset>>,
	material_assets_map: &HashMap::<String, Rc<MaterialAsset>>,
	// default_material: Rc<Material>,
	default_material: Rc<MaterialAsset>,
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

	for (index, (model_component, ubo_component, aabb)) in model_vec.iter().enumerate()
	{
		let model_asset_ref = model_assets_map.get(&model_component.component.model_asset_name).unwrap();

		// println!("model has {} meshes", model_asset_ref.meshes.len());

		for (index, mesh) in model_asset_ref.meshes.iter().enumerate()
		{
			let material_asset_ref = material_assets_map.get(&mesh.material_asset_name).unwrap_or(&default_material);

			let vertex_buffers: Vec<VkBuffer> = 
				vec![
					mesh.mesh_vulkan_buffers.vertex.buffer,
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
				mesh.mesh_vulkan_buffers.index.buffer, 
				0,
				VkIndexType::VK_INDEX_TYPE_UINT32,
			);

			{
				let descriptor_set_vec = 
					vec![
						ubo_component.descriptor_sets[vk_handle.current_frame],
						material_asset_ref.descriptor_set,
					];

				vkCmdBindDescriptorSets(
					current_command_buffer, 
					VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
					vk_handle.pipeline_layout, 
					0, 
					descriptor_set_vec.len() as _,
					descriptor_set_vec.as_ptr(),
					0, 
					nullptr()
				);
			}

			vkCmdDrawIndexed(
				current_command_buffer,
				mesh.mesh_vulkan_buffers.index_count as _,
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