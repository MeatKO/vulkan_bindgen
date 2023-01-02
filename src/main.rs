use std::ffi::{c_char, c_void};

mod vulkan;

// only takes static string slices because I have trust issues 
// (I don't know how the C code will later use this memory)
unsafe fn c_string(input_str: &'static str) -> *const i8
{
	input_str.as_ptr() as *const i8
}

fn main() 
{
	unsafe
	{
		let application_info = vulkan::VkApplicationInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pApplicationName: c_string("deta:l vulkan\0"),
			applicationVersion: vulkan::VK_MAKE_VERSION(1, 0, 0),
			pEngineName: c_string("deta:l alpha\0"),
			engineVersion: vulkan::VK_MAKE_VERSION(1, 0, 0),
			apiVersion: vulkan::VK_API_VERSION_1_0(),
			pNext: 0 as *const c_void
		};

		
		// Extensions
		let mut extension_count = 0u32;
		vulkan::vkEnumerateInstanceExtensionProperties(0 as *const c_char, &mut extension_count as *mut u32, 0 as *mut _);
		let mut extension_vec = vec![ std::mem::zeroed::<vulkan::VkExtensionProperties>(); extension_count as usize ];
		vulkan::vkEnumerateInstanceExtensionProperties(0 as *const c_char, &mut extension_count as *mut u32, extension_vec.as_mut_ptr());
		
		println!("Instance Extensions :");
		println!("Supported extension count : {}", extension_count);
		println!("Supported extensions : ");
		for elem in &extension_vec 
		{
			println!("{} - {:?}", elem, elem.extensionName.as_ptr());
		}

		let extension_names = extension_vec
			.iter()
			.map(|extension_property|  extension_property.extensionName.as_ptr())
			.collect::<Vec<*const i8>>();

		// Instance
		let create_info = vulkan::VkInstanceCreateInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pApplicationInfo: &application_info,
			enabledExtensionCount: extension_count,
			ppEnabledExtensionNames: extension_names.as_ptr(),
			enabledLayerCount: 0,
			ppEnabledLayerNames: 0 as *const *const i8,
			flags: 0,
			pNext: 0 as *const c_void
		};

		let mut vulkan_instance = 0 as vulkan::VkInstance;
		let create_info_result = vulkan::vkCreateInstance(&create_info, 0 as *const _, &mut vulkan_instance);

		print!("\nCreating vulkan instance...");
		if create_info_result != vulkan::VkResult_VK_SUCCESS
		{
			panic!("failed with code {}.", create_info_result);
		}
		println!("success!");

		vulkan::vkDestroyInstance(vulkan_instance, 0 as *const _);

		// Validation layers
		// let mut layer_count = 0u32;
		// vulkan::vkEnumerateInstanceLayerProperties(&mut extension_count as *mut u32, 0 as *mut _);
		// let mut extension_vec = vec![
		// 	vulkan::VkExtensionProperties{extensionName: [0i8; 256], specVersion: 0};
		// 	extension_count as usize
		// ];
		// vulkan::vkEnumerateInstanceExtensionProperties(0 as *const c_char, &mut extension_count as *mut u32, extension_vec.as_mut_ptr());
		

	}
}
