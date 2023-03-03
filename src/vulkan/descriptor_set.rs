use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::uniform_buffer::*;

use std::mem::size_of;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_descriptor_set_layout(vk_handle: &mut VkHandle)
{
	let ubo_layout_binding = VkDescriptorSetLayoutBinding{
		binding: 0,
		descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
		descriptorCount: 1,
		stageFlags: VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32,
		pImmutableSamplers: nullptr()
	};

	// Descriptor set
	let descriptor_set_layout_create_info = VkDescriptorSetLayoutCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
		bindingCount: 1,
		pBindings: &ubo_layout_binding,
		flags: 0,	
		pNext: nullptr(),
	};

	match vkCreateDescriptorSetLayout(vk_handle.logical_device, &descriptor_set_layout_create_info, nullptr(), &mut vk_handle.descriptor_set_layout)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateDescriptorSetLayout()"); }
		err => { panic!("✗ vkCreateDescriptorSetLayout() failed with code {:?}.", err); }
	}
}

pub unsafe fn create_descriptor_sets(vk_handle: &mut VkHandle)
{
	let layouts: Vec<VkDescriptorSetLayout> = vec![vk_handle.descriptor_set_layout; vk_handle.frames_in_flight];

	let descriptor_set_allocate_info = 
		VkDescriptorSetAllocateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			descriptorPool: vk_handle.descriptor_pool,
			descriptorSetCount: vk_handle.frames_in_flight as u32,
			pSetLayouts: layouts.as_ptr(),
			pNext: nullptr()
		};

	vk_handle.descriptor_sets = vec![nullptr(); vk_handle.frames_in_flight];

	match vkAllocateDescriptorSets(vk_handle.logical_device, &descriptor_set_allocate_info, vk_handle.descriptor_sets.as_mut_ptr())
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateDescriptorSets()"); }
		err => { panic!("✗ vkAllocateDescriptorSets() failed with code {:?}.", err); }
	}

	for i in 0..(vk_handle.descriptor_sets.len())
	{
		let buffer_info = 
			VkDescriptorBufferInfo {
				buffer: vk_handle.uniform_buffers[i],
				offset: 0,
				range: size_of::<UniformBufferObject>() as u64
			};
		
		let descriptor_write = 
			VkWriteDescriptorSet {
				sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet: vk_handle.descriptor_sets[i],
				dstBinding: 0,
				dstArrayElement: 0,
				descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
				descriptorCount: 1,
				pBufferInfo: &buffer_info,
				pImageInfo: nullptr(),
				pTexelBufferView: nullptr(),
				pNext: nullptr()
			};

		vkUpdateDescriptorSets(vk_handle.logical_device, 1, &descriptor_write, 0, nullptr());
	}
}