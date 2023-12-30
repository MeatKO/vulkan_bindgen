use std::array::TryFromSliceError;
use std::path::Path;

use crate::pixcell::format::ImageFormat;

use super::error::TextureLoadError;
// use super::image::Image;
use super::image::{ImageDimensions, GenericImage};

const fn size_of_item<T>(x: &[T]) -> usize {
    std::mem::size_of::<T>()
}

const TGA_HEADER_SIZE: usize = 18;

// http://tfc.duke.free.fr/coding/tga_specs.pdf
#[repr(C)]
struct TGAHeader
{
	id_length: u8,
	color_map_type: u8,
	data_type_code: u8,
	color_map_origin: u16,
	color_map_length: u16,
	color_map_depth: u8,
	x_origin: u16,
	y_origin: u16,
	width: u16,
	height: u16,
	bits_per_pixel: u8,
	image_descriptor: u8,
}

pub struct TGAImage
{
	path: String,
	header: TGAHeader,
	format: ImageFormat,
	// data: Vec<u8>,
	data: Vec<u32>,
}

// impl Image for TGAImage
// {
//     fn get_data_ref(&self) -> &Vec<u8> 
// 	{
//         return &self.data;
//     }

//     fn get_dimensions(&self) -> ImageDimensions 
// 	{
//         return ImageDimensions{
//             width: self.header.width as u32,
//             height: self.header.height as u32,
//         };
//     }

//     fn get_format(&self) -> ImageFormat 
// 	{
// 		return self.format.clone();
// 	}

// 	fn get_byte_size(&self) -> usize 
// 	{
// 		return self.data.len();
// 	}
// }

impl TGAImage
{
	pub fn to_generic(self) -> GenericImage
	{
		return 
			GenericImage::new( 
				self.data, 
				self.path,
				self.format, 
				ImageDimensions{
					width: self.header.width as u32,
					height: self.header.height as u32,
				}
			)
	}
	
	pub fn new<P>(tga_path: P) -> Result<TGAImage, TextureLoadError>
	where P : AsRef<Path> + Clone
	{
		let start = std::time::Instant::now();

		let file_bytes = std::fs::read(tga_path.clone()).map_err(|err| TextureLoadError::IoError(err))?;

		if file_bytes.len() < TGA_HEADER_SIZE
		{
			return Err(TextureLoadError::CorruptHeader("file is not big enough to contain a tga header".to_owned()));
		}

		let mut header = 
			TGAHeader {
				id_length: file_bytes[0],
				color_map_type: file_bytes[1],
				data_type_code: file_bytes[2],
				color_map_origin: u16::from_le_bytes(file_bytes[3..5].try_into().map_err(|err: TryFromSliceError| TextureLoadError::CorruptHeader(err.to_string()))?),
				color_map_length: u16::from_le_bytes(file_bytes[5..7].try_into().map_err(|err: TryFromSliceError| TextureLoadError::CorruptHeader(err.to_string()))?),
				color_map_depth: file_bytes[7],
				x_origin: u16::from_le_bytes(file_bytes[8..10].try_into().map_err(|err: TryFromSliceError| TextureLoadError::CorruptHeader(err.to_string()))?),
				y_origin: u16::from_le_bytes(file_bytes[10..12].try_into().map_err(|err: TryFromSliceError| TextureLoadError::CorruptHeader(err.to_string()))?),
				width: u16::from_le_bytes(file_bytes[12..14].try_into().map_err(|err: TryFromSliceError| TextureLoadError::CorruptHeader(err.to_string()))?),
				height: u16::from_le_bytes(file_bytes[14..16].try_into().map_err(|err: TryFromSliceError| TextureLoadError::CorruptHeader(err.to_string()))?),
				bits_per_pixel: file_bytes[16],
				image_descriptor: file_bytes[17],
			};

		if header.width == 0 || header.height == 0 || header.bits_per_pixel == 0
		{
			return Err(TextureLoadError::CorruptHeader(format!("invalid image dimensions w:{} h:{} bpp:{}", header.width, header.height, header.bits_per_pixel).to_owned()));
		}

		let format = 
			match header.bits_per_pixel
			{
				24 => { ImageFormat::RGB }
				32 => { ImageFormat::RGBA }
				bpp => { return Err(TextureLoadError::CorruptData(format!("unsupported pixel width {}bpp", bpp).to_owned())); }
			};

		// since the only two types of tga files we read are of type 2 and 10, we don't have a color-map section
		match header.data_type_code
		{
			0 => { return Err(TextureLoadError::CorruptData("tga file does not contain image data".to_owned())) }
			2 | 10 => {}
			_ => { return Err(TextureLoadError::CorruptData("only true-color tga files are supported".to_owned())) }
		}

		// id_length contains the length of the image identifier block
		let read_offset: usize = TGA_HEADER_SIZE + header.id_length as usize;
		let image_byte_size: usize = header.width as usize * header.height  as usize  * (header.bits_per_pixel  as usize / 8);

		// println!("estimated image pixel size : {}", header.width as usize * header.height  as usize);
		// println!("estimated image byte size : {}", image_byte_size);
		// println!("file len : {}", file_bytes.len());

		// not sure if < or <= 
		if file_bytes.len() < read_offset + image_byte_size
		{
			return Err(TextureLoadError::CorruptData("estimated image size is larger than file size".to_owned()));
		}

		let image_data = file_bytes[read_offset..(read_offset + image_byte_size)].to_vec();

		// let final_image: Vec<u8>;
		let final_image: Vec<u32>;

		match format
		{
			ImageFormat::RGBA => 
			{
				if image_data.len() % 4 != 0
				{
					return Err(TextureLoadError::CorruptData("RGBA Image data was expected but the data array was not divisible by 4".to_owned()));
				}

				final_image = 
					image_data
					.chunks(4)
					// .map(|rgba| [rgba[2], rgba[1], rgba[0], rgba[3]])
					// .flatten()
					// .collect::<Vec<u8>>();
					.map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap_or([0u8; 4])))
					.collect::<Vec<u32>>();
			}
			ImageFormat::RGB =>
			{
				if image_data.len() % 3 != 0
				{
					return Err(TextureLoadError::CorruptData("RGB Image data was expected but the data array was not divisible by 3".to_owned()));
				}

				// We will also need to convert BGR to RGBA because of RGB compatibility issues
				final_image = 
					image_data
					.chunks(3)
					.map(|rgb| [rgb[2], rgb[1], rgb[0], 255u8])
					// .map(|rgb| [rgb[2], rgb[1], rgb[0]])
					// .flatten()
					// .collect::<Vec<u8>>();
					.map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap_or([0u8; 4])))
					.collect::<Vec<u32>>();

				header.bits_per_pixel = 32;
			}
		}

		let end = std::time::Instant::now();

		println!("Texture loading time : {:?}", end.duration_since(start));
		println!("image dimensions w:{} h:{} bpp:{}", header.width, header.height, header.bits_per_pixel);
		println!("original byte length {}", image_data.len());
		println!("processed byte length {}", final_image.len() * size_of_item(&final_image)); // for u8

		return Ok(
			TGAImage { 
				path: tga_path.as_ref().to_str().unwrap().to_owned(),
				header: header, 
				format: ImageFormat::RGBA,
				data:  final_image
			}
		);
	}
}