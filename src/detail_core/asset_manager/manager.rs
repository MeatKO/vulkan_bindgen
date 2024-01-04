use std::{collections::{HashMap, hash_map::Entry}, any::{TypeId, Any}, rc::{Rc, Weak}};

use decs::component_derive::component;
use decs::component::Component;

#[component]
pub struct AssetManager
{
	assets: HashMap<TypeId, Vec<(String, Rc<dyn Any>)>>,
}

impl AssetManager
{
	pub fn new() -> Self
	{
		Self
		{
			assets: HashMap::new(),
		}
	}

	pub fn add_asset<T: Any>(&mut self, asset_name: impl ToString, asset: T) -> Result<(), String>
	{
		let asset_type_id = TypeId::of::<T>();
		let asset_type_name = std::any::type_name::<T>();

		let asset_name = asset_name.to_string();
	
		match self.assets
			.entry(asset_type_id)
		{
			Entry::Occupied(mut occupied) => 
			{
				for (current_asset_name, _) in occupied.get()
				{
					if *current_asset_name == asset_name
					{
						return Err(format!("asset called '{}' of type '{}' already exists.", asset_name, asset_type_name).to_owned())
					}
				}

				occupied.get_mut().push((asset_name, Rc::new(asset)));
			},
			Entry::Vacant(vacant) => 
			{
				vacant.insert(Vec::new()).push((asset_name, Rc::new(asset)));
			},
		}

		Ok(())
	}

	pub fn get_asset<T: Any>(&self, asset_name: impl ToString) -> Result<Weak<T>, String>
	{
		let asset_type_id = TypeId::of::<T>();
		let asset_type_name = std::any::type_name::<T>();

		let asset_name = asset_name.to_string();
	
		match self.assets.get(&asset_type_id)
		{
			Some(assets) => 
			{
				for (current_asset_name, asset) in assets
				{
					if *current_asset_name == asset_name
					{
						let cast: Rc<T> = asset.clone().downcast().unwrap();
						let weak: Weak<T> = Rc::downgrade(&cast);
						return Ok(weak)
					}
				}
			},
			_ => {}
		}

		Err(format!("asset called '{}' of type '{}' doesn't exist.", asset_name, asset_type_name).to_owned())
	}

	pub fn get_asset_rc<T: Any>(&self, asset_name: impl ToString) -> Result<Rc<T>, String>
	{
		let asset_type_id = TypeId::of::<T>();
		let asset_type_name = std::any::type_name::<T>();

		let asset_name = asset_name.to_string();
	
		match self.assets.get(&asset_type_id)
		{
			Some(assets) => 
			{
				for (current_asset_name, asset) in assets
				{
					if *current_asset_name == asset_name
					{
						let cast: Rc<T> = asset.clone().downcast().unwrap();
						return Ok(cast)
					}
				}
			},
			_ => {}
		}

		Err(format!("asset called '{}' of type '{}' doesn't exist.", asset_name, asset_type_name).to_owned())
	}
}