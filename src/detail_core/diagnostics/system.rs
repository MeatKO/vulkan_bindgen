use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::components::misc::DeltaTime;

#[system]
pub fn print_delta_time_system()
{
	let main_loop_var = 
		match decs.get_components_global::<DeltaTime>()
			{
				Ok(delta_time_vec) =>  { delta_time_vec.into_iter().next().unwrap() }
				Err(_) => { return }
			};

	println!("Last delta time : {}", main_loop_var.last_delta_time)
}