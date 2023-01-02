use std::ffi::{c_void};

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
		let enable_validation_layers = true;

		let application_info = vulkan::VkApplicationInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pApplicationName: c_string("deta:l vulkan\0"),
			applicationVersion: vulkan::VK_MAKE_VERSION(1, 0, 0),
			pEngineName: c_string("deta:l alpha\0"),
			engineVersion: vulkan::VK_MAKE_VERSION(1, 0, 0),
			apiVersion: vulkan::VK_MAKE_API_VERSION(0, 1, 2, 0),
			pNext: 0 as *const c_void
		};

		// Extensions
		let mut extension_count = 0u32;
		vulkan::vkEnumerateInstanceExtensionProperties(0 as _, &mut extension_count as _, 0 as _);
		let mut extension_vec = vec![ std::mem::zeroed::<_>(); extension_count as usize ];
		vulkan::vkEnumerateInstanceExtensionProperties(0 as _, &mut extension_count as _, extension_vec.as_mut_ptr());
		
		println!("Instance Extensions :");
		println!("\tSupported extension count : {}", extension_count);
		println!("\tSupported extensions : ");
		for elem in &extension_vec 
		{
			println!("\t\t{} - {:?}", elem.get_extension_name().unwrap(), elem.extensionName.as_ptr());
		}

		let extension_names = extension_vec
			.iter()
			.map(|extension_property|  extension_property.extensionName.as_ptr())
			.collect::<Vec<*const i8>>();

		// Validation layers
		let needed_layers = vec![
			"VK_LAYER_KHRONOS_validation"
		];

		let mut layer_count = 0u32;
		vulkan::vkEnumerateInstanceLayerProperties(&mut layer_count as _, 0 as _);
		let mut layer_vec = vec![ std::mem::zeroed::<_>(); layer_count as usize ];
		vulkan::vkEnumerateInstanceLayerProperties(&mut layer_count as _, layer_vec.as_mut_ptr());

		println!("\nIntance Layers :");
		println!("\tSupported layer count: {}", layer_count);
		println!("\tSupported layers : ");
		for property in &layer_vec 
		{
			println!("\t\t{} - {:?}", property.get_layer_name().unwrap(), property.layerName.as_ptr());
		}

		let layer_names = layer_vec
			.iter()
			.filter(|layer|
				layer.get_layer_name().unwrap() != "VK_LAYER_LUNARG_gfxreconstruct"
			)
			.map(|layer|  layer.layerName.as_ptr())
			.collect::<Vec<*const i8>>();

		if enable_validation_layers
		{
			println!("\nChecking layer availability...");
			// vulkan::check_layer_availability(&needed_layers, &layer_vec);
			println!("success!");
		}

		// Instance
		let create_info = vulkan::VkInstanceCreateInfo{
			sType: vulkan::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pApplicationInfo: &application_info,
			enabledExtensionCount: extension_names.len() as u32,
			ppEnabledExtensionNames: extension_names.as_ptr(),
			enabledLayerCount: layer_names.len() as u32,
			ppEnabledLayerNames: layer_names.as_ptr(),
			flags: 0,
			pNext: 0 as _
		};

		let mut vulkan_instance = 0 as vulkan::VkInstance;
		let create_info_result = vulkan::vkCreateInstance(&create_info, 0 as _, &mut vulkan_instance);

		println!("\nCreating vulkan instance...");
		if create_info_result != vulkan::VkResult_VK_SUCCESS
		{
			panic!("failed with code {}.", create_info_result);
		}
		println!("success!");




		// Cleanup
		vulkan::vkDestroyInstance(vulkan_instance, 0 as _);
	}
}
