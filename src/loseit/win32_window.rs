use crate::vulkan::{
	vk_bindgen::*,
	handle::VkHandle,
	// surface::choose_surface_format,
	extension::get_missing_extensions
};

use crate::loseit::{
    win32::*,
	// win32_bindgen::*,
	win32_vk_bindgen::*,
	window_traits::*,
	window_events::*,
};

use std::ffi::c_void;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::ptr::null_mut as nullptr;

const DEFAULT_WINDOW_NAME: &str = "Vulkan Window";

pub unsafe fn get_win32_presentation_support_function(instance: &VkInstance) -> Option<unsafe extern "C" fn(*mut VkPhysicalDevice_T, u32) -> u32>
{
	std::mem::transmute::<_, PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>(vkGetInstanceProcAddr(*instance, "vkGetPhysicalDeviceXcbPresentationSupportKHR\0".as_ptr() as _))
}

pub unsafe fn create_win32_surface_function(instance: &VkInstance) -> Option<unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult>
{
	std::mem::transmute::<_, PFN_vkCreateWin32SurfaceKHR>(vkGetInstanceProcAddr(*instance, "vkCreateWin32SurfaceKHR\0".as_ptr() as _))
}

#[derive(Clone)]
pub struct Win32Handle
{
    h_instance: HINSTANCE,
    wc: WNDCLASSW,
    hwnd: *mut c_void,
// 	xcb_conn: *mut xcb_connection_t,
// 	xcb_window: xcb_window_t,
// 	atom_wm_protocols: xcb_atom_t,
// 	atom_wm_delete_window: xcb_atom_t
}

impl Drop for Win32Handle
{
	fn drop(&mut self)
	{
		println!("Destroying Win32 window.");
		self.destroy()
	}
}

impl VulkanWindowHandle for Win32Handle
{
	fn new(window_title: &Option<String>, width: u32, height: u32, vk_handle: &mut VkHandle) -> Option<Self>
	{
		unsafe
		{
            let mut handle = 
            Win32Handle
			{
                h_instance: nullptr(),
                wc: std::mem::zeroed::<_>(),
                hwnd: nullptr(),
			};
			
			let title = 
				match window_title
				{
					Some(title) => { title.clone() }
					None => { DEFAULT_WINDOW_NAME.to_owned() }
				};

            handle.h_instance = GetModuleHandleW(core::ptr::null());
            let sample_window_class_wn = wide_null(&title);
        
            let mut wc = std::mem::zeroed::<WNDCLASSW>();
            //let mut wc :WNDCLASSW =  unsafe { core::mem::zeroed() };
            wc.lpfnWndProc = Some(window_procedure);
        
            wc.hInstance = handle.h_instance;
            wc.lpszClassName = sample_window_class_wn.as_ptr();
            //wc.hCursor = unsafe { LoadCursorW(hInstance, IDC_ARROW) };
            wc.hCursor = LoadCursorW(nullptr(), IDC_ARROW);
            let atom = RegisterClassW(&wc);
            if atom == 0 
            {
                let last_error = GetLastError();
                panic!(
                    "Could not register the window class, error code: {}",
                    last_error
                );
            }
        
            let sample_window_name_wn = wide_null("Sample Window Name");
            // in main
            let lparam: *mut i32 = Box::leak(Box::new(5i32));
            handle.hwnd = 
                CreateWindowExW(
                    0,
                    sample_window_class_wn.as_ptr(),
                    sample_window_name_wn.as_ptr(),
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    width as i32,
                    height as i32,
                    nullptr(),
                    nullptr(),
                    handle.h_instance,
                    lparam.cast(),
                );
        
            if handle.hwnd.is_null() 
            {
                panic!("Failed to create a window.");
            }
        
            let _previously_visible = ShowWindow(handle.hwnd, SW_SHOW);

			match create_win32_surface_function(&vk_handle.instance)
			{
				None => { panic!("This platform doesn't offer a 'vkCreateWin32SurfaceKHR' function.") }
				Some(function) => 
				{
					let surface_create_info = 
						VkWin32SurfaceCreateInfoKHR 
						{
							sType: VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
							hinstance: handle.h_instance as _,
							hwnd: handle.hwnd as _,
							flags: 0,
							pNext: nullptr()
						};

					let result = function(vk_handle.instance, &surface_create_info, nullptr(), &mut vk_handle.window_surface);
					match result
					{
						VkResult::VK_SUCCESS => {}
						res => { panic!("Vulkan is not supported on given Win32 window. vkCreateWin32SurfaceKHR() resulted in {:?}", res) }
					}
				}
			}

            Some(handle)
		}
	}

	fn get_event(&self) -> Option<WindowEvent> 
	{
        unsafe
		{
			let mut msg = MSG::default();
    
			// BLOCKING
			// Replace with SetWindowsHookEx() ? 
			// match GetMessageW(&mut msg, nullptr(), 0, 0)
			match PeekMessageW(&mut msg, nullptr(), 0, 0, 1)
			{
				0 => { return None; }
				// -1 => { panic!("Error with `GetMessageW`, error code: {}", GetLastError()); }
				-1 => { panic!("Error with `PeekMessageW`, error code: {}", GetLastError()); }
				x =>
				{
					println!("Win32 msg code : {}", x);
					println!("Win32 msg : {:?}", &msg);
					TranslateMessage(&msg);
					DispatchMessageW(&msg);
					return None;
				}
			};
		}		
	}

	fn destroy(&mut self)
	{
		
	}

	fn lock_pointer(&self) 
	{
		
	}

	fn center_pointer(&self) 
	{
		
	}

	fn hide_cursor(&self) 
	{
		
	}

	fn unlock_pointer(&self) 
	{
		
	}
}