use std::env;

fn main() 
{
	let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

	let library_location = 
	{
		match target_os.as_str()
		{
			"linux" => { None }
			"windows" => { Some("/home/gate/Documents/Programming/VulkanSDKWindows/1.3.236.0/Lib/") }
			_ => { panic!("unsupported platform") }
		}
	};

	let library_name = 
	{
		match target_os.as_str()
		{
			"linux" => { "vulkan" }
			"windows" => { "vulkan-1" }
			_ => { panic!("unsupported platform") }
		}
	};
	
	match library_location 
	{
		Some(location) => { println!("cargo:rustc-link-search=native={}", location); }
		None => {}
	}

	println!("cargo:rustc-link-lib={}", library_name);
}