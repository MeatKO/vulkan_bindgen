mod vulkan;
use vulkan::{
	vk_bindgen::*, device::*,handle::VkHandle, swapchain::*,
	draw::*, framebuffer::*, command_pool::*, command_buffer::*,
	pipeline::*, instance::*, physical_device::*, synchronization::*,
};

mod loseit;
use loseit::window::*;

mod ffi;
mod calcium;

fn main() 
{
	unsafe
	{
		let mut vk_handle = VkHandle::new_empty();

		create_instance(&mut vk_handle);

		let _window = 
			Window::new()
			.with_title("deta:l")
			.with_dimensions(400, 400)
			.build_vulkan(&mut vk_handle);

		create_physical_device(&mut vk_handle);
	
		create_logical_device(&mut vk_handle);
		
		create_swapchain(&mut vk_handle);

		create_swapchain_image_views(&mut vk_handle);

		create_pipeline(&mut vk_handle);

		create_framebuffer(&mut vk_handle);

		create_command_pool(&mut vk_handle);

		create_command_buffer(&mut vk_handle);

		// creating semaphores & fence
		create_synchronization_structures(&mut vk_handle);
		
		// loop 
		// {
			draw_frame(&mut vk_handle);
		// }
		
		vkDeviceWaitIdle(vk_handle.logical_device);

		std::thread::sleep(std::time::Duration::from_secs(2));

		// Cleanup
		println!("Destroying vk objects...");
		
		vk_handle.destroy_vk_resources();
	}
}
