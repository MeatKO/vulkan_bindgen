pub trait VulkanWindowHandle : Sized
{
	fn new(window_title: Option<String>, width: u32, height: u32) -> Option<Self>;
}