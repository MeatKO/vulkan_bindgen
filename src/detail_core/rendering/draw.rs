use decs::component::AsAny;
use decs::component_derive::system;
use decs::manager::{dECS, QueryResult, QueryResultMut};

use crate::cotangens::vec3::Vec3;
use crate::detail_core::components::rendering::{ModelComponent, UniformBufferComponent};
use crate::detail_core::model::asset::ModelAsset;
use crate::detail_core::model::component::VulkanModelComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::{Model, VulkanModel};
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::command_buffer::{record_command_buffer_single};
use crate::vulkan::command_buffer_wireframe::record_command_buffer_wireframe_ref;
use crate::vulkan::handle::VkHandle;
use crate::vulkan::swapchain::recreate_swapchain;
use crate::vulkan::uniform_buffer::{update_uniform_buffer, UniformBufferObject};
use crate::vulkan::uniform_buffer_wireframe::update_uniform_buffer_wireframe;
use crate::vulkan::vk_bindgen::{vkWaitForFences, vkResetFences, vkAcquireNextImageKHR, VkResult, VK_TRUE, VkPipelineStageFlags, VkPipelineStageFlagBits, VkSubmitInfo, VkStructureType, vkQueueSubmit, VkPresentInfoKHR, vkDeviceWaitIdle, vkQueuePresentKHR, VkDescriptorBufferInfo, VkWriteDescriptorSet, VkDescriptorType, vkUpdateDescriptorSets};

use std::collections::HashMap;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;
use std::rc::Rc;

// #[system]
// pub fn rendering_system3()
// {
// 	unsafe
// 	{
// 		let vk_handle: &mut VkHandle =
// 			unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

// 		let default_albedo_map =
// 			decs.get_asset_rc::<Material>("material_defaults").unwrap();

// 		vkWaitForFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame], VK_TRUE, u64::MAX);
// 		vkResetFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame]);

// 		let mut image_index = 0u32;
// 		match vkAcquireNextImageKHR(vk_handle.logical_device, vk_handle.swapchain, u64::MAX, vk_handle.image_available_semaphore_vec[vk_handle.current_frame], nullptr(), &mut image_index)
// 		{
// 			VkResult::VK_SUCCESS => {}
// 			VkResult::VK_ERROR_OUT_OF_DATE_KHR => 
// 			{
// 				recreate_swapchain(vk_handle); 
// 			}
// 			e => { panic!("vkAcquireNextImageKHR() resulted in {:?}", e) }
// 		}

// 		vk_handle.command_buffer_vec[vk_handle.current_frame].reset(None);

// 		let error_model: Rc<Model<VulkanModel>> = decs.get_asset_rc::<Model<VulkanModel>>("error_model").unwrap();

// 		'model_rendering:
// 		{
// 			let models: Vec<QueryResult<ModelComponent>> =
// 				match decs.get_components_global_unchecked::<ModelComponent>()
// 				{
// 					Ok(model_vec) => 
// 					{
// 						model_vec
// 					}
// 					Err(_) => { println!("no models to render"); break 'model_rendering }
// 				};

// 			let mut model_aabb_ubo_pairs = vec![];
// 			let aabb_empty = AABB::new_empty();

// 			for model in models.iter()
// 			{
// 				// check if we point to a valid model first 
// 				// no idea how we're gonna remove invalid pointers later rofl
// 				let model_component = 
// 					match model.component.model_asset.upgrade()
// 					{
// 						Some(model) => { model }
// 						None => { error_model.clone() }
// 					};

// 				let aabb = 
// 					match decs.get_components::<AABB>(model.entity_id)
// 					{
// 						Ok(aabb_vec) => 
// 						{
// 							match aabb_vec.first()
// 							{
// 								Some(aabb) => { aabb }
// 								None => { &aabb_empty }
// 							}
// 						},
// 						Err(_) => 
// 						{
// 							&aabb_empty
// 						}
// 					};

// 				let ubo = 
// 					decs.get_components::<UniformBufferComponent>(model.entity_id).unwrap().remove(0);

// 				model_aabb_ubo_pairs.push((aabb, model_component, ubo))
// 			}

