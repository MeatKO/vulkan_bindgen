pub fn vk_make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32
{
	return (variant << 29) | (major << 22) | (minor << 12) | (patch);
}

// DEPRECATED: This define is deprecated. VK_MAKE_API_VERSION should be used instead.
pub fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32
{
	return vk_make_api_version(0, major, minor, patch);
}

// pub fn vk_api_version_1_0() -> u32
// {
// 	return vk_make_api_version(0, 1, 0, 0);
// }