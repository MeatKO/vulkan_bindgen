use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use crate::pixcell::error::*;

use crate::pixcell::image::Image;

use crate::pixcell::tga::TGAImage;
use crate::vulkan::handle::VkHandle;
use crate::vulkan::texture::create_texture_image;
use crate::vulkan::texture_view::{create_texture_image_view, create_texture_sampler};
use crate::vulkan::vk_bindgen::{
	VkImage,
	VkDeviceMemory, VkImageView, VkSampler,
};

pub struct Texture<T>(T);

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

pub struct TexturePath
{
	file_path: PathBuf
}
pub struct LoadedTexture
{
	image: Box<dyn Image>
}

#[derive(Clone)]
pub struct VulkanTexture
{
	pub texture_image: VkImage,
	pub texture_image_memory: VkDeviceMemory,
	pub texture_image_view: VkImageView,
	pub texture_sampler: VkSampler,
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
				Ok(Texture (
					LoadedTexture { 
						image: Box::new(TGAImage::new(self.file_path.clone())?) 
					}
				))
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
	pub fn process_vk(self, vk_handle: &VkHandle) -> Result<Texture<VulkanTexture>, ()>
	{
		unsafe
		{
			let (texture_image, texture_image_memory) = create_texture_image(&vk_handle, &self.image);
			let texture_image_view = create_texture_image_view(&vk_handle, &texture_image);
			let texture_sampler = create_texture_sampler(&vk_handle).unwrap();

			Ok(
				Texture (
					VulkanTexture { 
						texture_image: texture_image, 
						texture_image_memory: texture_image_memory, 
						texture_image_view: texture_image_view, 
						texture_sampler: texture_sampler
					}
				)
			)
		}
	}	
}