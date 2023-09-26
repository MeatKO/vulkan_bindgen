use crate::exedra::model::Model;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::vk_buffer::*;
use crate::vulkan::depth_buffer::*;
use crate::vulkan::vk_memory::find_memory_type;

use crate::pixcell::tga::*;

use std::ptr::null_mut as nullptr;

enum ImageLayoutTransition 
{
    UndefinedToTransferDst,
    UndefinedToDepthStencilAttachment,
    TransferDstToShaderReadOnly,
}

impl ImageLayoutTransition 
{
    fn from_layouts(old_layout: VkImageLayout, new_layout: VkImageLayout) -> Option<Self> 
	{
        match (old_layout, new_layout) 
		{
            (VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED, VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL) => 
				Some(Self::UndefinedToTransferDst),

            (VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED, VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL) => 
				Some(Self::UndefinedToDepthStencilAttachment),

            (VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL) => 
				Some(Self::TransferDstToShaderReadOnly),
				
            _ => None
        }
    }
}

pub unsafe fn create_texture_image(
	vk_handle: &mut VkHandle,
	model: &mut Model,
	tga_image_path: String
)
{
	// let image = TGAImage::new("./detail/textures/test_marked.tga").unwrap();
	// let image = TGAImage::new("./detail/models/viking_room/viking_room.tga").unwrap();
	let image = TGAImage::new(tga_image_path).unwrap();

	let (staging_buffer, staging_buffer_memory) = 
		match create_buffer(
				vk_handle, 
				image.data.len() as u64,
				VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32,
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
			)
		{
			Ok(tuple) => { tuple }
			Err(e) => { panic!("Couldn't create index staging buffer - {}", e) }
		};

	let mut data = nullptr();
	vkMapMemory(vk_handle.logical_device, staging_buffer_memory, 0, image.data.len() as u64, 0, &mut data);
	std::ptr::copy_nonoverlapping(image.data.as_ptr() as _, data as _, image.data.len());
	vkUnmapMemory(vk_handle.logical_device, staging_buffer_memory);

	let (vk_image, vk_image_memory) = 
		create_image(
			vk_handle,
			image.header.width as u32, 
			image.header.height as u32, 
			VkFormat::VK_FORMAT_R8G8B8A8_SRGB,
			VkImageTiling::VK_IMAGE_TILING_OPTIMAL, 
			VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32 | 
			VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as u32, 
			VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		);
	
	model.texture_image = vk_image;
	model.texture_image_memory = vk_image_memory;

	transition_image_layout(
		vk_handle, 
		VkFormat::VK_FORMAT_R8G8B8A8_SRGB, 
		model.texture_image, 
		VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED, 
		VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL
	);
	copy_buffer_to_image(vk_handle, staging_buffer, model.texture_image, image.header.width as u32, image.header.height as u32);
	transition_image_layout(
		vk_handle, 
		VkFormat::VK_FORMAT_R8G8B8A8_SRGB, 
		model.texture_image, 
		VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 
		VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
	);

	vkDestroyBuffer(vk_handle.logical_device, staging_buffer, nullptr());
	vkFreeMemory(vk_handle.logical_device, staging_buffer_memory, nullptr());
}

pub unsafe fn create_image(
	vk_handle: &mut VkHandle,
	width: u32,
	height: u32,
	format: VkFormat,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	properties: VkMemoryPropertyFlags,
) -> (VkImage, VkDeviceMemory)
{
	let image_create_info = 
		VkImageCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
			imageType: VkImageType::VK_IMAGE_TYPE_2D,
			extent: VkExtent3D { 
				width: width, 
				height: height, 
				depth: 1
			},
			mipLevels: 1,
			arrayLayers: 1,
			format: format,
			tiling: tiling,
			initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
			usage: usage,
			samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
			sharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices: nullptr(),
			flags: 0,	
			pNext: nullptr(),
		};

	let mut image: VkImage = nullptr();
	let mut image_memory: VkDeviceMemory = nullptr();

	match vkCreateImage(vk_handle.logical_device, &image_create_info, nullptr(), &mut image)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateImage()"); }
		err => { panic!("✗ vkCreateImage() failed with code {:?}.", err); }
	}	

	let mut memory_requirements: VkMemoryRequirements = std::mem::zeroed();
	vkGetImageMemoryRequirements(vk_handle.logical_device, image, &mut memory_requirements);

	let memory_allocate_info = 
		VkMemoryAllocateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
			allocationSize: memory_requirements.size,
			memoryTypeIndex: find_memory_type(
				vk_handle.physical_device, 
				memory_requirements.memoryTypeBits, 
				properties
			).unwrap(),
			pNext: nullptr(),
		};

	// replace with dt_memory func
	match vkAllocateMemory(vk_handle.logical_device, &memory_allocate_info, nullptr(), &mut image_memory)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkAllocateMemory() for texture"); }
		err => { panic!("vkAllocateMemory() failed with code {:?}", err) }
	}

	vkBindImageMemory(vk_handle.logical_device, image, image_memory, 0);

	(image, image_memory)
}

