use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::asset_manager::manager::AssetManager;
use crate::detail_core::components::misc::WindowComponent;
use crate::detail_core::model::asset::MaterialAsset;
use crate::detail_core::texture::texture::{VulkanTexture, Texture};
use crate::detail_core::window::create_vulkan_surface;
use crate::vulkan::descriptor_set::update_descriptor_sets;
use crate::vulkan::handle::VkHandle;

use crate::vulkan::shader::create_shader_module;
use crate::vulkan::vertex::Vertex;
use crate::vulkan::vk_bindgen::{VkCommandPoolCreateFlagBits, VkPolygonMode, VkFormat, VkDescriptorType};
use crate::vulkan::wrappers::vk_command_buffer::CommandBufferBuilder;
use crate::vulkan::wrappers::vk_command_pool::CommandPoolBuilder;
use crate::vulkan::wrappers::vk_descriptor_layout::VkDescriptorLayoutBuilder;
use crate::vulkan::wrappers::vk_descriptor_pool::VkDescriptorPoolBuilder;
use crate::vulkan::{
	swapchain::{
		create_swapchain, 
		create_swapchain_image_views
	},
	descriptor_set::create_descriptor_sets, 
	device::create_logical_device,
	framebuffer::create_framebuffers, 
	pipeline::create_pipeline, 
	physical_device::create_physical_device, 
	synchronization::create_synchronization_structures,
	depth_buffer::create_depth_buffer,
};

use std::vec;

#[system]
pub fn init_rendering_assets()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>().unwrap().remove(0).component };

	let asset_manager: &mut AssetManager =
		unsafe { decs.get_components_global_mut_unchecked::<AssetManager>().unwrap().remove(0).component };
	
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

	let material_defaults_descriptor_set = 
		unsafe 
		{
			create_descriptor_sets(
				&vk_handle, 
				&vk_handle.global_descriptor_pool_material, 
				&vk_handle.global_descriptor_set_layout_material, 
				1
			).unwrap()
		};

	unsafe 
	{
		update_descriptor_sets(
			&vk_handle, 
			&material_defaults_descriptor_set, 
			&default_albedo_map.clone(), 
			&default_normal_map.clone()
		).unwrap();
	}

	let material_defaults =	
		MaterialAsset {
			name: "default".into(),
			smooth_shading: true,
			descriptor_set: material_defaults_descriptor_set[0],
			albedo_map: default_albedo_map.clone(),
			normal_map: default_normal_map.clone(),
		};

	asset_manager.add_asset("material_defaults", material_defaults).unwrap();	
}

#[system]
pub fn init_window_handle()
{
	let window: &mut WindowComponent =
		unsafe { decs.get_components_global_mut_unchecked::<WindowComponent>() }.unwrap().remove(0).component;

	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

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
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

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
	
		vk_handle.global_descriptor_set_layout_ubo =
			VkDescriptorLayoutBuilder::new()
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
				.build(vk_handle.logical_device)
				.unwrap();

		vk_handle.global_descriptor_set_layout_material =
			VkDescriptorLayoutBuilder::new()
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.build(vk_handle.logical_device)
				.unwrap();

		vk_handle.global_descriptor_pool_material =
			VkDescriptorPoolBuilder::new()
			.add_pool_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 1000)
			.build(vk_handle.logical_device, 1000)
			.unwrap();

		vk_handle.global_descriptor_pool_ubo =
			VkDescriptorPoolBuilder::new()
			.add_pool_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 1000)
			.build(vk_handle.logical_device, 1000)
			.unwrap();
	}
}


#[system]
pub fn init_pipelines()
{
	unsafe
	{
		let vk_handle: &mut VkHandle =
			decs.get_components_global_mut_unchecked::<VkHandle>().unwrap().remove(0).component;

		// main forward shading
		{
			//
			// let vertex_shader_source = include_bytes!("../../../detail/shaders/normal/vert.spv");
			// let fragment_shader_source = include_bytes!("../../../detail/shaders/normal/frag.spv");
			let vertex_shader_source = include_bytes!("../../../detail/shaders/normal_new_layout/vert.spv");
			let fragment_shader_source = include_bytes!("../../../detail/shaders/normal_new_layout/frag.spv");
			//
			let vertex_shader_module = create_shader_module(&vk_handle, vertex_shader_source);
			let fragment_shader_module = create_shader_module(&vk_handle, fragment_shader_source);
			//
			let binding_description = Vertex::get_binding_description();
			let attribute_descriptions_vec = Vertex::get_attribute_descriptions();
			//

			let descriptor_set_layout = 
				VkDescriptorLayoutBuilder::new()
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.build(vk_handle.logical_device)
				.unwrap();

			let (pipeline_layout, render_pass, pipeline) = 
				create_pipeline(
					vk_handle, 
					vertex_shader_module, 
					fragment_shader_module, 
					binding_description, 
					attribute_descriptions_vec, 
					VkPolygonMode::VK_POLYGON_MODE_FILL,
					true,
					vec![
						vk_handle.global_descriptor_set_layout_ubo,
						vk_handle.global_descriptor_set_layout_material
					],
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

			let descriptor_set_layout = 
				VkDescriptorLayoutBuilder::new()
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
				// .add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				// .add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.build(vk_handle.logical_device)
				.unwrap();

			let (pipeline_layout, render_pass, pipeline) = 
				create_pipeline(
					vk_handle, 
					vertex_shader_module, 
					fragment_shader_module, 
					binding_description, 
					attribute_descriptions_vec,
					VkPolygonMode::VK_POLYGON_MODE_LINE, 
					false,
					vec![
						descriptor_set_layout
					]
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

			let descriptor_set_layout = 
				VkDescriptorLayoutBuilder::new()
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.add_binding(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)
				.build(vk_handle.logical_device)
				.unwrap();

			let (pipeline_layout_hud, render_pass_hud, pipeline_hud) = 
				create_pipeline(
					vk_handle, 
					vertex_shader_module, 
					fragment_shader_module, 
					binding_description, 
					attribute_descriptions_vec,
					VkPolygonMode::VK_POLYGON_MODE_FILL, 
					false,
					vec![
						descriptor_set_layout
					]
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
