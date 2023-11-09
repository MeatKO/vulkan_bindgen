use crate::cotangens::vec3::Vec3;

use super::aabb::AABB;

pub fn run_physics(aabb_vector: &mut Vec<&mut AABB>, delta_time_ms: f64)
{
	for aabb in aabb_vector.iter_mut()
	{
		aabb.color = Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32};
	}
	
	// let mut new_velocities = 
	// 	vec![
	// 		Vec3::new(0.0f32); 
	// 		aabb_vector.len()
	// 	];

	let mut new_velocities = aabb_vector.iter().map(|aabb| aabb.velocity).collect::<Vec<Vec3>>();

	for i in 0..aabb_vector.len()
	{
		for j in i+1..aabb_vector.len()
		{
			let penetration = aabb_vector[i].compute_penetration(aabb_vector[j]);
			// println!("Pen factors : {:?}", penetration);

			let axis_of_least_penetration = 
				if penetration.x.min(penetration.y).min(penetration.z) == penetration.x 
				{ 
					"x" 
				} 
				else if penetration.x.min(penetration.y).min(penetration.z) == penetration.y 
				{ 
					"y" 
				} 
				else 
				{ 
					"z" 
				};

            // Reflect velocity vector based on the axis of collision

			if penetration.x > 0.0f32 &&
				penetration.y > 0.0f32 &&
				penetration.z > 0.0f32
			{
				match axis_of_least_penetration 
				{
					"x" => 
					{ 
						println!("reflect X");
						new_velocities[i].x = (-new_velocities[i].x * aabb_vector[i].restitution) + penetration.x;
						// new_velocities[i].x = -new_velocities[i].x * aabb_vector[i].restitution;
						// new_velocities[j].x = -new_velocities[j].x * aabb_vector[j].restitution;
					},
					"y" => 
					{ 
						println!("reflect Y");
						new_velocities[i].y = (-new_velocities[i].y * aabb_vector[i].restitution) + penetration.y;
						// new_velocities[i].y = -new_velocities[i].y * aabb_vector[i].restitution;
						// new_velocities[j].y = -new_velocities[j].y * aabb_vector[j].restitution;
					},
					"z" => 
					{ 
						println!("reflect Z");
						new_velocities[i].z = (-new_velocities[i].z * aabb_vector[i].restitution) + penetration.z;
						// new_velocities[i].z = -new_velocities[i].z * aabb_vector[i].restitution;
						// new_velocities[j].z = -new_velocities[j].z * aabb_vector[j].restitution;
					},
					_ => {},
				}

				aabb_vector[i].color = Vec3{ x: 1.0f32, y: 0.0f32, z: 0.0f32};
				aabb_vector[j].color = Vec3{ x: 1.0f32, y: 0.0f32, z: 0.0f32};
			}
		}
	}

	// for aabb in aabb_vector.iter_mut()
	// {
	// 	aabb.apply_gravity(delta_time_ms as f32);
	// }

	for (index, aabb) in aabb_vector.iter_mut().enumerate()
	{
		aabb.velocity = new_velocities[index];
	}

	// for aabb in aabb_vector.iter_mut()
	// {
	// 	aabb.apply_gravity(delta_time_ms as f32);
	// }

	for aabb in aabb_vector.iter_mut()
	{
		aabb.translation += aabb.velocity;
	}

	for aabb in aabb_vector.iter_mut()
	{
		aabb.apply_gravity(delta_time_ms as f32);
	}
}