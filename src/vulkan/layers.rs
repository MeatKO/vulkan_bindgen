use std::fmt;

use crate::vulkan::vk_bindgen::*;
use crate::ffi::strings::*;

impl VkLayerProperties
{
	pub fn get_layer_name(&self) -> &str
	{
		from_c_string(&self.layerName).unwrap()
	}
}
impl fmt::Display for VkLayerProperties 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
        write!(f, "{}", String::from_utf8(self.layerName.iter().map(|&c| c as u8).collect()).unwrap())
    }
}

pub fn check_layer_availability(needed_layers: &Vec<&str>, available_layers: &Vec<VkLayerProperties>)
{
	for needed_layer in needed_layers
	{
		let mut found = false;
		for available_layer in available_layers
		{
			if (*needed_layer).to_owned() == available_layer.get_layer_name()
			{
				found = true;
				break;
			}
		}

		if found == false
		{
			panic!("Layer {} is not available", needed_layer);
		}
	}
}