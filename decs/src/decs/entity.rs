use std::{collections::HashMap, any::TypeId};

use super::component::Component;

pub struct Entity
{
	pub id: usize,
	pub components: HashMap<TypeId, Vec<Box<dyn Component>>>,
}

impl Entity
{
	pub fn new(id: usize) -> Self
	{
		Entity { 
			id: id, 
			components: HashMap::new()
		}
	}
}