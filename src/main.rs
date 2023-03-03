mod ffi;
mod cotangens;

mod loseit;
use loseit::{
	window::*, window_events::*,
};

mod vulkan;
use vulkan::{
	vk_bindgen::*, device::*,handle::VkHandle, swapchain::*,
	draw::*, framebuffer::*, command_pool::*, command_buffer::*,
	pipeline::*, instance::*, physical_device::*, synchronization::*,
	vertex::*, uniform_buffer::*, descriptor_set::*, descriptor_pool::*,
	descriptor_set::*,
};

use crate::vulkan::index::create_index_buffer;

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

		create_command_pool(&mut vk_handle);

		create_vertex_buffer(&mut vk_handle);

		create_index_buffer(&mut vk_handle);

		create_uniform_buffers(&mut vk_handle);

		create_descriptor_set_layout(&mut vk_handle);

		create_descriptor_pool(&mut vk_handle);

		create_descriptor_sets(&mut vk_handle);

		create_pipeline(&mut vk_handle);

		create_framebuffer(&mut vk_handle);

		create_synchronization_structures(&mut vk_handle);

		create_command_buffer(&mut vk_handle);
		
		// I will eventually add window event handling <.<
		'main_loop: loop 
		{
			loop {
				match _window.get_event()
				{
					// _ => {}
					Some(e) => 
					{
						match e 
						{
							WindowEvent::KeyPress(val) => 
							{ 
								println!("a key press {:?}!", val);

								match val
								{
									KeyValues::ESC => { break 'main_loop; }
									_ => {}
								}
							}
							_ => { println!("useless event") }
						}
					}
					None => { break }
				};
			}
			
			draw_frame(&mut vk_handle);
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
		
		vkDeviceWaitIdle(vk_handle.logical_device);

		// std::thread::sleep(std::time::Duration::from_secs(2));

		// Cleanup
		println!("Destroying vk objects...");
		
		vk_handle.destroy_vk_resources();
	}
}
