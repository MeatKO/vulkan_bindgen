use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::components::misc::WindowComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::Model;
use crate::detail_core::texture::texture::{VulkanTexture, Texture};
use crate::detail_core::window::create_vulkan_surface;
use crate::vulkan::handle::VkHandle;

use crate::vulkan::shader::create_shader_module;
use crate::vulkan::vertex::Vertex;
use crate::vulkan::vk_bindgen::{VkCommandPoolCreateFlagBits, VkPolygonMode, VkFormat};
use crate::vulkan::wrappers::vk_command_buffer::CommandBufferBuilder;
use crate::vulkan::wrappers::vk_command_pool::CommandPoolBuilder;
use crate::vulkan::{
	vk_bindgen::
		vkDeviceWaitIdle, 
	texture_view::{
		create_texture_image_view, 
		create_texture_sampler
	}, 
	swapchain::{
		create_swapchain, 
		create_swapchain_image_views
	},
	descriptor_set::{
		create_descriptor_set_layout, 
		create_descriptor_sets
	}, 
	device::create_logical_device,
	framebuffer::create_framebuffers, 
	pipeline::create_pipeline, 
	instance::create_instance, 
	physical_device::create_physical_device, 
	synchronization::create_synchronization_structures,
	depth_buffer::create_depth_buffer,
};

#[system]
pub fn init_rendering_assets()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let default_normal_map: Texture<VulkanTexture> = 
	Texture::new("./detail/textures/smiley_normal.tga".into())
	.load()
	.unwrap()
	.process_vk(
		&vk_handle, 
		VkFormat::VK_FORMAT_R8G8B8A8_UNORM
	)
	.unwrap();

	let default_albedo_map: Texture<VulkanTexture> = 
		Texture::new("./detail/textures/test.tga".into())
		.load()
		.unwrap()
		.process_vk(
			&vk_handle, 
			VkFormat::VK_FORMAT_R8G8B8A8_SRGB
		)
		.unwrap();

	let material_defaults =	
		Material {
			name: "default".into(),
			albedo_path: "unused".to_string(),
			normal_path: "unused".to_string(),
			albedo_map: Some(default_albedo_map.clone()),
			normal_map: Some(default_normal_map.clone()),
		};

	decs.add_asset("material_defaults", material_defaults.clone()).unwrap();

	let mut error_model = Model::new("./detail/models/error/error.obj".into()).process_meshes(&vk_handle, material_defaults.clone());
	match error_model.process_textures(&vk_handle)
	{
		Ok(()) => {},
		Err(err) => { println!("couldn't parse textures for model '{}' err : '{}'", error_model.name, err) }
	}

	decs.add_asset("error_model", error_model).expect("couldnt add error_model asset");
}

