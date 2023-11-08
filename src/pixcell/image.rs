use super::format::ImageFormat;

#[derive(Clone)]
pub struct ImageDimensions
{
	pub width: u32,
	pub height: u32,
}

pub struct GenericImage
{
	data: Vec<u32>,
	format: ImageFormat,
	dimensions: ImageDimensions,
}

const fn size_of_item<T>(x: &[T]) -> usize {
    std::mem::size_of::<T>()
}

impl GenericImage
{
	pub fn new(data: Vec<u32>, format: ImageFormat, dimensions: ImageDimensions) -> Self
	// pub fn new(data: Vec<u8>, format: ImageFormat, dimensions: ImageDimensions) -> Self
	{
		GenericImage { 
			data: data, 
			format: format, 
			dimensions: dimensions 
		}
	}

	pub fn set_data_u32(&mut self, new_data: Vec<u32>)
	{
		return self.data = new_data;
	}

	// pub fn set_data_u8(&mut self, new_data: Vec<u8>)
	// {
	// 	return self.data = new_data;
	// }

	pub fn set_dimensions(&mut self, new_dimensions:ImageDimensions)
	{
		return self.dimensions = new_dimensions;
	}

	pub fn get_data_u8_ptr(&self) -> *const u8
	{
		return self.data.as_ptr() as _
	}

	pub fn get_data_u32_ref(&self) -> &Vec<u32>
	{
		return &self.data
	}

	// pub fn get_data_u8_ref(&self) -> &Vec<u8>
	// {
	// 	return &self.data
	// }

	pub fn get_format(&self) -> ImageFormat
	{	
		return self.format.clone()
	}

	pub fn get_dimensions(&self) -> ImageDimensions
	{
		return self.dimensions.clone()
	}

	pub fn get_byte_size(&self) -> usize
	{
		return self.data.len() * size_of_item(&self.data)
		// return self.data.len()
	}
}

// pub trait Image
// {
// 	fn get_data_ref(&self) -> &Vec<u8>;
// 	fn get_format(&self) -> ImageFormat;
// 	fn get_dimensions(&self) -> ImageDimensions;
// 	fn get_byte_size(&self) -> usize;
// }