use std::vec;

use decs::component_derive::system;
use decs::manager::{dECS, QueryResultMut};

use crate::cotangens::vec3::Vec3;
use crate::detail_core::components::misc::{DeltaTime, RaycastObject, RaycastObjectState};
use crate::detail_core::model::model::{Model, VulkanModel};
use crate::vulkan::handle::VkHandle;

use super::aabb::AABB;

// #[system]
// pub fn physics_system()
// {
// 	let _ =
// 		decs.modify_components_global::<Model<VulkanModel>>(
// 			|model|
// 			{
// 				model.aabb.translation += Vec3{ x: 0.0f32, y: -0.001f32, z: 0.0f32 };
// 				Ok(())
// 			}
// 		);
// }

#[system]
pub fn physics_system_2()
{
	// let mut aabb_vector = 
	// 	unsafe {
	// 		decs.get_components_global_mut_unchecked::<Model<VulkanModel>>().unwrap().into_iter().map(|model| &mut model.component.aabb).collect::<Vec<&mut AABB>>()
	// 	};

	let mut aabb_vector: Vec<QueryResultMut<AABB>> = 
		unsafe {decs.get_components_global_mut_unchecked::<AABB>() }.unwrap();

	let raycast_object: &mut RaycastObject =
		unsafe { decs.get_components_global_mut_unchecked::<RaycastObject>() }.unwrap().remove(0).component;

	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	match raycast_object.state
	{
		RaycastObjectState::Thrown((index, length, obj_relative_hit)) =>
		{
			if !aabb_vector[index].component.is_static
			{
				let aabb_pos = aabb_vector[index].component.translation;

				// aabb_vector[index].velocity += 
				// 	(aabb_pos - vk_handle.camera.get_position()).normalize() / 1.0f32;

				aabb_vector[index].component.velocity = 
					(aabb_pos - vk_handle.camera.get_position()).normalize() / 1.0f32;
			}
		}
		RaycastObjectState::Picked((index, length, obj_relative_hit)) =>
		{
			if !aabb_vector[index].component.is_static
			{
				aabb_vector[index].component.translation = vk_handle.camera.get_position() + (vk_handle.camera.get_front() * length) + obj_relative_hit;
				aabb_vector[index].component.velocity = Vec3::new(0.0f32);
			}
		}
		_ => {}
	}

	let delta_time: &mut DeltaTime =
		unsafe { decs.get_components_global_mut_unchecked::<DeltaTime>() }.unwrap().remove(0).component;

	for aabb in aabb_vector.iter_mut()
	{
		aabb.component.color = Vec3{ x: 0.1f32, y: 0.1f32, z: 0.1f32} * aabb.component.mass.exp();
	}

	let gravity = Vec3 { x: 0.0, y: -9.81f32, z: 0.0 };
	for aabb in aabb_vector.iter_mut()
	{
		if aabb.component.is_static
		{
			continue;
		}

		aabb.component.velocity += gravity * (delta_time.last_delta_time / 50000.0f32);
	}

	for aabb in aabb_vector.iter_mut()
	{
		aabb.component.translation += aabb.component.velocity;
	}

	let mut penetration_pairs: Vec<(usize, usize, Vec3)> = vec![];

	for i in 0..aabb_vector.len()
	{
		for j in 0..aabb_vector.len()
		{
			if i == j
			{
				continue;
			}

			let penetration = aabb_vector[i].component.compute_penetration(aabb_vector[j].component);

			if penetration.x < 0.0f32 ||
				penetration.y < 0.0f32 ||
				penetration.z < 0.0f32
			{
				continue;
			}

			penetration_pairs.push((i.min(j), j.max(i), penetration));
		}
	}

	penetration_pairs.sort_by(|a, b| 
		{
			match a.0.cmp(&b.0) 
			{
				std::cmp::Ordering::Equal => a.1.cmp(&b.1),
				other => other,
			}
		}
	);
	penetration_pairs.dedup();
	// println!("penetration pairs : {:?}", penetration_pairs);

	for (i, j, penetration) in penetration_pairs.iter().cloned()
	{
		aabb_vector[i].component.color = Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32};
		aabb_vector[j].component.color = Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32};
	}

	let mut velocity_delta_vec = vec![Vec3::new(0.0f32); aabb_vector.len()];
	let mut translation_delta_vec = vec![Vec3::new(0.0f32); aabb_vector.len()];

	let translation_delta_damp_factor = 0.777; // picking a very Godly number, Terry Davis approved, may he rest in peace.
	let velocity_damp_factor = 0.9f32;

	for (i, j, penetration) in penetration_pairs.iter().cloned()
	{
		if aabb_vector[i].component.is_static && aabb_vector[j].component.is_static
		{
			continue;
		}

		let min_penetration = penetration.x.min(penetration.y).min(penetration.z);

		let axis_of_least_penetration = 
			if min_penetration == penetration.x  { "x" } 
			else if min_penetration == penetration.y { "y" } 
			else { "z" };

		let center_vec = aabb_vector[i].component.translation - aabb_vector[j].component.translation;
		// let relative_velocity = aabb_vector[i].velocity - aabb_vector[j].velocity;

		// Calculating the immediate translation first because its piss easy, A:B mass ratio and we translate by penetration.x * ratio
		// putting .max(0.0f32) here to avoid NaNs if we divide by 0 somewhere
		let mass_i = aabb_vector[i].component.mass.abs();
		let mass_j = aabb_vector[j].component.mass.abs();

		let velocity_i = aabb_vector[i].component.velocity;
		let velocity_j = aabb_vector[j].component.velocity;

		let combined_masses = mass_i + mass_j;

		// let mut translation_ratio_factor_i = ((mass_i / combined_masses)).max(0.0f32);
		// let mut translation_ratio_factor_j = ((mass_j / combined_masses)).max(0.0f32);
		let mut translation_ratio_factor_i = 0.5f32;
		let mut translation_ratio_factor_j = 0.5f32;

		let mut energy_ratio_factor_i = ((mass_j / combined_masses)).max(0.0f32);
		let mut energy_ratio_factor_j = ((mass_i / combined_masses)).max(0.0f32);
		
		if aabb_vector[i].component.is_static
		{
			energy_ratio_factor_j = 2.0f32;

			translation_ratio_factor_i = 1.0f32;
			translation_ratio_factor_j = 0.0f32;
		}
		if aabb_vector[j].component.is_static
		{
			energy_ratio_factor_i = 2.0f32;

			translation_ratio_factor_i = 0.0f32;
			translation_ratio_factor_j = 1.0f32;
		}

		match axis_of_least_penetration
		{
			"x" => 
			{
				// if i is on the bottom, we want to push it "down" so the center vec needs to be negated
				// I wrote it this way in order to avoid confision, all such calculations are aabb[i] - aabb[j]
				let i_sign = center_vec.x.signum();
				let j_sign = -center_vec.x.signum();

				// let relative_velocity_x = relative_velocity.x.abs();

				let combined_momentum = mass_i * velocity_i.x.abs() + mass_j * velocity_j.x.abs();
				// let combined_momentum = mass_i * (relative_velocity_x / 2.0f32) + mass_j * (relative_velocity_x / 2.0f32);

				// panic!("rel vel x for {{ {}:{} }} = {}", i, j, relative_velocity_x);

				// Calculate the force influence factors based on the object masses
				// Two objects of masses A and B will redistribute the combined energy in A:B ratio
				// 	Since im accumulating the velocities so they get applied at the end, after *all* collisions have been done
				// 	I've no idea how im going to implement and handle the complete reversal of velocities

				//	I fucked the delta_time somewhere in the calculations and now im simulating black holes

				// println!("X [{}] [{}] \ni_mf : {:?}\ni_mass : {:?}\nj_mf : {:?}\ni_mass : {:?}", 
				// 	i, j, 
				// 	translation_ratio_factor_i, aabb_vector[i].component.mass, 
				// 	translation_ratio_factor_j, aabb_vector[j].component.mass
				// );
				
				translation_delta_vec[i].x += ((translation_ratio_factor_j * penetration.x) * i_sign) * translation_delta_damp_factor;
				translation_delta_vec[j].x += ((translation_ratio_factor_i * penetration.x) * j_sign) * translation_delta_damp_factor;

				// Calculating the force which will influence the velocity
				velocity_delta_vec[i].x += ((energy_ratio_factor_i * combined_momentum) / mass_i) * i_sign * velocity_damp_factor;
				velocity_delta_vec[j].x += ((energy_ratio_factor_j * combined_momentum) / mass_j) * j_sign * velocity_damp_factor;
			},
			"y" => 
			{
				let i_sign = center_vec.y.signum();
				let j_sign = -center_vec.y.signum();

				// let relative_velocity_y = relative_velocity.y.abs();

				let combined_momentum = mass_i * velocity_i.y.abs() + mass_j * velocity_j.y.abs();
				// let combined_momentum = mass_i * (relative_velocity_y / 2.0f32) + mass_j * (relative_velocity_y / 2.0f32);

				// println!("Y [{}] [{}] \ni_mf : {:?}\ni_mass : {:?}\nj_mf : {:?}\ni_mass : {:?}", 
				// 	i, j, 
				// 	translation_ratio_factor_i, aabb_vector[i].component.mass, 
				// 	translation_ratio_factor_j, aabb_vector[j].component.mass
				// );
				
				translation_delta_vec[i].y += ((translation_ratio_factor_j * penetration.y) * i_sign) * translation_delta_damp_factor;
				translation_delta_vec[j].y += ((translation_ratio_factor_i * penetration.y) * j_sign) * translation_delta_damp_factor;

				velocity_delta_vec[i].y += ((energy_ratio_factor_i * combined_momentum) / mass_i) * i_sign * velocity_damp_factor;
				velocity_delta_vec[j].y += ((energy_ratio_factor_j * combined_momentum) / mass_j) * j_sign * velocity_damp_factor;
			},
			"z" => 
			{
				let i_sign = center_vec.z.signum();
				let j_sign = -center_vec.z.signum();

				// let relative_velocity_z = relative_velocity.z.abs();

				let combined_momentum = mass_i * velocity_i.z.abs() + mass_j * velocity_j.z.abs();
				// let combined_momentum = mass_i * (relative_velocity_z / 2.0f32) + mass_j * (relative_velocity_z / 2.0f32);

				// println!("Z [{}] [{}] \ni_mf : {:?}\ni_mass : {:?}\nj_mf : {:?}\ni_mass : {:?}", 
				// 	i, j, 
				// 	translation_ratio_factor_i, aabb_vector[i].component.mass, 
				// 	translation_ratio_factor_j, aabb_vector[j].component.mass
				// );

				translation_delta_vec[i].z += ((translation_ratio_factor_j * penetration.z) * i_sign) * translation_delta_damp_factor;
				translation_delta_vec[j].z += ((translation_ratio_factor_i * penetration.z) * j_sign) * translation_delta_damp_factor;

				velocity_delta_vec[i].z += ((energy_ratio_factor_i * combined_momentum) / mass_i) * i_sign * velocity_damp_factor;
				velocity_delta_vec[j].z += ((energy_ratio_factor_j * combined_momentum) / mass_j) * j_sign * velocity_damp_factor;
			},

			_ => {}
		}
	}

	for (index, aabb) in aabb_vector.iter_mut().enumerate()
	{
		if aabb.component.is_static
		{
			continue
		}

		aabb.component.velocity += velocity_delta_vec[index];
		// aabb.velocity = velocity_delta_vec[index]; // technically we recalculate the end-velocity above so we don't just accumulate here
	}

	for (index, aabb) in aabb_vector.iter_mut().enumerate()
	{
		if aabb.component.is_static
		{
			continue
		}

		aabb.component.translation += translation_delta_vec[index];
	}

	// for aabb in aabb_vector.iter_mut()
	// {
	// 	aabb.velocity *= aabb.damping;
	// }

	for (index, aabb) in aabb_vector.iter_mut().enumerate()
	{
		// println!("[{}] static:{} mass:{:?}\nvel:{:?} = {}", index, aabb.component.is_static, aabb.component.mass, aabb.component.velocity, aabb.component.velocity.len());
	}
}


