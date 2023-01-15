#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HINSTANCE {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HWND {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HANDLE {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LPCWSTR {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SECURITY_ATTRIBUTES {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DWORD {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HMONITOR {
    _unused: [u8; 0],
}
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
#[test]
fn bindgen_test_layout_VkWin32SurfaceCreateInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkWin32SurfaceCreateInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkWin32SurfaceCreateInfoKHR>(),
        40usize,
        concat!("Size of: ", stringify!(VkWin32SurfaceCreateInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkWin32SurfaceCreateInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkWin32SurfaceCreateInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32SurfaceCreateInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32SurfaceCreateInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32SurfaceCreateInfoKHR),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hinstance) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32SurfaceCreateInfoKHR),
            "::",
            stringify!(hinstance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hwnd) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32SurfaceCreateInfoKHR),
            "::",
            stringify!(hwnd)
        )
    );
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
#[test]
fn bindgen_test_layout_VkImportMemoryWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkImportMemoryWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkImportMemoryWin32HandleInfoKHR>(),
        40usize,
        concat!("Size of: ", stringify!(VkImportMemoryWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkImportMemoryWin32HandleInfoKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkImportMemoryWin32HandleInfoKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(handleType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handle) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(handle)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(name)
        )
    );
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
#[test]
fn bindgen_test_layout_VkExportMemoryWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkExportMemoryWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkExportMemoryWin32HandleInfoKHR>(),
        40usize,
        concat!("Size of: ", stringify!(VkExportMemoryWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkExportMemoryWin32HandleInfoKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkExportMemoryWin32HandleInfoKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAttributes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(pAttributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwAccess) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(dwAccess)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoKHR),
            "::",
            stringify!(name)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub memoryTypeBits: u32,
}
#[test]
fn bindgen_test_layout_VkMemoryWin32HandlePropertiesKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkMemoryWin32HandlePropertiesKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkMemoryWin32HandlePropertiesKHR>(),
        24usize,
        concat!("Size of: ", stringify!(VkMemoryWin32HandlePropertiesKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkMemoryWin32HandlePropertiesKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkMemoryWin32HandlePropertiesKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryWin32HandlePropertiesKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryWin32HandlePropertiesKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memoryTypeBits) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryWin32HandlePropertiesKHR),
            "::",
            stringify!(memoryTypeBits)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
#[test]
fn bindgen_test_layout_VkMemoryGetWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkMemoryGetWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkMemoryGetWin32HandleInfoKHR>(),
        32usize,
        concat!("Size of: ", stringify!(VkMemoryGetWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkMemoryGetWin32HandleInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkMemoryGetWin32HandleInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryGetWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryGetWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memory) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryGetWin32HandleInfoKHR),
            "::",
            stringify!(memory)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkMemoryGetWin32HandleInfoKHR),
            "::",
            stringify!(handleType)
        )
    );
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
#[test]
fn bindgen_test_layout_VkWin32KeyedMutexAcquireReleaseInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkWin32KeyedMutexAcquireReleaseInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkWin32KeyedMutexAcquireReleaseInfoKHR>(),
        72usize,
        concat!(
            "Size of: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<VkWin32KeyedMutexAcquireReleaseInfoKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).acquireCount) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(acquireCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAcquireSyncs) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(pAcquireSyncs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAcquireKeys) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(pAcquireKeys)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAcquireTimeouts) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(pAcquireTimeouts)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).releaseCount) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(releaseCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pReleaseSyncs) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(pReleaseSyncs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pReleaseKeys) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoKHR),
            "::",
            stringify!(pReleaseKeys)
        )
    );
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
#[test]
fn bindgen_test_layout_VkImportSemaphoreWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkImportSemaphoreWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkImportSemaphoreWin32HandleInfoKHR>(),
        48usize,
        concat!("Size of: ", stringify!(VkImportSemaphoreWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkImportSemaphoreWin32HandleInfoKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).semaphore) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(semaphore)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(handleType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handle) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(handle)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(name)
        )
    );
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
#[test]
fn bindgen_test_layout_VkExportSemaphoreWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkExportSemaphoreWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkExportSemaphoreWin32HandleInfoKHR>(),
        40usize,
        concat!("Size of: ", stringify!(VkExportSemaphoreWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkExportSemaphoreWin32HandleInfoKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkExportSemaphoreWin32HandleInfoKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAttributes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(pAttributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwAccess) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(dwAccess)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportSemaphoreWin32HandleInfoKHR),
            "::",
            stringify!(name)
        )
    );
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
#[test]
fn bindgen_test_layout_VkD3D12FenceSubmitInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkD3D12FenceSubmitInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkD3D12FenceSubmitInfoKHR>(),
        48usize,
        concat!("Size of: ", stringify!(VkD3D12FenceSubmitInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkD3D12FenceSubmitInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkD3D12FenceSubmitInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkD3D12FenceSubmitInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkD3D12FenceSubmitInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).waitSemaphoreValuesCount) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkD3D12FenceSubmitInfoKHR),
            "::",
            stringify!(waitSemaphoreValuesCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pWaitSemaphoreValues) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkD3D12FenceSubmitInfoKHR),
            "::",
            stringify!(pWaitSemaphoreValues)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).signalSemaphoreValuesCount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkD3D12FenceSubmitInfoKHR),
            "::",
            stringify!(signalSemaphoreValuesCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pSignalSemaphoreValues) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VkD3D12FenceSubmitInfoKHR),
            "::",
            stringify!(pSignalSemaphoreValues)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
