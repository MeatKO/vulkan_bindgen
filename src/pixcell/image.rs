use super::format::ImageFormat;

pub struct ImageSize
{
	pub width: u32,
	pub height: u32,
}

pub trait Image
{
	fn get_data_ref(&self) -> &Vec<u8>;
	fn get_format(&self) -> ImageFormat;
	fn get_size(&self) -> ImageSize;
}