// pub fn run_physics(aabb_vector: &mut Vec<&mut AABB>, delta_time_ms: f64)
// {
// 	for aabb in aabb_vector.iter_mut()
// 	{
// 		aabb.color = Vec3{ x: 0.1f32, y: 0.1f32, z: 0.1f32} * aabb.mass.exp();
// 	}

// 	let gravity = Vec3 { x: 0.0, y: -9.81f32, z: 0.0 };
// 	for aabb in aabb_vector.iter_mut()
// 	{
// 		if aabb.is_static
// 		{
// 			continue;
// 		}

// 		aabb.velocity += gravity * (delta_time_ms as f32 / 50000.0f32);
// 	}

// 	for aabb in aabb_vector.iter_mut()
// 	{
// 		aabb.translation += aabb.velocity;
// 	}

// 	let mut penetration_pairs: Vec<(usize, usize, Vec3)> = vec![];

// 	for i in 0..aabb_vector.len()
// 	{
// 		for j in 0..aabb_vector.len()
// 		{
// 			if i == j
// 			{
// 				continue;
// 			}

// 			let penetration = aabb_vector[i].compute_penetration(aabb_vector[j]);

// 			if penetration.x < 0.0f32 ||
// 				penetration.y < 0.0f32 ||
// 				penetration.z < 0.0f32
// 			{
// 				continue;
// 			}

