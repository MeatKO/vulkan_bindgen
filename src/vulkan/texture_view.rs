use crate::exedra::model::Model;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_texture_image_view(
	vk_handle: &mut VkHandle,
	model: &mut Model
)
{
	model.texture_image_view = 
		create_image_view(
			vk_handle, 
			model.texture_image,
			VkFormat::VK_FORMAT_R8G8B8A8_SRGB, 
			VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32
		);
}

pub unsafe fn create_image_view(
	vk_handle: &mut VkHandle,
	image: VkImage,
	format: VkFormat,
	aspect_flags: VkImageAspectFlags,
) -> VkImageView
{
	let image_view_create_info = 
		VkImageViewCreateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			image: image,
			viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
			format: format,
			subresourceRange: VkImageSubresourceRange { 
				aspectMask: aspect_flags as u32, 
				baseMipLevel: 0, 
				levelCount: 1, 
				baseArrayLayer: 0, 
				layerCount: 1 
			},
			components: VkComponentMapping { 
				r: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY,
				g: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY, 
				b: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY, 
				a: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY 
			},
			flags: 0,	
			pNext: nullptr(),
		};

	let mut image_view: VkImageView = nullptr();
	match vkCreateImageView(vk_handle.logical_device, &image_view_create_info, nullptr(), &mut image_view)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateImageView()"); }
		err => { panic!("✗ vkCreateImageView() failed with code {:?}.", err); }
	}	

	return image_view
}

pub unsafe fn create_texture_sampler(
	vk_handle: &mut VkHandle,
	model: &mut Model
)
{
	let mut physical_device_properties: VkPhysicalDeviceProperties = std::mem::zeroed();
	vkGetPhysicalDeviceProperties(vk_handle.physical_device, &mut physical_device_properties);

	let sampler_create_info = 
		VkSamplerCreateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
			magFilter: VkFilter::VK_FILTER_LINEAR,
			minFilter: VkFilter::VK_FILTER_LINEAR,
			// magFilter: VkFilter::VK_FILTER_NEAREST,
			// minFilter: VkFilter::VK_FILTER_NEAREST,
			addressModeU: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
			addressModeV: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
			addressModeW: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
			// anisotropyEnable: VK_FALSE, // its not enabled yet ?
			anisotropyEnable: VK_TRUE,
			maxAnisotropy: physical_device_properties.limits.maxSamplerAnisotropy,
			borderColor: VkBorderColor::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
			unnormalizedCoordinates: VK_FALSE,
			compareOp: VkCompareOp::VK_COMPARE_OP_ALWAYS,
			mipmapMode: VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_LINEAR,
			maxLod: 0f32,
			minLod: 0f32,
			mipLodBias: 0f32,
			compareEnable: VK_FALSE,
			flags: 0,	
			pNext: nullptr(),
		};

	match vkCreateSampler(vk_handle.logical_device, &sampler_create_info, nullptr(), &mut model.texture_sampler)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateSampler()"); }
		err => { panic!("✗ vkCreateSampler() failed with code {:?}.", err); }
	}	

}