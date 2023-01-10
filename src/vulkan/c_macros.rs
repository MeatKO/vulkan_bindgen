#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32
{
	return (variant << 29) | (major << 22) | (minor << 12) | (patch);
}

// DEPRECATED: This define is deprecated. VK_MAKE_API_VERSION should be used instead.
pub fn VK_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32
{
	return VK_MAKE_API_VERSION(0, major, minor, patch);
}

pub fn VK_API_VERSION_1_0() -> u32
{
	return VK_MAKE_API_VERSION(0, 1, 0, 0);
}