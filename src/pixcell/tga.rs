use core::time;
use std::array::TryFromSliceError;

use crate::pixcell::format::ImageFormat;

const TGA_HEADER_SIZE: usize = 18;

// http://tfc.duke.free.fr/coding/tga_specs.pdf
#[repr(C)]
pub struct TGAHeader
{
	id_length: u8,
	color_map_type: u8,
	data_type_code: u8,
	color_map_origin: u16,
	color_map_length: u16,
	color_map_depth: u8,
	x_origin: u16,
	y_origin: u16,
	pub width: u16,
	pub height: u16,
	pub bits_per_pixel: u8,
	image_descriptor: u8,
}

pub struct TGAImage
{
	pub header: TGAHeader,
	format: ImageFormat,
	pub data: Vec<u8>,
}

impl TGAImage
{
	pub fn new<T: ToString>(tga_path: T) -> Result<TGAImage, String>
	{
		let start = std::time::Instant::now();

		let file_bytes = std::fs::read(tga_path.to_string()).map_err(|err| err.to_string())?;

		if file_bytes.len() < TGA_HEADER_SIZE
		{
			return Err("file is not big enough to contain a tga header".to_owned())
		}

		let mut header = 
			TGAHeader {
				id_length: file_bytes[0],
				color_map_type: file_bytes[1],
				data_type_code: file_bytes[2],
				color_map_origin: u16::from_le_bytes(file_bytes[3..5].try_into().map_err(|err: TryFromSliceError| err.to_string())?),
				color_map_length: u16::from_le_bytes(file_bytes[5..7].try_into().map_err(|err: TryFromSliceError| err.to_string())?),
				color_map_depth: file_bytes[7],
				x_origin: u16::from_le_bytes(file_bytes[8..10].try_into().map_err(|err: TryFromSliceError| err.to_string())?),
				y_origin: u16::from_le_bytes(file_bytes[10..12].try_into().map_err(|err: TryFromSliceError| err.to_string())?),
				width: u16::from_le_bytes(file_bytes[12..14].try_into().map_err(|err: TryFromSliceError| err.to_string())?),
				height: u16::from_le_bytes(file_bytes[14..16].try_into().map_err(|err: TryFromSliceError| err.to_string())?),
				bits_per_pixel: file_bytes[16],
				image_descriptor: file_bytes[17],
			};

		if header.width == 0 || header.height == 0 || header.bits_per_pixel == 0
		{
			return Err(format!("invalid image dimensions w:{} h:{} bpp:{}", header.width, header.height, header.bits_per_pixel).to_owned()) 
		}

		let format = 
			match header.bits_per_pixel
			{
				24 => { ImageFormat::RGB }
				32 => { ImageFormat::RGBA }
				bpp => { return Err(format!("unsupported pixel width {}bpp", bpp).to_owned()) }
			};

		// since the only two types of tga files we read are of type 2 and 10, we don't have a color-map section
		match header.data_type_code
		{
			0 => { return Err("tga file does not contain image data".to_owned()) }
			2 | 10 => {}
			_ => { return Err("only true-color tga files are supported".to_owned()) }
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
			return Err("estimated image size is larger than file size".to_owned()) 
		}

		let image_data = file_bytes[read_offset..(read_offset + image_byte_size)].to_vec();

		let final_image: Vec<u8>;

		match format
		{
			ImageFormat::RGBA => 
			{
				if image_data.len() % 4 != 0
				{
					return Err("RGBA Image data was present but the data array was not divisible by 4".to_owned())
				}

				final_image = 
					image_data
					.chunks(4)
					.map(|rgba| [rgba[2], rgba[1], rgba[0], rgba[3]])
					.flatten()
					.collect::<Vec<u8>>();
			}
			ImageFormat::RGB =>
			{
				if image_data.len() % 3 != 0
				{
					return Err("RGB Image data was present but the data array was not divisible by 3".to_owned())
				}

				// We will also need to convert BGR to RGBA because of RGB compatibility issues
				final_image = 
					image_data
					.chunks(3)
					.map(|rgb| [rgb[2], rgb[1], rgb[0], 255u8])
					.flatten()
					.collect::<Vec<u8>>();

				header.bits_per_pixel = 32;
			}
		}

		let end = std::time::Instant::now();

		println!("Texture loading time : {:?}", end.duration_since(start));
		println!("image dimensions w:{} h:{} bpp:{}", header.width, header.height, header.bits_per_pixel);
		println!("original byte length {}", image_data.len());
		println!("processed byte length {}", final_image.len());

		return Ok(
			TGAImage { 
				header: header, 
				format: ImageFormat::RGBA,
				data:  final_image
			}
		);
	}
}