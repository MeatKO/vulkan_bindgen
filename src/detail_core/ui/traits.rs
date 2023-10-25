use super::button::VulkanButtonData;

pub trait HUDElement 
{
	fn get_vulkan_data(&self) -> Option<VulkanButtonData>;
	fn is_inside(&self, cursor_pos_x: i32, cursor_pos_y: i32) -> bool;
}