pub unsafe fn transition_image_layout(
	vk_handle: &VkHandle,
	format: VkFormat,
	image: VkImage,
	old_layout: VkImageLayout,
	new_layout: VkImageLayout
)
{
	let command_buffer = begin_single_time_commands(vk_handle).unwrap();

	let mut barrier = 
		VkImageMemoryBarrier{
			sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
			oldLayout: old_layout,
			newLayout: new_layout,
			srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED as u32,
			dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED as u32,
			image: image,
			subresourceRange: 
				VkImageSubresourceRange {
					aspectMask: 0, // TBD
					baseMipLevel: 0,
					levelCount: 1,
					baseArrayLayer: 0,
					layerCount: 1
				},
			srcAccessMask: 0, // TBD
			dstAccessMask: 0, // TBD
			pNext: nullptr(),
		};

	barrier.subresourceRange.aspectMask =
		match new_layout
		{
			VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => 
			{
				if has_stencil_component(&format)
				{
					VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32 | 
					VkImageAspectFlagBits::VK_IMAGE_ASPECT_STENCIL_BIT as u32
				}
				else
				{
					VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32
				}
			},
			_ => { VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32 }
		};

	let (source_access_mask, destination_access_mask) = 
		match ImageLayoutTransition::from_layouts(old_layout, new_layout) 
		{
			Some(ImageLayoutTransition::UndefinedToTransferDst) => (
				0 as u32,
				VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32
			),
			Some(ImageLayoutTransition::UndefinedToDepthStencilAttachment) => (
				0 as u32,
				(VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT as u32 | 
				VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT as u32)
			),
			Some(ImageLayoutTransition::TransferDstToShaderReadOnly) => (
				VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32,
				VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as u32
			),
			_ => panic!("unsupported layout transition {:?} -> {:?}", old_layout, new_layout)
		};

	barrier.srcAccessMask = source_access_mask;
	barrier.dstAccessMask = destination_access_mask;

	let (source_stage, destination_stage) = 
		match ImageLayoutTransition::from_layouts(old_layout, new_layout) 
		{
			Some(ImageLayoutTransition::UndefinedToTransferDst) => (
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT as u32,
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as u32
			),
			Some(ImageLayoutTransition::UndefinedToDepthStencilAttachment) => (
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT as u32,
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT as u32
			),
			Some(ImageLayoutTransition::TransferDstToShaderReadOnly) => (
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as u32,
				VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as u32
			),
			_ => panic!("unsupported layout transition {:?} -> {:?}", old_layout, new_layout)
		};

	vkCmdPipelineBarrier(
		command_buffer, 
		source_stage, 
		destination_stage, 
		0, 
		0, 
		nullptr(), 
		0, 
		nullptr(), 
		1, 
		&barrier
	);

	end_single_time_commands(vk_handle, command_buffer);
}

pub unsafe fn copy_buffer_to_image(
	vk_handle: &mut VkHandle,
	buffer: VkBuffer,
	image: VkImage,
	width: u32,
	height: u32,
)
{
	let command_buffer = begin_single_time_commands(vk_handle).unwrap();

	let region = VkBufferImageCopy{
		bufferOffset: 0,
		bufferRowLength: 0,
		bufferImageHeight: 0,
		imageSubresource: VkImageSubresourceLayers {
			aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32,
			mipLevel: 0,
			baseArrayLayer: 0,
			layerCount: 1,
		},
		imageOffset: VkOffset3D { 
			x: 0, 
			y: 0, 
			z: 0
		},
		imageExtent: VkExtent3D { 
			width: width, 
			height: height, 
			depth: 1
		},
	};

	vkCmdCopyBufferToImage(command_buffer, buffer, image, VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region);

	end_single_time_commands(vk_handle, command_buffer);
}

// pub unsafe fn create_image_barrier(
// 	vk_handle: &VkHandle
// ) -> Result<VkImageMemoryBarrier, String>
// {

// }

pub unsafe fn end_single_time_commands(
	vk_handle: &VkHandle,
	command_buffer: VkCommandBuffer
)
{
	vkEndCommandBuffer(command_buffer);

	let submit_info = 
		VkSubmitInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
			commandBufferCount: 1,
			pCommandBuffers: &command_buffer,
			waitSemaphoreCount: 0,
			pWaitSemaphores: nullptr(),
			signalSemaphoreCount: 0,
			pSignalSemaphores: nullptr(),
			pWaitDstStageMask: nullptr(),
			pNext: nullptr(),
		};

	vkQueueSubmit(vk_handle.graphics_queue, 1, &submit_info, nullptr());
	vkQueueWaitIdle(vk_handle.graphics_queue);

	vkFreeCommandBuffers(vk_handle.logical_device, vk_handle.command_pool, 1, &command_buffer);	
}

pub unsafe fn begin_single_time_commands(
	vk_handle: &VkHandle,
) -> Result<VkCommandBuffer, String>
{
	let command_buffer_allocate_info = 
		VkCommandBufferAllocateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
			level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
			commandPool: vk_handle.command_pool,
			commandBufferCount: 1,
			pNext: nullptr(),
		};

	// let mut command_buffer = std::mem::zeroed();
	let mut command_buffer = nullptr();
	match vkAllocateCommandBuffers(vk_handle.logical_device, &command_buffer_allocate_info, &mut command_buffer)
	{
		VkResult::VK_SUCCESS => {}
		err => { return Err(format!("vkAllocateCommandBuffers failed with code {:?}", err))}
	}

	let command_buffer_begin_info = 
		VkCommandBufferBeginInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			flags: VkCommandBufferUsageFlagBits::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT as u32,
			pInheritanceInfo: nullptr(),
			pNext: nullptr(),
		};

	match vkBeginCommandBuffer(command_buffer, &command_buffer_begin_info)
	{
		VkResult::VK_SUCCESS => { Ok(command_buffer) }
		err => { return Err(format!("vkBeginCommandBuffer failed with code {:?}", err))}
	}
}