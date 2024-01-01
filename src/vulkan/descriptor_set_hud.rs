// use crate::detail_core::ui::button::VulkanButtonData;
// use crate::vulkan::vk_bindgen::*;
// use crate::vulkan::handle::*;

// use std::mem::size_of;
// use std::ptr::null_mut as nullptr;

// use super::uniform_buffer_hud::UniformBufferObjectHUD;
// use super::wrappers::vk_descriptor_layout::VkDescriptorLayoutBuilder;

// // pub unsafe fn create_descriptor_set_layout_hud(
// // 	vk_handle: &mut VkHandle,
// // 	// model: &mut Model
// // )
// // {
// // 	let ubo_layout_binding = 
// // 		VkDescriptorSetLayoutBinding{
// // 			binding: 0,
// // 			descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
// // 			descriptorCount: 1,
// // 			stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32,
// // 			pImmutableSamplers: nullptr()
// // 		};

// // 	let albedo_layout_binding = 
// // 		VkDescriptorSetLayoutBinding{
// // 			binding: 1,
// // 			descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
// // 			descriptorCount: 1,
// // 			stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32,
// // 			pImmutableSamplers: nullptr()
// // 		};

// // 	let normal_layout_binding = 
// // 		VkDescriptorSetLayoutBinding{
// // 			binding: 2,
// // 			descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
// // 			descriptorCount: 1,
// // 			stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32,
// // 			pImmutableSamplers: nullptr()
// // 		};


// // 	let bindings = 
// // 		vec![
// // 			ubo_layout_binding, 
// // 			albedo_layout_binding,
// // 			normal_layout_binding
// // 		];

// // 	let descriptor_set_layout_create_info = 
// // 		VkDescriptorSetLayoutCreateInfo{
// // 			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
// // 			bindingCount: bindings.len() as u32,
// // 			pBindings: bindings.as_ptr(),
// // 			flags: 0,	
// // 			pNext: nullptr(),
// // 		};

// // 	match vkCreateDescriptorSetLayout(vk_handle.logical_device, &descriptor_set_layout_create_info, nullptr(), &mut vk_handle.descriptor_set_layout)
// // 	{
// // 		VkResult::VK_SUCCESS => { println!("✔️ vkCreateDescriptorSetLayout()"); }
// // 		err => { panic!("✗ vkCreateDescriptorSetLayout() failed with code {:?}.", err); }
// // 	}
// // }

// pub unsafe fn create_descriptor_sets_hud(
// 	vk_handle: &VkHandle,
// 	mesh_data: &mut VulkanButtonData,
// 	descriptor_pool: &VkDescriptorPool,
// ) -> Result<(), String>
// {
// 	let descriptor_set_layout = 
// 		VkDescriptorLayoutBuilder::new()
// 		.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
// 		.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
// 		.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
// 		.build(vk_handle.logical_device)
// 		.unwrap();

// 	let layouts: Vec<VkDescriptorSetLayout> = 
// 		vec![
// 			// vk_handle.descriptor_set_layout;
// 			descriptor_set_layout; 
// 			vk_handle.frames_in_flight
// 		];

// 	let descriptor_set_allocate_info = 
// 		VkDescriptorSetAllocateInfo {
// 			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
// 			descriptorPool: *descriptor_pool,
// 			descriptorSetCount: vk_handle.frames_in_flight as u32,
// 			pSetLayouts: layouts.as_ptr(),
// 			pNext: nullptr()
// 		};

// 	mesh_data.descriptor_sets = vec![nullptr(); vk_handle.frames_in_flight];

// 	match vkAllocateDescriptorSets(vk_handle.logical_device, &descriptor_set_allocate_info, mesh_data.descriptor_sets.as_mut_ptr())
// 	{
// 		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateDescriptorSets() HUD"); }
// 		err => { panic!("✗ vkAllocateDescriptorSets() HUD failed with code {:?}.", err); }
// 	}

// 	for i in 0..mesh_data.descriptor_sets.len()
// 	{
// 		let buffer_info = 
// 			VkDescriptorBufferInfo {
// 				buffer: mesh_data.uniform_buffers[i],
// 				offset: 0,
// 				// range: size_of::<UniformBufferObject>() as u64
// 				range: size_of::<UniformBufferObjectHUD>() as u64
// 			};
		
// 		let descriptor_writes = 
// 			vec![
// 				VkWriteDescriptorSet {
// 					sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
// 					dstSet: mesh_data.descriptor_sets[i],
// 					dstBinding: 0,
// 					dstArrayElement: 0,
// 					descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
// 					descriptorCount: 1,
// 					pBufferInfo: &buffer_info,
// 					pImageInfo: nullptr(),
// 					pTexelBufferView: nullptr(),
// 					pNext: nullptr()
// 				},
// 			];

// 		vkUpdateDescriptorSets(vk_handle.logical_device, descriptor_writes.len() as _, descriptor_writes.as_ptr(), 0, nullptr());
// 	}

// 	Ok(())
// }