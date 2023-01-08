use std::env;

fn main() 
{
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Couldn't find Cargo target OS");
    
    if target_os != env::consts::OS
    {
        panic!("Target OS {} must match Host OS {}", target_os, env::consts::OS);
    }

    let vulkan_sdk_location = env::var_os("VULKAN_SDK").expect("Couldn't Get ENV Variable VULKAN_SDK.\nPerhaps the SDK is not installed or added to PATH?\nVulkan SDK Download link - https://www.lunarg.com/vulkan-sdk/\n");
    
    match target_os.as_str()
    {
        "linux" => 
        { 
            println!("cargo:rustc-link-search=native={}/lib", vulkan_sdk_location.to_str().unwrap()); 
            println!("cargo:rustc-link-lib=vulkan");
        }
        "windows" => 
        { 
            println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk_location.to_str().unwrap());
            println!("cargo:rustc-link-lib=vulkan-1");
        }
        _ => { panic!("Unsupported platform.\nCurrent supported platforms are linux & windows."); }
    }
}