#[system]
pub fn init_window_handle()
{
	let window = 
		match decs.get_components_global::<WindowComponent>()
		{
			Ok(window_component_vec) => 
			{
				window_component_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	let vk_handle = 
		match decs.get_components_global::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	let surface = create_vulkan_surface(&window.window, vk_handle);

	let vk_handle = 
		match decs.get_components_global_mut::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	vk_handle.window_surface = surface.unwrap();
}

#[system]
pub fn init_rendering_objects()
{
	let vk_handle = 
		match decs.get_components_global_mut::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	
	// vk_handle.window_surface = create_vulkan_surface(&mut window, &mut vk_handle).unwrap();

	unsafe
	{
		// create_instance(vk_handle);

		create_physical_device(vk_handle);
		create_logical_device(vk_handle);
		
		create_swapchain(vk_handle);
		create_swapchain_image_views(vk_handle);
	
		let command_pool = 
			CommandPoolBuilder::new()
			.with_flag(VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT)
			.with_queue_family_index(vk_handle.queue_family_indices[0])
			.build(&vk_handle.logical_device)
			.unwrap();
		vk_handle.command_pool = Some(command_pool);
	
		vk_handle.descriptor_set_layout = create_descriptor_set_layout(&vk_handle.logical_device).unwrap();
	
	}
}


#[system]
pub fn init_pipelines()
{
	let vk_handle = 
		match decs.get_components_global_mut::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	unsafe
	{

		// main forward shading
		{
			//
			let vertex_shader_source = include_bytes!("../../../detail/shaders/normal/vert.spv");
			let fragment_shader_source = include_bytes!("../../../detail/shaders/normal/frag.spv");
			//
			let vertex_shader_module = create_shader_module(&vk_handle, vertex_shader_source);
			let fragment_shader_module = create_shader_module(&vk_handle, fragment_shader_source);
			//
			let binding_description = Vertex::get_binding_description();
			let attribute_descriptions_vec = Vertex::get_attribute_descriptions();
			//
			let (pipeline_layout, render_pass, pipeline) = 
				create_pipeline(
					vk_handle, 
					vertex_shader_module, 
					fragment_shader_module, 
					binding_description, 
					attribute_descriptions_vec, 
					VkPolygonMode::VK_POLYGON_MODE_FILL, 
					// VkPolygonMode::VK_POLYGON_MODE_LINE, 
					true
				);
			//
			vk_handle.pipeline_layout = pipeline_layout;
			vk_handle.render_pass = render_pass;
			vk_handle.graphics_pipeline = pipeline;
		}

		// wireframe
		{
			//
			let vertex_shader_source = include_bytes!("../../../detail/shaders/wireframe_hitbox/vert.spv");
			let fragment_shader_source = include_bytes!("../../../detail/shaders/wireframe_hitbox/frag.spv");
			//
			let vertex_shader_module = create_shader_module(&vk_handle, vertex_shader_source);
			let fragment_shader_module = create_shader_module(&vk_handle, fragment_shader_source);
			//
			let binding_description = Vertex::get_binding_description();
			let attribute_descriptions_vec = Vertex::get_attribute_descriptions();
			//
			let (pipeline_layout, render_pass, pipeline) = 
				create_pipeline(
					vk_handle, 
					vertex_shader_module, 
					fragment_shader_module, 
					binding_description, 
					attribute_descriptions_vec, 
					// VkPolygonMode::VK_POLYGON_MODE_FILL, 
					VkPolygonMode::VK_POLYGON_MODE_LINE, 
					false
				);
			//
			vk_handle.pipeline_layout_wireframe = pipeline_layout;
			vk_handle.render_pass_wireframe = render_pass;
			vk_handle.graphics_pipeline_wireframe = pipeline;
		}

		// hud
		{
			//
			let vertex_shader_source = include_bytes!("../../../detail/shaders/hud/vert.spv");
			let fragment_shader_source = include_bytes!("../../../detail/shaders/hud/frag.spv");
			//
			let vertex_shader_module = create_shader_module(&vk_handle, vertex_shader_source);
			let fragment_shader_module = create_shader_module(&vk_handle, fragment_shader_source);
			//
			let binding_description = Vertex::get_binding_description();
			let attribute_descriptions_vec = Vertex::get_attribute_descriptions();
			//
			let (pipeline_layout_hud, render_pass_hud, pipeline_hud) = 
				create_pipeline(
					vk_handle, 
					vertex_shader_module, 
					fragment_shader_module, 
					binding_description, 
					attribute_descriptions_vec,
					VkPolygonMode::VK_POLYGON_MODE_FILL, 
					false,
				);
			//
			vk_handle.pipeline_layout_hud = pipeline_layout_hud;
			vk_handle.render_pass_hud = render_pass_hud;
			vk_handle.graphics_pipeline_hud = pipeline_hud;
		}
	}
}

#[system]
pub fn init_buffer_objects()
{
	let vk_handle = 
		match decs.get_components_global_mut::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	unsafe
	{
		create_depth_buffer(vk_handle);
		create_framebuffers(vk_handle);

		create_synchronization_structures(vk_handle);

		let command_buffer_count = vk_handle.frames_in_flight as u32;

		{
			let command_buffer_graphics =
			CommandBufferBuilder::new()
			.with_command_pool(&vk_handle.command_pool.as_ref().unwrap())
			.with_count(command_buffer_count)
			.build(&vk_handle.logical_device)
			.unwrap();
			vk_handle.command_buffer_vec = command_buffer_graphics;
		}
		{
			let command_buffer_hud =
			CommandBufferBuilder::new()
			.with_command_pool(&vk_handle.command_pool.as_ref().unwrap())
			.with_count(command_buffer_count)
			.build(&vk_handle.logical_device)
			.unwrap();
			vk_handle.command_buffer_hud_vec = command_buffer_hud;
		}
		{
			let command_buffer_wireframe =
			CommandBufferBuilder::new()
			.with_command_pool(&vk_handle.command_pool.as_ref().unwrap())
			.with_count(command_buffer_count)
			.build(&vk_handle.logical_device)
			.unwrap();
			vk_handle.command_buffer_wireframe_vec = command_buffer_wireframe;
		}	
	}
}
