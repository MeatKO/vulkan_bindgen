use std::env;

#[cfg(not(target_env = "gnu"))]
fn check_target_env()
{
	compile_error!("GNU Toolchain is required !")
}

fn main() 
{
	let target_os = 
		env::var("CARGO_CFG_TARGET_OS")
		.expect("Couldn't find Cargo target OS");

	if target_os != env::consts::OS
	{
		panic!("Target OS {} must match Host OS {}", target_os, env::consts::OS);
	}

	let vulkan_sdk_path = 
		env::var_os("VULKAN_SDK")
		.expect("Couldn't Get ENV Variable VULKAN_SDK.\nPerhaps the SDK is not installed or added to PATH?\nVulkan SDK Download link - https://www.lunarg.com/vulkan-sdk/\n")
		.to_str()
		.expect("Couldn't convert the Vulkan SDK Path to valid utf-8 string.")
		.to_owned();

	match target_os.as_str()
	{
		"linux" => 
		{ 
			println!("cargo:rustc-link-search=native={}/lib", vulkan_sdk_path); 
			println!("cargo:rustc-link-lib=vulkan");
			println!("cargo:rustc-link-lib=xcb");
			println!("cargo:rustc-link-lib=xcb-xfixes");
		}
		"windows" => 
		{ 
			let libclang_path = 
				env::var_os("LIBCLANG_PATH")
				.expect("Couldn't Get ENV Variable LIBCLANG_PATH.\nPerhaps the MSVC Clang build tools were not installed or added to PATH?\nlibclang.lib can be found at <VS installation location>\\VC\\Tools\\Llvm\\lib\n")
				.to_str()
				.expect("Couldn't convert the libClang path to valid utf-8 string.")
				.to_owned();

			println!("cargo:rustc-link-search={}", libclang_path);
			println!("cargo:rustc-link-lib=libclang");
			println!("cargo:rustc-link-lib=user32");
			
			println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk_path);
			println!("cargo:rustc-link-lib=vulkan-1");
		}
		any => 
		{ 
			if any.to_lowercase().contains("none")
			{
				panic!("Target OS cannot be 'none'.\nPerhaps you changed the global cargo config for a no-OS freestanding binary?");
			}

			panic!("Unsupported platform '{}'.\nCurrent supported platforms are linux & windows.", any); 
		}
	}
}