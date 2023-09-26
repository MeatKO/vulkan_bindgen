use crate::exedra::model::Model;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::uniform_buffer::*;

use std::mem::size_of;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_descriptor_set_layout(
	vk_handle: &mut VkHandle,
	// model: &mut Model
)
{
	let ubo_layout_binding = 
		VkDescriptorSetLayoutBinding{
			binding: 0,
			descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
			descriptorCount: 1,
			stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32,
			pImmutableSamplers: nullptr()
		};

	let sampler_layout_binding = 
		VkDescriptorSetLayoutBinding{
			binding: 1,
			descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
			descriptorCount: 1,
			stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32,
			pImmutableSamplers: nullptr()
		};

	let bindings = 
		vec![
			ubo_layout_binding, 
			sampler_layout_binding
		];

	let descriptor_set_layout_create_info = 
		VkDescriptorSetLayoutCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			bindingCount: bindings.len() as u32,
			pBindings: bindings.as_ptr(),
			flags: 0,	
			pNext: nullptr(),
		};

	match vkCreateDescriptorSetLayout(vk_handle.logical_device, &descriptor_set_layout_create_info, nullptr(), &mut vk_handle.descriptor_set_layout)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateDescriptorSetLayout()"); }
		err => { panic!("✗ vkCreateDescriptorSetLayout() failed with code {:?}.", err); }
	}
}

pub unsafe fn create_descriptor_sets(
	vk_handle: &mut VkHandle,
	model: &mut Model
)
{
	let layouts: Vec<VkDescriptorSetLayout> = 
		vec![
			vk_handle.descriptor_set_layout; 
			vk_handle.frames_in_flight
		];

	let descriptor_set_allocate_info = 
		VkDescriptorSetAllocateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			descriptorPool: model.descriptor_pool,
			descriptorSetCount: vk_handle.frames_in_flight as u32,
			pSetLayouts: layouts.as_ptr(),
			pNext: nullptr()
		};

	model.descriptor_sets = vec![nullptr(); vk_handle.frames_in_flight];

	match vkAllocateDescriptorSets(vk_handle.logical_device, &descriptor_set_allocate_info, model.descriptor_sets.as_mut_ptr())
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateDescriptorSets()"); }
		err => { panic!("✗ vkAllocateDescriptorSets() failed with code {:?}.", err); }
	}

	for i in 0..model.descriptor_sets.len()
	{
		let buffer_info = 
			VkDescriptorBufferInfo {
				buffer: model.uniform_buffers[i],
				offset: 0,
				range: size_of::<UniformBufferObject>() as u64
			};

		let image_info = 
			VkDescriptorImageInfo {
				imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
				imageView: model.texture_image_view,
				sampler: model.texture_sampler
			};
		
		let descriptor_writes = 
			vec![
				VkWriteDescriptorSet {
					sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
					dstSet: model.descriptor_sets[i],
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
					dstSet: model.descriptor_sets[i],
					dstBinding: 1,
					dstArrayElement: 0,
					descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
					descriptorCount: 1,
					pBufferInfo: nullptr(),
					pImageInfo: &image_info,
					pTexelBufferView: nullptr(),
					pNext: nullptr()
				},
			];

		vkUpdateDescriptorSets(vk_handle.logical_device, descriptor_writes.len() as _, descriptor_writes.as_ptr(), 0, nullptr());
	}
}