// 			penetration_pairs.push((i.min(j), j.max(i), penetration));
// 		}
// 	}

// 	penetration_pairs.sort_by(|a, b| 
// 		{
// 			match a.0.cmp(&b.0) 
// 			{
// 				std::cmp::Ordering::Equal => a.1.cmp(&b.1),
// 				other => other,
// 			}
// 		}
// 	);
// 	penetration_pairs.dedup();
// 	// println!("penetration pairs : {:?}", penetration_pairs);

// 	for (i, j, penetration) in penetration_pairs.iter().cloned()
// 	{
// 		aabb_vector[i].color = Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32};
// 		aabb_vector[j].color = Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32};
// 	}

// 	let mut velocity_delta_vec = vec![Vec3::new(0.0f32); aabb_vector.len()];
// 	let mut translation_delta_vec = vec![Vec3::new(0.0f32); aabb_vector.len()];

// 	let translation_delta_damp_factor = 0.777; // picking a very Godly number, Terry Davis approved, may he rest in peace.
// 	let velocity_damp_factor = 0.9f32;

// 	for (i, j, penetration) in penetration_pairs.iter().cloned()
// 	{
// 		if aabb_vector[i].is_static && aabb_vector[j].is_static
// 		{
// 			continue;
// 		}

// 		let min_penetration = penetration.x.min(penetration.y).min(penetration.z);

