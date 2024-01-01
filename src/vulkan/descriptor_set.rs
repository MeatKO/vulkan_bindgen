use crate::detail_core::model::mesh::VulkanMeshData;
use crate::detail_core::texture::texture::Texture;
use crate::detail_core::texture::texture::VulkanTexture;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::uniform_buffer::*;

use std::mem::size_of;
use std::ptr::null_mut as nullptr;

use super::wrappers::vk_descriptor_layout::VkDescriptorLayoutBuilder;

pub unsafe fn create_descriptor_sets(
	vk_handle: &VkHandle,
	descriptor_pool: &VkDescriptorPool,
	descriptor_set_layout: &VkDescriptorSetLayout,
	count: usize
) -> Result<Vec<VkDescriptorSet>, String>
{
	// let descriptor_set_layout = 
	// 	VkDescriptorLayoutBuilder::new()
	// 	.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
	// 	.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
	// 	.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
	// 	.build(vk_handle.logical_device)
	// 	.unwrap();

	let layouts: Vec<VkDescriptorSetLayout> = 
		vec![
			*descriptor_set_layout; 
			// vk_handle.frames_in_flight
			count
		];

	let descriptor_set_allocate_info = 
		VkDescriptorSetAllocateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			descriptorPool: *descriptor_pool,
			// descriptorSetCount: vk_handle.frames_in_flight as u32,
			descriptorSetCount: layouts.len() as u32,
			pSetLayouts: layouts.as_ptr(),
			pNext: nullptr()
		};

	let mut out_descriptor_sets = 
		vec![
			nullptr(); 
			// vk_handle.frames_in_flight // size
			layouts.len()
		];

	match vkAllocateDescriptorSets(vk_handle.logical_device, &descriptor_set_allocate_info, out_descriptor_sets.as_mut_ptr())
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateDescriptorSets()"); }
		err => { panic!("✗ vkAllocateDescriptorSets() failed with code {:?}.", err); }
	}

	Ok(out_descriptor_sets)
}

pub unsafe fn update_descriptor_sets(
	vk_handle: &VkHandle,
	// mesh_data: &mut VulkanMeshData,
	descriptor_sets: &Vec<VkDescriptorSet>,
	albedo_map: &Texture<VulkanTexture>,
	normal_map: &Texture<VulkanTexture>,
) -> Result<(), String>
{
	for i in 0..descriptor_sets.len()
	{
		// let buffer_info = 
		// 	VkDescriptorBufferInfo {
		// 		buffer: mesh_data.uniform_buffers[i],
		// 		offset: 0,
		// 		range: size_of::<UniformBufferObject>() as u64
		// 	};

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
		
		let descriptor_writes = 
			vec![
				// VkWriteDescriptorSet {
				// 	sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				// 	dstSet: mesh_data.descriptor_sets[i],
				// 	dstBinding: 0,
				// 	dstArrayElement: 0,
				// 	descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
				// 	descriptorCount: 1,
				// 	pBufferInfo: &buffer_info,
				// 	pImageInfo: nullptr(),
				// 	pTexelBufferView: nullptr(),
				// 	pNext: nullptr()
				// },
				VkWriteDescriptorSet {
					sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
					dstSet: descriptor_sets[i],
					dstBinding: 0,
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
					dstSet: descriptor_sets[i],
					dstBinding: 1,
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
	}

	Ok(())
}

// pub unsafe fn create_descriptor_sets(
// 	vk_handle: &VkHandle,
// 	mesh_data: &mut VulkanMeshData,
// 	albedo_map: &Texture<VulkanTexture>,
// 	normal_map: &Texture<VulkanTexture>,
// 	descriptor_pool: &VkDescriptorPool,
// ) -> Result<(), String>
// {
// 	let layouts: Vec<VkDescriptorSetLayout> = 
// 		vec![
// 			vk_handle.descriptor_set_layout; 
// 			vk_handle.frames_in_flight // size
// 		];

// 	let descriptor_set_allocate_info = 
// 		VkDescriptorSetAllocateInfo {
// 			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
// 			descriptorPool: *descriptor_pool,
// 			descriptorSetCount: vk_handle.frames_in_flight as u32,
// 			pSetLayouts: layouts.as_ptr(),
// 			pNext: nullptr()
// 		};

// 	mesh_data.descriptor_sets = 
// 		vec![
// 			nullptr(); 
// 			vk_handle.frames_in_flight // size
// 		];

// 	match vkAllocateDescriptorSets(vk_handle.logical_device, &descriptor_set_allocate_info, mesh_data.descriptor_sets.as_mut_ptr())
// 	{
// 		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateDescriptorSets()"); }
// 		err => { panic!("✗ vkAllocateDescriptorSets() failed with code {:?}.", err); }
// 	}

// 	for i in 0..mesh_data.descriptor_sets.len()
// 	{
// 		let buffer_info = 
// 			VkDescriptorBufferInfo {
// 				buffer: mesh_data.uniform_buffers[i],
// 				offset: 0,
// 				range: size_of::<UniformBufferObject>() as u64
// 			};

// 		let albedo_image_info = 
// 			VkDescriptorImageInfo {
// 				imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
// 				imageView: albedo_map.texture_image_view,
// 				sampler: albedo_map.texture_sampler
// 			};

// 		let normal_image_info = 
// 			VkDescriptorImageInfo {
// 				imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
// 				imageView: normal_map.texture_image_view,
// 				sampler: normal_map.texture_sampler
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
// 				VkWriteDescriptorSet {
// 					sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
// 					dstSet: mesh_data.descriptor_sets[i],
// 					dstBinding: 1,
// 					dstArrayElement: 0,
// 					descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
// 					descriptorCount: 1,
// 					pBufferInfo: nullptr(),
// 					pImageInfo: &albedo_image_info,
// 					pTexelBufferView: nullptr(),
// 					pNext: nullptr()
// 				},
// 				VkWriteDescriptorSet {
// 					sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
// 					dstSet: mesh_data.descriptor_sets[i],
// 					dstBinding: 2,
// 					dstArrayElement: 0,
// 					descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
// 					descriptorCount: 1,
// 					pBufferInfo: nullptr(),
// 					pImageInfo: &normal_image_info,
// 					pTexelBufferView: nullptr(),
// 					pNext: nullptr()
// 				},
// 			];

// 		vkUpdateDescriptorSets(vk_handle.logical_device, descriptor_writes.len() as _, descriptor_writes.as_ptr(), 0, nullptr());
// 	}

// 	Ok(())
// }