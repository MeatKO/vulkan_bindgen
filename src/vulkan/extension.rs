use std::fmt;

use crate::vulkan::vk_bindgen::*;

use crate::ffi::strings::*;


impl VkExtensionProperties
{
	pub fn get_extension_name(&self) -> &str
	{
		from_c_string(&self.extensionName).unwrap()
	}
}
impl fmt::Display for VkExtensionProperties 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
        write!(f, "{}", String::from_utf8(self.extensionName.iter().map(|&c| c as u8).collect()).unwrap())
    }
}	

pub fn get_missing_extensions(required_extensions: &Vec<&str>, available_extensions: &Vec<VkExtensionProperties>) -> Option<Vec<String>>
{
	let available_extensions = 
		available_extensions
		.iter()
		.map(|extension_property|  extension_property.get_extension_name())
		.collect::<Vec<&str>>();

	let out_vec = 
		required_extensions
		.iter()
		.copied()
		.filter(
			|required_extension|
			!(*available_extensions).contains(required_extension)
		)
		.map(
			|required_extension|
			required_extension.to_owned()
		)
		.collect::<Vec<String>>();

	if out_vec.is_empty()
	{
		return None
	}

	Some(out_vec)
}