use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::texture::*;
use crate::vulkan::texture_view::*;

pub unsafe fn create_depth_buffer(vk_handle: &mut VkHandle)
{
	let depth_format = find_depth_format(vk_handle);

	let (image, image_memory) = 
		create_image(
			vk_handle, 
			vk_handle.swapchain_extent.width, 
			vk_handle.swapchain_extent.height, 
			depth_format, 
			VkImageTiling::VK_IMAGE_TILING_OPTIMAL, 
			VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT as u32, 
			VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
		).unwrap();

	vk_handle.depth_image = image;
	vk_handle.depth_image_memory = image_memory;
	vk_handle.depth_image_view = 
		create_image_view(
			vk_handle, 
			&vk_handle.depth_image,
			depth_format, 
			VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32
		);

	transition_image_layout(
		vk_handle, 
		depth_format,
		vk_handle.depth_image, 
		VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED, 
		VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL
	);

}

pub unsafe fn has_stencil_component(
	format: &VkFormat
) -> bool
{
	return format.eq(&VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT)|| 
			format.eq(&VkFormat::VK_FORMAT_D24_UNORM_S8_UINT)
}

pub unsafe fn find_depth_format(
	vk_handle: &mut VkHandle
) -> VkFormat
{
	return find_supported_format(
		vk_handle, 
		&vec![
			VkFormat::VK_FORMAT_D32_SFLOAT,
			VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT,
			VkFormat::VK_FORMAT_D24_UNORM_S8_UINT,
		], 
		VkImageTiling::VK_IMAGE_TILING_OPTIMAL, 
		VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT as u32
	)
}

unsafe fn find_supported_format(
	vk_handle: &mut VkHandle,
	candidates: &Vec<VkFormat>,
	tiling: VkImageTiling,
	features: VkFormatFeatureFlags
) -> VkFormat
{
	let features = features as u32;

	for format in candidates.iter().copied()
	{
		let mut properties: VkFormatProperties = std::mem::zeroed();
		vkGetPhysicalDeviceFormatProperties(vk_handle.physical_device, format, &mut properties);

		if tiling == VkImageTiling::VK_IMAGE_TILING_LINEAR &&
			(properties.linearTilingFeatures & features) == features
		{
			return format
		}
		else if tiling == VkImageTiling::VK_IMAGE_TILING_OPTIMAL &&
			(properties.optimalTilingFeatures & features) == features
		{
			return format
		}
	}

	panic!("couldn't find supported format")
}