use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::components::misc::StringComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::Model;
use crate::detail_core::texture::texture::{Texture, VulkanTexture};
use crate::vulkan::handle::VkHandle;
use crate::vulkan::vk_bindgen::VkFormat;

#[system]
pub fn init_domatena_shtaiga_object()
{
	let shtaiga = decs.create_entity();
	decs.add_component(shtaiga, StringComponent{ string : String::from("shtaiga_1") }).unwrap();

	let vk_handle = 
		match decs.get_components_global::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};


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

	let model = 
		Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone());
				
	decs.add_component(shtaiga, model).unwrap();

}