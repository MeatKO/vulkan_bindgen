use crate::vulkan::vk_bindgen::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_texture_image_view(
	device: &VkDevice,
	texture_image: &VkImage,
	vk_format: VkFormat,
) -> Result<VkImageView, String>
{
	create_image_view(
		device, 
		texture_image,
		vk_format,
		VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32
	)
}

pub unsafe fn create_image_view(
	device: &VkDevice,
	image: &VkImage,
	format: VkFormat,
	aspect_flags: VkImageAspectFlags,
// ) -> VkImageView
) -> Result<VkImageView, String>
{
	let image_view_create_info = 
		VkImageViewCreateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			image: *image,
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
	match vkCreateImageView(*device, &image_view_create_info, nullptr(), &mut image_view)
	{
		// VkResult::VK_SUCCESS => { println!("✔️ vkCreateImageView()"); }
		VkResult::VK_SUCCESS => {}
		err => { return Err(format!("✗ vkCreateImageView() failed with code {:?}.", err).to_owned()); }
	}	

	return Ok(image_view)
}

pub unsafe fn create_texture_sampler(
	device: &VkDevice,
	physical_device: &VkPhysicalDevice,
) -> Result<VkSampler, String>
{
	let mut physical_device_properties: VkPhysicalDeviceProperties = std::mem::zeroed();
	vkGetPhysicalDeviceProperties(*physical_device, &mut physical_device_properties);

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

	let mut texture_sampler = nullptr();
	match vkCreateSampler(*device, &sampler_create_info, nullptr(), &mut texture_sampler)
	{
		VkResult::VK_SUCCESS => 
		{
			Ok(texture_sampler)
		}
		error_code => 
		{ 
			Err(
				format!("vkCreateSampler Failed With Code '{:?}'. logical_device_pointer:{:p} sampler_create_info_pointer:{:p}", 
				error_code, *device, &sampler_create_info).to_owned()
			)
		}
	}	
}