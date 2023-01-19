#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;
use crate::loseit::win32_bindgen::*;

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: *mut HINSTANCE,
    pub hwnd: *mut HWND,
}
pub type PFN_vkCreateWin32SurfaceKHR = ::std::option::Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
>;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = ::std::option::Option<
    unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32,
>;
extern "C" {
    pub fn vkCreateWin32SurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
    ) -> VkBool32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
    pub handle: *mut HANDLE,
    pub name: *mut LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: *mut DWORD,
    pub name: *mut LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub memoryTypeBits: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
pub type PFN_vkGetMemoryWin32HandleKHR = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
    ) -> VkResult,
>;
extern "C" {
    pub fn vkGetMemoryWin32HandleKHR(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetMemoryWin32HandlePropertiesKHR(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
    ) -> VkResult;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeouts: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
    pub handle: *mut HANDLE,
    pub name: *mut LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: *mut DWORD,
    pub name: *mut LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
pub type PFN_vkImportSemaphoreWin32HandleKHR = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
    ) -> VkResult,
>;
pub type PFN_vkGetSemaphoreWin32HandleKHR = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;
extern "C" {
    pub fn vkImportSemaphoreWin32HandleKHR(
        device: VkDevice,
        pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetSemaphoreWin32HandleKHR(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
    pub handle: *mut HANDLE,
    pub name: *mut LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: *mut DWORD,
    pub name: *mut LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}
pub type PFN_vkImportFenceWin32HandleKHR = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
    ) -> VkResult,
>;
pub type PFN_vkGetFenceWin32HandleKHR = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;
extern "C" {
    pub fn vkImportFenceWin32HandleKHR(
        device: VkDevice,
        pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetFenceWin32HandleKHR(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsNV,
    pub handle: *mut HANDLE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: *mut DWORD,
}
pub type PFN_vkGetMemoryWin32HandleNV = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        memory: VkDeviceMemory,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;
extern "C" {
    pub fn vkGetMemoryWin32HandleNV(
        device: VkDevice,
        memory: VkDeviceMemory,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut HANDLE,
    ) -> VkResult;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeoutMilliseconds: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VkFullScreenExclusiveEXT {
    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
    VK_FULL_SCREEN_EXCLUSIVE_MAX_ENUM_EXT = 2147483647,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub fullScreenExclusiveSupported: VkBool32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub hmonitor: *mut HMONITOR,
}
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = ::std::option::Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult,
>;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT = ::std::option::Option<
    unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult,
>;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT = ::std::option::Option<
    unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult,
>;
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
    ) -> VkResult,
>;
extern "C" {
    pub fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkAcquireFullScreenExclusiveModeEXT(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkReleaseFullScreenExclusiveModeEXT(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetDeviceGroupSurfacePresentModes2EXT(
        device: VkDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
    ) -> VkResult;
}