// 			for (index,(aabb, model, ubo_component)) in model_aabb_ubo_pairs.iter().enumerate()
// 			{
// 				for mesh in &model.meshes
// 				{
// 					let vulkan_data = 
// 						match &mesh.vulkan_data
// 						{
// 							Some(vd) => vd,
// 							None => continue
// 						};

// 					update_uniform_buffer(
// 						vk_handle, 
// 						ubo_component.uniform_buffers_mapped[vk_handle.current_frame],
// 						&aabb.scale, 
// 						&aabb.translation, 
// 						&Vec3::new(0.0f32),
// 						&Vec3::new(0.0f32)
// 					);
// 					// update_uniform_buffer(
// 					// 	vk_handle, 
// 					// 	vulkan_data, 
// 					// 	index, 
// 					// 	&aabb.scale, 
// 					// 	&aabb.translation, 
// 					// 	&Vec3::new(0.0f32),
// 					// 	&Vec3::new(0.0f32)
// 					// );
// 				}
// 			}
// 			record_command_buffer_ref(vk_handle, image_index, &model_aabb_ubo_pairs, default_albedo_map);
// 		}

// 		'physbox_rendering:
// 		{
// 			let aabb_vec: Vec<QueryResult<AABB>> =
// 				match decs.get_components_global_unchecked::<AABB>()
// 				{
// 					Ok(model_vec) => 
// 					{
// 						model_vec
// 					}
// 					Err(_) => { println!("no physboxes to render"); break 'physbox_rendering }
// 				};

// 			let aabb_vec = aabb_vec.iter().map(|result| result.component).collect::<Vec<&AABB>>();

// 			for aabb in aabb_vec.iter()
// 			{
// 				update_uniform_buffer_wireframe(
// 					vk_handle, 
// 					aabb.aabb_vulkan_data.as_ref().unwrap(), 
// 					&aabb.scale,
// 					// &model.translation, 
// 					&aabb.translation, 
// 					&Vec3::new(0.0f32), 
// 					&aabb.color,
// 				);
// 			}

// 			record_command_buffer_wireframe_ref(vk_handle, image_index, &aabb_vec);
// 		}

// 		let wait_semaphore_vec = vec![vk_handle.image_available_semaphore_vec[vk_handle.current_frame]];
// 		let wait_stages_vec : Vec<VkPipelineStageFlags> = vec![VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32];
// 		let signal_semaphores_vec = vec![vk_handle.rendering_finished_semaphore_vec[vk_handle.current_frame]];

// 		let command_buffers = 
// 			vec![
// 				vk_handle.command_buffer_vec[vk_handle.current_frame].get_command_buffer_ptr(),
// 				vk_handle.command_buffer_wireframe_vec[vk_handle.current_frame].get_command_buffer_ptr(),
// 			];

// 		let submit_info = 
// 			VkSubmitInfo{
// 				sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
// 				waitSemaphoreCount: 1,
// 				pWaitSemaphores: wait_semaphore_vec.as_ptr(),
// 				pWaitDstStageMask: wait_stages_vec.as_ptr(),
// 				commandBufferCount: command_buffers.len() as _,
// 				pCommandBuffers: command_buffers.as_ptr(),
// 				signalSemaphoreCount: 1,
// 				pSignalSemaphores: signal_semaphores_vec.as_ptr(),
// 				pNext: nullptr(),
// 			};

// 		match vkQueueSubmit(vk_handle.graphics_queue, 1, &submit_info, vk_handle.in_flight_fence_vec[vk_handle.current_frame])
// 		{
// 			VkResult::VK_SUCCESS => {}
// 			err => { panic!("✗ vkQueueSubmit() failed with code {:?}.", err); }
// 		}

// 		let swapchain_vec = vec![vk_handle.swapchain];

// 		let present_info = 
// 			VkPresentInfoKHR{
// 				sType: VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
// 				waitSemaphoreCount: 1,
// 				pWaitSemaphores: signal_semaphores_vec.as_ptr(),
// 				swapchainCount: 1,
// 				pSwapchains: swapchain_vec.as_ptr(),
// 				pImageIndices: &image_index,
// 				pResults: nullptr(),
// 				pNext: nullptr()
// 			};


