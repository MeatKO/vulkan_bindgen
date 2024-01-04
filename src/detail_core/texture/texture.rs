use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::ptr::null_mut as nullptr;

use crate::pixcell::error::*;


use crate::pixcell::image::GenericImage;
use crate::pixcell::tga::TGAImage;
use crate::vulkan::handle::VkHandle;
use crate::vulkan::texture::create_texture_image;
use crate::vulkan::texture_view::{create_texture_image_view, create_texture_sampler};
use crate::vulkan::vk_bindgen::{
	VkImage,
	VkDeviceMemory, VkImageView, VkSampler, VkFormat,
};

#[derive(Debug)]
pub struct Texture<T>(T);

pub struct TexturePath
{
	file_path: PathBuf
}
pub struct LoadedTexture
{
	image: GenericImage
}

#[derive(Clone, Debug)]
pub struct VulkanTexture
{
	pub path: String, 
	
	pub texture_image: VkImage,
	pub texture_image_memory: VkDeviceMemory,
	pub texture_image_view: VkImageView,
	pub texture_sampler: VkSampler,

	pub byte_size: usize,
}

impl<T> Deref for Texture<T> 
{
    type Target = T;

    fn deref(&self) -> &T 
	{
        &self.0
    }
}

impl<T> DerefMut for Texture<T> 
{
    fn deref_mut(&mut self) -> &mut T 
	{
        &mut self.0
    }
}

impl<T: Clone> Clone for Texture<T> 
{
    fn clone(&self) -> Self 
	{
        Texture(self.0.clone())
    }
}

impl Texture<TexturePath>
{
	pub fn new(in_file_path: PathBuf) -> Self
	{
		Texture (
			TexturePath { file_path: in_file_path }
		)
	}

	pub fn load(self) -> Result<Texture<LoadedTexture>, TextureLoadError>
	{
		match self.file_path.extension().ok_or(TextureLoadError::UnsupportedFileType("None".to_owned()))?.to_str()
		{
			Some("tga") => 
			{
				Ok(
					Texture (
						LoadedTexture { 
							image: TGAImage::new(self.file_path.clone())?.to_generic()
						}
					)
				)
			},
			_ => return Err(TextureLoadError::UnsupportedFileType("None".to_owned())),
		}
	}
}

impl Texture<LoadedTexture>
{
	// add a TextureProcessingError later (in this crate ?)
	// Change this to later incorporate additional RGB & RGBA checks, not all input images are equal...
	// None of this is error-handled btw ?
	// pub fn process_vk(self, vk_handle: &VkHandle, in_vk_format: VkFormat) -> Result<Texture<VulkanTexture>, String>
	pub fn process_vk(self, vk_handle: &VkHandle, vk_format: VkFormat) -> Result<Texture<VulkanTexture>, String>
	{
		unsafe
		{
			let (texture_image, texture_image_memory) = create_texture_image(&vk_handle, &self.image, vk_format)?;
			let texture_image_view = create_texture_image_view(&vk_handle, &texture_image, vk_format);
			let texture_sampler = create_texture_sampler(&vk_handle).unwrap();

			Ok(
				Texture (
					VulkanTexture {
						path: self.image.get_path().to_owned(), 
						texture_image: texture_image, 
						texture_image_memory: texture_image_memory, 
						texture_image_view: texture_image_view, 
						texture_sampler: texture_sampler,
						byte_size: self.image.get_byte_size(),
					}
				)
			)
		}
	}

	// pub fn downscale(mut self, factor: f32) -> Self
	// {
	// 	let old_image_data = self.image.get_data_u32_ref();
	// 	let old_image_dimensions = self.image.get_dimensions();

	// 	let (mut old_width, mut old_height) = 
	// 		(old_image_dimensions.width,
	// 		old_image_dimensions.height);

	// 	let (mut new_width, mut new_height) = 
	// 		((old_image_dimensions.width as f32 * factor) as usize, 
	// 		(old_image_dimensions.height as f32 * factor) as usize);

	// 	let mut new_image_data: Vec<u32> = vec![0u32; (new_width * new_height) as usize];
		
	// 	for x in 0..new_width
	// 	{
	// 		for y in 0..new_height
	// 		{
	// 			let new_sample_index = (x * new_width) + y;
	// 			let old_sample_index = (((x as f32 / factor) as u32 * old_width) + (y as f32 / factor) as u32) as usize;
	// 			new_image_data[new_sample_index] = old_image_data[old_sample_index];
	// 		}
	// 	}

	// 	self.image.set_data_u32(new_image_data);
	// 	self.image.set_dimensions(ImageDimensions{ width: new_width as _, height: new_height as _});

	// 	self
	// }
}

impl Texture<VulkanTexture>
{
	pub fn new_invalid() -> Self
	{
		Texture (
			VulkanTexture {
				path: String::new(), 
				texture_image: nullptr(),
				texture_image_memory: nullptr(),
				texture_image_view: nullptr(),
				texture_sampler: nullptr(),
				byte_size: 0,
			}
		)
	}
}