#[test]
fn bindgen_test_layout_VkSemaphoreGetWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkSemaphoreGetWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkSemaphoreGetWin32HandleInfoKHR>(),
        32usize,
        concat!("Size of: ", stringify!(VkSemaphoreGetWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkSemaphoreGetWin32HandleInfoKHR>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkSemaphoreGetWin32HandleInfoKHR)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSemaphoreGetWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSemaphoreGetWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).semaphore) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSemaphoreGetWin32HandleInfoKHR),
            "::",
            stringify!(semaphore)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSemaphoreGetWin32HandleInfoKHR),
            "::",
            stringify!(handleType)
        )
    );
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
#[test]
fn bindgen_test_layout_VkImportFenceWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkImportFenceWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkImportFenceWin32HandleInfoKHR>(),
        48usize,
        concat!("Size of: ", stringify!(VkImportFenceWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkImportFenceWin32HandleInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkImportFenceWin32HandleInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fence) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(fence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(handleType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handle) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(handle)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportFenceWin32HandleInfoKHR),
            "::",
            stringify!(name)
        )
    );
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
#[test]
fn bindgen_test_layout_VkExportFenceWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkExportFenceWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkExportFenceWin32HandleInfoKHR>(),
        40usize,
        concat!("Size of: ", stringify!(VkExportFenceWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkExportFenceWin32HandleInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkExportFenceWin32HandleInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportFenceWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportFenceWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAttributes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportFenceWin32HandleInfoKHR),
            "::",
            stringify!(pAttributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwAccess) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportFenceWin32HandleInfoKHR),
            "::",
            stringify!(dwAccess)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportFenceWin32HandleInfoKHR),
            "::",
            stringify!(name)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}
#[test]
fn bindgen_test_layout_VkFenceGetWin32HandleInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkFenceGetWin32HandleInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkFenceGetWin32HandleInfoKHR>(),
        32usize,
        concat!("Size of: ", stringify!(VkFenceGetWin32HandleInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkFenceGetWin32HandleInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkFenceGetWin32HandleInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkFenceGetWin32HandleInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkFenceGetWin32HandleInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fence) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkFenceGetWin32HandleInfoKHR),
            "::",
            stringify!(fence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkFenceGetWin32HandleInfoKHR),
            "::",
            stringify!(handleType)
        )
    );
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
#[test]
fn bindgen_test_layout_VkImportMemoryWin32HandleInfoNV() {
    const UNINIT: ::std::mem::MaybeUninit<VkImportMemoryWin32HandleInfoNV> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkImportMemoryWin32HandleInfoNV>(),
        32usize,
        concat!("Size of: ", stringify!(VkImportMemoryWin32HandleInfoNV))
    );
    assert_eq!(
        ::std::mem::align_of::<VkImportMemoryWin32HandleInfoNV>(),
        8usize,
        concat!("Alignment of ", stringify!(VkImportMemoryWin32HandleInfoNV))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoNV),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoNV),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handleType) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoNV),
            "::",
            stringify!(handleType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).handle) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkImportMemoryWin32HandleInfoNV),
            "::",
            stringify!(handle)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: *mut DWORD,
}
#[test]
fn bindgen_test_layout_VkExportMemoryWin32HandleInfoNV() {
    const UNINIT: ::std::mem::MaybeUninit<VkExportMemoryWin32HandleInfoNV> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkExportMemoryWin32HandleInfoNV>(),
        32usize,
        concat!("Size of: ", stringify!(VkExportMemoryWin32HandleInfoNV))
    );
    assert_eq!(
        ::std::mem::align_of::<VkExportMemoryWin32HandleInfoNV>(),
        8usize,
        concat!("Alignment of ", stringify!(VkExportMemoryWin32HandleInfoNV))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoNV),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoNV),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAttributes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoNV),
            "::",
            stringify!(pAttributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dwAccess) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkExportMemoryWin32HandleInfoNV),
            "::",
            stringify!(dwAccess)
        )
    );
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
#[test]
fn bindgen_test_layout_VkWin32KeyedMutexAcquireReleaseInfoNV() {
    const UNINIT: ::std::mem::MaybeUninit<VkWin32KeyedMutexAcquireReleaseInfoNV> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkWin32KeyedMutexAcquireReleaseInfoNV>(),
        72usize,
        concat!(
            "Size of: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<VkWin32KeyedMutexAcquireReleaseInfoNV>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).acquireCount) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(acquireCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAcquireSyncs) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(pAcquireSyncs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAcquireKeys) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(pAcquireKeys)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pAcquireTimeoutMilliseconds) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(pAcquireTimeoutMilliseconds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).releaseCount) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(releaseCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pReleaseSyncs) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(pReleaseSyncs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pReleaseKeys) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(VkWin32KeyedMutexAcquireReleaseInfoNV),
            "::",
            stringify!(pReleaseKeys)
        )
    );
}
pub const VkFullScreenExclusiveEXT_VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT =
    0;
