use std::rc::Weak;

use decs::component_derive::system;
use decs::manager::dECS;

use crate::cotangens::vec3::Vec3;
use crate::detail_core::components::misc::StringComponent;
use crate::detail_core::components::rendering::{ModelComponent, UniformBufferComponent};
use crate::detail_core::model::component::VulkanModelComponent;
use crate::detail_core::model::material::Material;
use crate::detail_core::model::model::{Model, VulkanModel};
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::descriptor_set::create_descriptor_sets;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn init_domatena_shtaiga_assets()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let material_defaults_ptr = decs.get_asset::<Material>("material_defaults").unwrap();
	let material_defaults = material_defaults_ptr.upgrade().unwrap().clone();

	let mut shtaiga = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.as_ref().clone());
	match shtaiga.process_textures(&vk_handle)
	{
		Ok(()) => {},
		Err(err) => { println!("couldn't parse textures for model '{}' err : '{}'", shtaiga.name, err) }
	}

	decs.add_asset("shtaiga", shtaiga).expect("couldnt add shtaiga asset");
}

#[system]
pub fn init_domatena_shtaiga_assets_2()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let material_defaults_ptr = decs.get_asset::<Material>("material_defaults").unwrap();
	let material_defaults = material_defaults_ptr.upgrade().unwrap().clone();

	let mut tomato_crate = Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).to_asset(&vk_handle).unwrap();

	decs.add_asset("tomato_crate", tomato_crate).expect("couldnt add tomato_crate asset");
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