use std::rc::Weak;

use decs::component_derive::system;
use decs::manager::dECS;

use crate::cotangens::vec3::Vec3;
use crate::detail_core::components::misc::StringComponent;
use crate::detail_core::components::rendering::UniformBufferComponent;
use crate::detail_core::model::asset::MaterialAsset;
use crate::detail_core::model::component::VulkanModelComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::Model;
// use crate::detail_core::model::model::{Model, VulkanModel};
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::descriptor_set::create_descriptor_sets;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn init_domatena_shtaiga_assets_2()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	// let material_defaults_ptr = decs.get_asset::<MaterialAsset>("material_defaults").unwrap();
	// let material_defaults = material_defaults_ptr.upgrade().unwrap().clone();

	// let mut tomato_crate = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).to_asset(&vk_handle).unwrap();
	let (tomato_crate, tomato_crate_materials) = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).to_asset_material(&vk_handle).unwrap();

	decs.add_asset("tomato_crate", tomato_crate).expect("couldnt add tomato_crate asset");

	for material in tomato_crate_materials.into_iter()
	{
		decs.add_asset(material.name.clone(), material).expect("couldnt add tomato_crate_material asset");
	}
}

#[system]
pub fn init_domatena_shtaiga_object()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	// let shtaiga_asset: Weak<Model<VulkanModel>> = decs.get_asset::<Model<VulkanModel>>("shtaiga").unwrap();

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
			// AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
		];

	for aabb in phys_boxes.iter_mut()
	{
		unsafe { aabb.process_vulkan(vk_handle) };
	}

	for i in 5..phys_boxes.len()
	{
		let h = i - 3;
		phys_boxes[i].translation = Vec3{ x: 0.0f32, y: (h * h) as f32, z: 0.0f32 };
		// models[i].aabb.scale = Vec3{ x: h as f32, y: h as f32, z: h as f32 };
		phys_boxes[i].mass = (h ) as f32;
	}

	for (index, aabb) in phys_boxes.into_iter().enumerate()
	{
		let shtaiga = decs.create_entity();

		let mut model_component = VulkanModelComponent::new("tomato_crate".into());
		// model_component.descriptor_pool = unsafe { create_descriptor_pool(vk_handle).unwrap() };
		// model_component.descriptor_sets = unsafe { create_descriptor_sets(vk_handle, &model_component.descriptor_pool).unwrap() };

		decs.add_component(shtaiga, StringComponent{ string : format!("shtaiga_{}", index).to_owned() }).unwrap();
		decs.add_component(shtaiga, aabb).unwrap();
		// decs.add_component(shtaiga, ModelComponent{model_asset: shtaiga_asset.clone()}).unwrap();
		decs.add_component(shtaiga, model_component).unwrap();
		let mut ubo = UniformBufferComponent::new(vk_handle).unwrap();
		

		decs.add_component(shtaiga, UniformBufferComponent::new(vk_handle).unwrap()).unwrap();
	}
}


#[system]
pub fn init_misc_assets()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	// let material_defaults_ptr = decs.get_asset::<MaterialAsset>("material_defaults").unwrap();
	// let material_defaults = material_defaults_ptr.upgrade().unwrap().clone();

	let model_name_vec =
		vec![
			// "viking_room".to_owned(),
			"woag".to_owned(),
			"earth_2".to_owned(),
		];

	for model_name in model_name_vec.into_iter()
	{
		let (model, materials) = 
			match Model::new(format!("./detail/models/{}/{}.obj", model_name, model_name).into()).to_asset_material(&vk_handle)
			{
				Ok((model, materials)) => (model, materials),
				Err(err) => { println!("Couldn't load Model {} : {}", model_name, err ); continue; }
			};

		match decs.add_asset(model_name.clone(), model)
		{
			Ok(_) => {},
			Err(err) => { println!("Couldn't add Asset {} : {}", model_name, err); continue; }
		}

		for material in materials.into_iter()
		{
			let material_name = material.name.clone();
			match decs.add_asset(&material_name, material)
			{
				Ok(_) => {},
				Err(_) => { println!("Couldn't load Material {}", material_name); continue; }
			}
		}
	}

	// let mut tomato_crate = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).to_asset(&vk_handle).unwrap();
	// let (tomato_crate, tomato_crate_materials) = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).to_asset_material(&vk_handle).unwrap();

	// decs.add_asset("tomato_crate", tomato_crate).expect("couldnt add tomato_crate asset");

	// for material in tomato_crate_materials.into_iter()
	// {
	// 	decs.add_asset(material.name.clone(), material).expect("couldnt add tomato_crate_material asset");
	// }
}

// #[system]
// pub fn init_misc_objects()
// {
// 	let vk_handle: &mut VkHandle =
// 		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

// 	// let shtaiga_asset: Weak<Model<VulkanModel>> = decs.get_asset::<Model<VulkanModel>>("shtaiga").unwrap();

// 	// let mut phys_boxes: Vec<AABB> =
// 	// 	vec![
// 	// 		AABB::new_nonverbose(Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32 }, Vec3{ x: 100.0f32, y: 1.0f32, z: 100.0f32 }, true),
// 	// 		AABB::new_nonverbose(Vec3{ x: 1.0f32, y: -1.0f32, z: 100.0f32 }, Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 }, true),
// 	// 		AABB::new_nonverbose(Vec3{ x: 1.0f32, y: -1.0f32, z: -100.0f32 }, Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 }, true),
// 	// 		AABB::new_nonverbose(Vec3{ x: 100.0f32, y: -1.0f32, z: 1.0f32 }, Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 }, true),
// 	// 		AABB::new_nonverbose(Vec3{ x: -100.0f32, y: -1.0f32, z: 1.0f32 }, Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 }, true),

// 	// 		AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
// 	// 		AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
// 	// 		AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
// 	// 		// AABB::new_nonverbose(Vec3::new(0.0f32), Vec3::new(1.0f32), false),
// 	// 	];

// 	// for aabb in phys_boxes.iter_mut()
// 	// {
// 	// 	unsafe { aabb.process_vulkan(vk_handle) };
// 	// }

// 	// for i in 5..phys_boxes.len()
// 	// {
// 	// 	let h = i - 3;
// 	// 	phys_boxes[i].translation = Vec3{ x: 0.0f32, y: (h * h) as f32, z: 0.0f32 };
// 	// 	// models[i].aabb.scale = Vec3{ x: h as f32, y: h as f32, z: h as f32 };
// 	// 	phys_boxes[i].mass = (h ) as f32;
// 	// }

// 	for (index, aabb) in phys_boxes.into_iter().enumerate()
// 	{
// 		let shtaiga = decs.create_entity();

// 		let mut model_component = VulkanModelComponent::new("tomato_crate".into());
// 		// model_component.descriptor_pool = unsafe { create_descriptor_pool(vk_handle).unwrap() };
// 		// model_component.descriptor_sets = unsafe { create_descriptor_sets(vk_handle, &model_component.descriptor_pool).unwrap() };

// 		decs.add_component(shtaiga, StringComponent{ string : format!("shtaiga_{}", index).to_owned() }).unwrap();
// 		decs.add_component(shtaiga, aabb).unwrap();
// 		// decs.add_component(shtaiga, ModelComponent{model_asset: shtaiga_asset.clone()}).unwrap();
// 		decs.add_component(shtaiga, model_component).unwrap();
// 		let mut ubo = UniformBufferComponent::new(vk_handle).unwrap();
		

// 		decs.add_component(shtaiga, UniformBufferComponent::new(vk_handle).unwrap()).unwrap();
// 	}
// }