pub const VkFullScreenExclusiveEXT_VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT =
    1;
pub const VkFullScreenExclusiveEXT_VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT:
    VkFullScreenExclusiveEXT = 2;
pub const VkFullScreenExclusiveEXT_VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT:
    VkFullScreenExclusiveEXT = 3;
pub const VkFullScreenExclusiveEXT_VK_FULL_SCREEN_EXCLUSIVE_MAX_ENUM_EXT: VkFullScreenExclusiveEXT =
    2147483647;
pub type VkFullScreenExclusiveEXT = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}
#[test]
fn bindgen_test_layout_VkSurfaceFullScreenExclusiveInfoEXT() {
    const UNINIT: ::std::mem::MaybeUninit<VkSurfaceFullScreenExclusiveInfoEXT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkSurfaceFullScreenExclusiveInfoEXT>(),
        24usize,
        concat!("Size of: ", stringify!(VkSurfaceFullScreenExclusiveInfoEXT))
    );
    assert_eq!(
        ::std::mem::align_of::<VkSurfaceFullScreenExclusiveInfoEXT>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkSurfaceFullScreenExclusiveInfoEXT)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceFullScreenExclusiveInfoEXT),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceFullScreenExclusiveInfoEXT),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fullScreenExclusive) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceFullScreenExclusiveInfoEXT),
            "::",
            stringify!(fullScreenExclusive)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType: VkStructureType,
    pub pNext: *mut ::std::os::raw::c_void,
    pub fullScreenExclusiveSupported: VkBool32,
}
#[test]
fn bindgen_test_layout_VkSurfaceCapabilitiesFullScreenExclusiveEXT() {
    const UNINIT: ::std::mem::MaybeUninit<VkSurfaceCapabilitiesFullScreenExclusiveEXT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkSurfaceCapabilitiesFullScreenExclusiveEXT>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(VkSurfaceCapabilitiesFullScreenExclusiveEXT)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<VkSurfaceCapabilitiesFullScreenExclusiveEXT>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkSurfaceCapabilitiesFullScreenExclusiveEXT)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceCapabilitiesFullScreenExclusiveEXT),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceCapabilitiesFullScreenExclusiveEXT),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).fullScreenExclusiveSupported) as usize - ptr as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceCapabilitiesFullScreenExclusiveEXT),
            "::",
            stringify!(fullScreenExclusiveSupported)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub hmonitor: *mut HMONITOR,
}
#[test]
fn bindgen_test_layout_VkSurfaceFullScreenExclusiveWin32InfoEXT() {
    const UNINIT: ::std::mem::MaybeUninit<VkSurfaceFullScreenExclusiveWin32InfoEXT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkSurfaceFullScreenExclusiveWin32InfoEXT>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(VkSurfaceFullScreenExclusiveWin32InfoEXT)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<VkSurfaceFullScreenExclusiveWin32InfoEXT>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(VkSurfaceFullScreenExclusiveWin32InfoEXT)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceFullScreenExclusiveWin32InfoEXT),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceFullScreenExclusiveWin32InfoEXT),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hmonitor) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkSurfaceFullScreenExclusiveWin32InfoEXT),
            "::",
            stringify!(hmonitor)
        )
    );
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
