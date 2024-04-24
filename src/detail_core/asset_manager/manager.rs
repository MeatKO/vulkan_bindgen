use std::{collections::{HashMap, hash_map::Entry}, any::{TypeId, Any}, rc::{Rc, Weak}, time::Instant};

use decs::component_derive::component;
use decs::component::Component;

struct Asset
{
	created_at: Instant,
	last_used_at: Instant,
	is_currently_loaded: bool,
	byte_size: usize,
	data: Rc<dyn Any>,
}

#[component]
pub struct AssetManager
{
	assets: HashMap<(TypeId, String), Rc<dyn Any>>,
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
			.entry((asset_type_id, asset_name.clone()))
		{
			Entry::Occupied(_) => 
			{
				return Err(format!("asset called '{}' of type '{}' already exists.", asset_name, asset_type_name).to_owned())
			},
			Entry::Vacant(vacant) => 
			{
				vacant.insert(Rc::new(asset));
			},
		}

		Ok(())
	}

	pub fn get_asset<T: Any>(&self, asset_name: impl ToString) -> Result<Weak<T>, String>
	{
		let asset = self.get_asset_rc::<T>(asset_name)?;

		Ok(Rc::downgrade(&asset))
	}

	pub fn get_asset_rc<T: Any>(&self, asset_name: impl ToString) -> Result<Rc<T>, String>
	{
		let asset_type_id = TypeId::of::<T>();
		let asset_type_name = std::any::type_name::<T>();

		let asset_name = asset_name.to_string();
	
		match self.assets.get(&(asset_type_id, asset_name.clone()))
		{
			Some(asset) => 
			{
				let cast: Rc<T> = asset.clone().downcast().unwrap(); // if this ever fails im Mickey Mouse
				Ok(cast)
			},
			_ => 
			{
				Err(format!("asset called '{}' of type '{}' doesn't exist.", asset_name, asset_type_name).to_owned())
			}
		}
	}
}