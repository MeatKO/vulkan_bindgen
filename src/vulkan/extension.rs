#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

use crate::vulkan::vk_bindgen::*;
use crate::vulkan::strings::*;

impl VkExtensionProperties
{
	pub fn get_extension_name(&self) -> &str
	{
		unsafe
		{
			from_c_string(&self.extensionName)
		}
	}
}
impl fmt::Display for VkExtensionProperties 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
        write!(f, "{}", String::from_utf8(self.extensionName.iter().map(|&c| c as u8).collect()).unwrap())
    }
}	

pub fn check_extension_availability(needed_extensions: &Vec<&str>, available_extensions: &Vec<VkExtensionProperties>)
{
	for needed_extension in needed_extensions
	{
		let mut found = false;
		for available_extension in available_extensions
		{
			if (*needed_extension).to_owned() == available_extension.get_extension_name()
			{
				found = true;
				break;
			}
		}

		if found == false
		{
			panic!("Extension {} is not available", needed_extension);
		}
	}
}