// 		match vkDeviceWaitIdle(vk_handle.logical_device)
// 		{
// 			VkResult::VK_SUCCESS => {} 
// 			// VkResult::VK_ERROR_DEVICE_LOST => 
// 			_ => { panic!("sheeeit") }
// 		}

// 		match vkQueuePresentKHR(vk_handle.presentation_queue, &present_info)
// 		{
// 			VkResult::VK_SUCCESS => {}
// 			VkResult::VK_ERROR_OUT_OF_DATE_KHR => { println!("vkQueuePresentKHR() out of date - recreating"); recreate_swapchain(vk_handle) }
// 			e => { panic!("vkQueuePresentKHR() resulted in {:?}", e) }
// 		}

// 		vk_handle.current_frame = (vk_handle.current_frame + 1) % vk_handle.frames_in_flight;
// 	}
// }

#[system]
pub fn rendering_system4()
{
	unsafe
	{
		let vk_handle: &mut VkHandle =
			unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

		let default_material =
			decs.get_asset_rc::<Material>("material_defaults").unwrap();

		vkWaitForFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame], VK_TRUE, u64::MAX);
		vkResetFences(vk_handle.logical_device, 1, &vk_handle.in_flight_fence_vec[vk_handle.current_frame]);

		let mut image_index = 0u32;
		match vkAcquireNextImageKHR(vk_handle.logical_device, vk_handle.swapchain, u64::MAX, vk_handle.image_available_semaphore_vec[vk_handle.current_frame], nullptr(), &mut image_index)
		{
			VkResult::VK_SUCCESS => {}
			VkResult::VK_ERROR_OUT_OF_DATE_KHR => 
			{
				recreate_swapchain(vk_handle); 
			}
			e => { panic!("vkAcquireNextImageKHR() resulted in {:?}", e) }
		}

		vk_handle.command_buffer_vec[vk_handle.current_frame].reset(None);

		'model_rendering:
		{
			let mut used_model_assets = HashMap::<String, Rc<ModelAsset>>::new();
			let default_material = 
				decs.get_asset_rc::<Material>("material_defaults").unwrap();

			// we need to append the UBO of the same entity to the model component
			// and get the resulting vector of tuples (model, ubo)
			let model_components = decs.get_components_global_mut_unchecked::<VulkanModelComponent>().unwrap();
			let models = 
				model_components
				.iter()
				.filter(
					|model_component| 
					!model_component.component.model_asset_name.is_empty()
				)
				.filter(
					|model_component| 
					match decs.get_components::<UniformBufferComponent>(model_component.entity_id)
					{
						Ok(vec) => !vec.is_empty(),
						Err(_) => false
					}
				)
				.map(
					|model_component| 
					{
						let ubo = 
							decs.get_components::<UniformBufferComponent>(model_component.entity_id).unwrap().remove(0);

						(model_component, ubo)
					}
				)
				.map(
					|(model_component, ubo)| 
					{
						let aabb = 
							decs.get_components::<AABB>(model_component.entity_id).unwrap().remove(0);

						(model_component, ubo, aabb)
					}
				)
				.collect::<Vec<(&QueryResultMut<VulkanModelComponent>, &UniformBufferComponent, &AABB)>>();

			for (index, (model_component, ubo_component, aabb)) in models.iter().enumerate()
			{
				let model_asset_ref =
					decs.get_asset_rc::<ModelAsset>(&model_component.component.model_asset_name).unwrap();
						
				used_model_assets.insert(model_component.component.model_asset_name.clone(), model_asset_ref.clone());
						
				update_uniform_buffer(
					vk_handle, 
					ubo_component.uniform_buffers_mapped[vk_handle.current_frame],
					&aabb.scale,
					// &model.translation, 
					&aabb.translation, 
					&Vec3::new(0.0f32), 
					&aabb.color,
				);

				{
					let buffer_info = 
						VkDescriptorBufferInfo {
							buffer: ubo_component.uniform_buffers[vk_handle.current_frame],
							offset: 0,
							range: size_of::<UniformBufferObject>() as u64
						};

					// default_material
					let descriptor_writes = 
						vec![
							VkWriteDescriptorSet {
								sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
								// dstSet: vk_handle.global_mesh_descriptor_sets[vk_handle.current_frame],
								dstSet: ubo_component.descriptor_sets[vk_handle.current_frame],
								dstBinding: 0,
								dstArrayElement: 0,
								descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
								descriptorCount: 1,
								pBufferInfo: &buffer_info,
								pImageInfo: nullptr(),
								pTexelBufferView: nullptr(),
								pNext: nullptr()
							},
						];

					vkUpdateDescriptorSets(vk_handle.logical_device, descriptor_writes.len() as _, descriptor_writes.as_ptr(), 0, nullptr());
		
				}
			}

			// println!("total number of models to draw : {}", models.len());

			record_command_buffer_single(vk_handle, image_index, &models, &used_model_assets, default_material);
		}

		'physbox_rendering:
		{
			let aabb_vec: Vec<QueryResult<AABB>> =
				match decs.get_components_global_unchecked::<AABB>()
				{
					Ok(model_vec) => 
					{
						model_vec
					}
					Err(_) => { println!("no physboxes to render"); break 'physbox_rendering }
				};

			let aabb_vec = aabb_vec.iter().map(|result| result.component).collect::<Vec<&AABB>>();

			for aabb in aabb_vec.iter()
			{
				update_uniform_buffer_wireframe(
					vk_handle, 
					aabb.aabb_vulkan_data.as_ref().unwrap(), 
					&aabb.scale,
					// &model.translation, 
					&aabb.translation, 
					&Vec3::new(0.0f32), 
					&aabb.color,
				);
			}

			// println!("total number of physboxes to draw : {}", aabb_vec.len());

			record_command_buffer_wireframe_ref(vk_handle, image_index, &aabb_vec);
		}

		let wait_semaphore_vec = vec![vk_handle.image_available_semaphore_vec[vk_handle.current_frame]];
		let wait_stages_vec : Vec<VkPipelineStageFlags> = vec![VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32];
		let signal_semaphores_vec = vec![vk_handle.rendering_finished_semaphore_vec[vk_handle.current_frame]];

		let command_buffers = 
			vec![
				vk_handle.command_buffer_vec[vk_handle.current_frame].get_command_buffer_ptr(),
				vk_handle.command_buffer_wireframe_vec[vk_handle.current_frame].get_command_buffer_ptr(),
			];

		let submit_info = 
			VkSubmitInfo{
				sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
				waitSemaphoreCount: 1,
				pWaitSemaphores: wait_semaphore_vec.as_ptr(),
				pWaitDstStageMask: wait_stages_vec.as_ptr(),
				commandBufferCount: command_buffers.len() as _,
				pCommandBuffers: command_buffers.as_ptr(),
				signalSemaphoreCount: 1,
				pSignalSemaphores: signal_semaphores_vec.as_ptr(),
				pNext: nullptr(),
			};

		match vkQueueSubmit(vk_handle.graphics_queue, 1, &submit_info, vk_handle.in_flight_fence_vec[vk_handle.current_frame])
		{
			VkResult::VK_SUCCESS => {}
			err => { panic!("✗ vkQueueSubmit() failed with code {:?}.", err); }
		}

		let swapchain_vec = vec![vk_handle.swapchain];

		let present_info = 
			VkPresentInfoKHR{
				sType: VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
				waitSemaphoreCount: 1,
				pWaitSemaphores: signal_semaphores_vec.as_ptr(),
				swapchainCount: 1,
				pSwapchains: swapchain_vec.as_ptr(),
				pImageIndices: &image_index,
				pResults: nullptr(),
				pNext: nullptr()
			};


		match vkDeviceWaitIdle(vk_handle.logical_device)
		{
			VkResult::VK_SUCCESS => {} 
			// VkResult::VK_ERROR_DEVICE_LOST => 
			_ => { panic!("sheeeit") }
		}

		match vkQueuePresentKHR(vk_handle.presentation_queue, &present_info)
		{
			VkResult::VK_SUCCESS => {}
			VkResult::VK_ERROR_OUT_OF_DATE_KHR => { println!("vkQueuePresentKHR() out of date - recreating"); recreate_swapchain(vk_handle) }
			e => { panic!("vkQueuePresentKHR() resulted in {:?}", e) }
		}

		vk_handle.current_frame = (vk_handle.current_frame + 1) % vk_handle.frames_in_flight;
	}
}
