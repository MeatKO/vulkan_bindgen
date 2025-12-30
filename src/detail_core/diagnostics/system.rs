use decs2::{component_derive::system};
use decs2::manager::dECSManager;

use crate::detail_core::components::misc::{CameraRaycastObject, DeltaTime, CameraRaycastObjectState::*};
use crate::detail_core::phys::aabb::AABB;

#[system]
pub fn print_delta_time_system()
{
	// let main_loop_var = 
	// 	match decs.get_components_global::<DeltaTime>()
	// 		{
	// 			Ok(delta_time_vec) =>  { delta_time_vec.into_iter().next().unwrap() }
	// 			Err(_) => { return }
	// 		};

	// println!("Last delta time : {}", main_loop_var.last_delta_time_sec);

	// let raycast_object: &mut CameraRaycastObject =
	// 	unsafe { decs.get_components_global_mut_unchecked::<CameraRaycastObject>() }.unwrap().remove(0).component;

	// let aabb_vector: Vec<QueryResultMut<AABB>> = 
	// 	unsafe {decs.get_components_global_mut_unchecked::<AABB>() }.unwrap();

	// match &raycast_object.state
	// {
	// 	Picked(picked_object) =>
	// 	{
	// 		println!("Picked object info : {:?}", picked_object);
	// 		println!("Picked object AABB info : {:?}", aabb_vector[picked_object.index].component);
	// 	}
	// 	Thrown(thrown_object) => { println!("Throwing object : {:?}", thrown_object); },
	// 	None => { println!("No picked object"); },
	// }

	
}