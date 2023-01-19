#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;
use crate::loseit::xcb_bindgen::*;

pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}
pub type PFN_vkCreateXcbSurfaceKHR = ::std::option::Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
>;
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = ::std::option::Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> VkBool32,
>;
extern "C" {
    pub fn vkCreateXcbSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> VkBool32;
}