// 		let axis_of_least_penetration = 
// 			if min_penetration == penetration.x  { "x" } 
// 			else if min_penetration == penetration.y { "y" } 
// 			else { "z" };

// 		let center_vec = aabb_vector[i].translation - aabb_vector[j].translation;
// 		let relative_velocity = aabb_vector[i].velocity - aabb_vector[j].velocity;


// 		// Calculating the immediate translation first because its piss easy, A:B mass ratio and we translate by penetration.x * ratio
// 		// putting .max(0.0f32) here to avoid NaNs if we divide by 0 somewhere
// 		let mass_i = aabb_vector[i].mass.abs();
// 		let mass_j = aabb_vector[j].mass.abs();

// 		let velocity_i = aabb_vector[i].velocity;
// 		let velocity_j = aabb_vector[j].velocity;

// 		let combined_masses = mass_i + mass_j;

// 		// let mut translation_ratio_factor_i = ((mass_i / combined_masses)).max(0.0f32);
// 		// let mut translation_ratio_factor_j = ((mass_j / combined_masses)).max(0.0f32);
// 		let mut translation_ratio_factor_i = 0.5f32;
// 		let mut translation_ratio_factor_j = 0.5f32;

// 		let mut energy_ratio_factor_i = ((mass_j / combined_masses)).max(0.0f32);
// 		let mut energy_ratio_factor_j = ((mass_i / combined_masses)).max(0.0f32);
		
// 		if aabb_vector[i].is_static
// 		{
// 			energy_ratio_factor_j = 2.0f32;

// 			translation_ratio_factor_i = 1.0f32;
// 			translation_ratio_factor_j = 0.0f32;
// 		}
// 		if aabb_vector[j].is_static
// 		{
// 			energy_ratio_factor_i = 2.0f32;

// 			translation_ratio_factor_i = 0.0f32;
// 			translation_ratio_factor_j = 1.0f32;
// 		}

// 		match axis_of_least_penetration
// 		{
// 			"x" => 
// 			{
// 				// if i is on the bottom, we want to push it "down" so the center vec needs to be negated
// 				// I wrote it this way in order to avoid confision, all such calculations are aabb[i] - aabb[j]
// 				let i_sign = center_vec.x.signum();
// 				let j_sign = -center_vec.x.signum();
// 				let relative_velocity_x = relative_velocity.x.abs();

// 				let combined_momentum = mass_i * velocity_i.x.abs() + mass_j * velocity_j.x.abs();
// 				// let combined_momentum = mass_i * (relative_velocity_x / 2.0f32) + mass_j * (relative_velocity_x / 2.0f32);

// 				// panic!("rel vel x for {{ {}:{} }} = {}", i, j, relative_velocity_x);

// 				// Calculate the force influence factors based on the object masses
// 				// Two objects of masses A and B will redistribute the combined energy in A:B ratio
// 				// 	Since im accumulating the velocities so they get applied at the end, after *all* collisions have been done
// 				// 	I've no idea how im going to implement and handle the complete reversal of velocities

// 				//	I fucked the delta_time somewhere in the calculations and now im simulating black holes

// 				println!("X [{}] [{}] \ni_mf : {:?}\ni_mass : {:?}\nj_mf : {:?}\ni_mass : {:?}", 
// 					i, j, 
// 					translation_ratio_factor_i, aabb_vector[i].mass, 
// 					translation_ratio_factor_j, aabb_vector[j].mass
// 				);
				
