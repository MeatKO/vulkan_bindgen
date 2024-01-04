use decs::component_derive::system;
use decs::manager::dECS;

use crate::cotangens::vec3::Vec3;
use crate::detail_core::asset_manager::manager::AssetManager;
use crate::detail_core::components::misc::StringComponent;
use crate::detail_core::components::rendering::UniformBufferComponent;
use crate::detail_core::model::asset::MaterialAsset;
use crate::detail_core::model::component::VulkanModelComponent;
use crate::detail_core::model::model::Model;
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn init_domatena_shtaiga_assets_2()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let asset_manager: &mut AssetManager =
		unsafe { decs.get_components_global_mut_unchecked::<AssetManager>().unwrap().remove(0).component };
	
	let default_material =
		asset_manager.get_asset_rc::<MaterialAsset>("material_defaults")
		.unwrap();
	
	let (tomato_crate, tomato_crate_materials) = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).to_asset_material(&vk_handle, &default_material).unwrap();

	asset_manager.add_asset("tomato_crate", tomato_crate).expect("couldnt add tomato_crate asset");

	for material in tomato_crate_materials.into_iter()
	{
		asset_manager.add_asset(material.name.clone(), material).expect("couldnt add tomato_crate_material asset");
	}
}

#[system]
pub fn init_domatena_shtaiga_object()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let mut phys_boxes: Vec<AABB> =
		vec![
			AABB::new_nonverbose(Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32 }, Vec3{ x: 100.0f32, y: 1.0f32, z: 100.0f32 }, true),
			AABB::new_nonverbose(Vec3{ x: 1.0f32, y: -1.0f32, z: 100.0f32 }, Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 }, true),
			AABB::new_nonverbose(Vec3{ x: 1.0f32, y: -1.0f32, z: -100.0f32 }, Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 }, true),
			AABB::new_nonverbose(Vec3{ x: 100.0f32, y: -1.0f32, z: 1.0f32 }, Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 }, true),
			AABB::new_nonverbose(Vec3{ x: -100.0f32, y: -1.0f32, z: 1.0f32 }, Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 }, true),

			AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
			AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
			AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
		];

	for aabb in phys_boxes.iter_mut()
	{
		unsafe { aabb.process_vulkan(vk_handle) };
	}

	for i in 5..phys_boxes.len()
	{
		let h = i - 3;
		phys_boxes[i].translation = Vec3{ x: 0.0f32, y: (h * h) as f32, z: 0.0f32 };
		phys_boxes[i].mass = (h ) as f32;
	}

	for (index, aabb) in phys_boxes.into_iter().enumerate()
	{
		let shtaiga = decs.create_entity();

		let model_component = VulkanModelComponent::new("tomato_crate".into());

		decs.add_component(shtaiga, StringComponent{ string : format!("shtaiga_{}", index).to_owned() }).unwrap();
		decs.add_component(shtaiga, aabb).unwrap();
		decs.add_component(shtaiga, model_component).unwrap();
		decs.add_component(shtaiga, UniformBufferComponent::new(vk_handle).unwrap()).unwrap();
	}
}


#[system]
pub fn init_misc_assets()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let asset_manager: &mut AssetManager =
		unsafe { decs.get_components_global_mut_unchecked::<AssetManager>().unwrap().remove(0).component };

	let default_material =
	asset_manager.get_asset_rc::<MaterialAsset>("material_defaults")
		.unwrap();
	
	let model_name_vec =
		vec![
			"valkyrie".to_owned(),
		];

	for model_name in model_name_vec.into_iter()
	{
		let (model, materials) = 
			Model::new(format!("./detail/models/{}/{}.obj", model_name, model_name).into()).to_asset_material(&vk_handle, &default_material).unwrap();
			// match Model::new(format!("./detail/models/{}/{}.obj", model_name, model_name).into()).to_asset_material(&vk_handle)
			// {
			// 	Ok((model, materials)) => (model, materials),
			// 	Err(err) => { println!("Couldn't load Model {} : {}", model_name, err ); continue; }
			// };

		match asset_manager.add_asset(model_name.clone(), model)
		{
			Ok(_) => {},
			Err(err) => { println!("Couldn't add Asset {} : {}", model_name, err); continue; }
		}

		for material in materials.into_iter()
		{
			let material_name = material.name.clone();
			asset_manager.add_asset(&material_name, material).unwrap();
			// match decs.add_asset(&material_name, material)
			// {
			// 	Ok(_) => {},
			// 	Err(_) => { println!("Couldn't load Material {}", material_name); continue; }
			// }
		}
	}
}

#[system]
pub fn init_misc_objects()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let shtaiga = decs.create_entity();

	let mut aabb = AABB::new_nonverbose(Vec3::new(3.0f32), Vec3::new(1.0f32), false);
	unsafe { aabb.process_vulkan(vk_handle) };

	decs.add_component(shtaiga, StringComponent{ string : "valkyrie".to_owned()}).unwrap();
	decs.add_component(shtaiga, VulkanModelComponent::new("valkyrie".into())).unwrap();
	decs.add_component(shtaiga, aabb).unwrap();
	decs.add_component(shtaiga, UniformBufferComponent::new(vk_handle).unwrap()).unwrap();
}