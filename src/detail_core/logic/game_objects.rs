use decs::component_derive::system;
use decs::manager::dECS;

use crate::cotangens::vec3::Vec3;
use crate::detail_core::components::misc::StringComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::{Model, VulkanModel};
use crate::detail_core::texture::texture::{Texture, VulkanTexture};
use crate::vulkan::handle::VkHandle;
use crate::vulkan::vk_bindgen::VkFormat;

#[system]
pub fn init_domatena_shtaiga_object()
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

			let mut models: Vec<Model<VulkanModel>> =
			vec![
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
			];

	models[0].aabb.translation = Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32 };
	models[0].aabb.scale = Vec3{ x: 100.0f32, y: 1.0f32, z: 100.0f32 };
	models[0].aabb.is_static = true;

	models[1].aabb.translation = Vec3{ x: 1.0f32, y: -1.0f32, z: 100.0f32 };
	models[1].aabb.scale = Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 };
	models[1].aabb.is_static = true;

	models[2].aabb.translation = Vec3{ x: 1.0f32, y: -1.0f32, z: -100.0f32 };
	models[2].aabb.scale = Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 };
	models[2].aabb.is_static = true;

	models[3].aabb.translation = Vec3{ x: 100.0f32, y: -1.0f32, z: 1.0f32 };
	models[3].aabb.scale = Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 };
	models[3].aabb.is_static = true;

	models[4].aabb.translation = Vec3{ x: -100.0f32, y: -1.0f32, z: 1.0f32 };
	models[4].aabb.scale = Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 };
	models[4].aabb.is_static = true;

	for i in 5..models.len()
	{
		let h = i - 3;
		models[i].aabb.translation = Vec3{ x: 0.0f32, y: (h * h) as f32, z: 0.0f32 };
		// models[i].aabb.scale = Vec3{ x: h as f32, y: h as f32, z: h as f32 };
		models[i].aabb.mass = (h ) as f32;
	}
	
	for model in models.iter_mut()
	{
		match model.process_textures(&vk_handle)
		{
			Ok(()) => {},
			Err(err) => { println!("couldn't parse textures for model '{}' err : '{}'", model.name, err) }
		}
	}

	for (index, model) in models.into_iter().enumerate()
	{
		let shtaiga = decs.create_entity();
		decs.add_component(shtaiga, StringComponent{ string : format!("shtaiga_{}", index).to_owned() }).unwrap();
		decs.add_component(shtaiga, model).unwrap();
	}
}