// 				translation_delta_vec[i].x += ((translation_ratio_factor_j * penetration.x) * i_sign) * translation_delta_damp_factor;
// 				translation_delta_vec[j].x += ((translation_ratio_factor_i * penetration.x) * j_sign) * translation_delta_damp_factor;

// 				// Calculating the force which will influence the velocity
// 				velocity_delta_vec[i].x += ((energy_ratio_factor_i * combined_momentum) / mass_i) * i_sign * velocity_damp_factor;
// 				velocity_delta_vec[j].x += ((energy_ratio_factor_j * combined_momentum) / mass_j) * j_sign * velocity_damp_factor;
// 			},
// 			"y" => 
// 			{
// 				let i_sign = center_vec.y.signum();
// 				let j_sign = -center_vec.y.signum();

// 				let relative_velocity_y = relative_velocity.y.abs();

// 				let combined_momentum = mass_i * velocity_i.y.abs() + mass_j * velocity_j.y.abs();
// 				// let combined_momentum = mass_i * (relative_velocity_y / 2.0f32) + mass_j * (relative_velocity_y / 2.0f32);

// 				println!("Y [{}] [{}] \ni_mf : {:?}\ni_mass : {:?}\nj_mf : {:?}\ni_mass : {:?}", 
// 					i, j, 
// 					translation_ratio_factor_i, aabb_vector[i].mass, 
// 					translation_ratio_factor_j, aabb_vector[j].mass
// 				);
				
// 				translation_delta_vec[i].y += ((translation_ratio_factor_j * penetration.y) * i_sign) * translation_delta_damp_factor;
// 				translation_delta_vec[j].y += ((translation_ratio_factor_i * penetration.y) * j_sign) * translation_delta_damp_factor;

// 				velocity_delta_vec[i].y += ((energy_ratio_factor_i * combined_momentum) / mass_i) * i_sign * velocity_damp_factor;
// 				velocity_delta_vec[j].y += ((energy_ratio_factor_j * combined_momentum) / mass_j) * j_sign * velocity_damp_factor;
// 			},
// 			"z" => 
// 			{
// 				let i_sign = center_vec.z.signum();
// 				let j_sign = -center_vec.z.signum();

// 				let relative_velocity_z = relative_velocity.z.abs();

// 				let combined_momentum = mass_i * velocity_i.z.abs() + mass_j * velocity_j.z.abs();
// 				// let combined_momentum = mass_i * (relative_velocity_z / 2.0f32) + mass_j * (relative_velocity_z / 2.0f32);

// 				println!("Z [{}] [{}] \ni_mf : {:?}\ni_mass : {:?}\nj_mf : {:?}\ni_mass : {:?}", 
// 					i, j, 
// 					translation_ratio_factor_i, aabb_vector[i].mass, 
// 					translation_ratio_factor_j, aabb_vector[j].mass
// 				);

// 				translation_delta_vec[i].z += ((translation_ratio_factor_j * penetration.z) * i_sign) * translation_delta_damp_factor;
// 				translation_delta_vec[j].z += ((translation_ratio_factor_i * penetration.z) * j_sign) * translation_delta_damp_factor;

// 				velocity_delta_vec[i].z += ((energy_ratio_factor_i * combined_momentum) / mass_i) * i_sign * velocity_damp_factor;
// 				velocity_delta_vec[j].z += ((energy_ratio_factor_j * combined_momentum) / mass_j) * j_sign * velocity_damp_factor;
// 			},

// 			_ => {}
// 		}
// 	}

// 	for (index, aabb) in aabb_vector.iter_mut().enumerate()
// 	{
// 		if aabb.is_static
// 		{
// 			continue
// 		}

// 		aabb.velocity += velocity_delta_vec[index];
// 		// aabb.velocity = velocity_delta_vec[index]; // technically we recalculate the end-velocity above so we don't just accumulate here
// 	}

// 	for (index, aabb) in aabb_vector.iter_mut().enumerate()
// 	{
// 		if aabb.is_static
// 		{
// 			continue
// 		}

// 		aabb.translation += translation_delta_vec[index];
// 	}

// 	// for aabb in aabb_vector.iter_mut()
// 	// {
// 	// 	aabb.velocity *= aabb.damping;
// 	// }

// 	for (index, aabb) in aabb_vector.iter_mut().enumerate()
// 	{
// 		println!("[{}] static:{} mass:{:?}\nvel:{:?} = {}", index, aabb.is_static, aabb.mass, aabb.velocity, aabb.velocity.len());
// 	}
// }
