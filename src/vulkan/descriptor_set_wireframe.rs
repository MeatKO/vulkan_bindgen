use crate::detail_core::phys::aabb::VulkanMeshData;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;

use std::mem::size_of;
use std::ptr::null_mut as nullptr;

use super::uniform_buffer_wireframe::UniformBufferObjectWireframe;
use super::wrappers::vk_descriptor_layout::VkDescriptorLayoutBuilder;

pub unsafe fn create_descriptor_sets_wireframe(
	vk_handle: &VkHandle,
	mesh_data: &mut VulkanMeshData,
	// descriptor_pool: &VkDescriptorPool,
) -> Result<(), String>
{
	let descriptor_set_layout = 
		VkDescriptorLayoutBuilder::new()
		.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
		.build(vk_handle.logical_device)
		.unwrap();

	let layouts: Vec<VkDescriptorSetLayout> = 
		vec![
			// vk_handle.descriptor_set_layout;
			descriptor_set_layout; 
			vk_handle.frames_in_flight
		];

	let descriptor_set_allocate_info = 
		VkDescriptorSetAllocateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			descriptorPool: mesh_data.descriptor_pool,
			descriptorSetCount: vk_handle.frames_in_flight as u32,
			pSetLayouts: layouts.as_ptr(),
			pNext: nullptr()
		};

	mesh_data.descriptor_sets = vec![nullptr(); vk_handle.frames_in_flight];

	match vkAllocateDescriptorSets(vk_handle.logical_device, &descriptor_set_allocate_info, mesh_data.descriptor_sets.as_mut_ptr())
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateDescriptorSets() WIREFRAME"); }
		err => { panic!("✗ vkAllocateDescriptorSets() WIREFRAME failed with code {:?}.", err); }
	}

	for i in 0..mesh_data.descriptor_sets.len()
	{
		let buffer_info = 
			VkDescriptorBufferInfo {
				buffer: mesh_data.uniform_buffers[i],
				offset: 0,
				range: size_of::<UniformBufferObjectWireframe>() as u64
				// range: size_of::<UniformBufferObjectHUD>() as u64
			};
		
		let descriptor_writes = 
			vec![
				VkWriteDescriptorSet {
					sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
					dstSet: mesh_data.descriptor_sets[i],
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

	Ok(())
}