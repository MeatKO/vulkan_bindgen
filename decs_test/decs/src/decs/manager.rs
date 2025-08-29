#![allow(non_camel_case_types)]

use std::{any::{Any, TypeId}, collections::HashMap};

use super::component::Component;

// not quite sure what 'static means for that (), do we even need to specify that ? 
// is there a () that isnt static ??
type System = dyn Fn(&mut dECS) -> () + Send + Sync + 'static;


// The QueryParameter design might've been a mistake
// is it more advantageous to put the typeid as an enum value 
// or is it better to just have a raw enum and then a struct
// that will have the typeid and queryparameter member at equal level ?
#[derive(PartialEq, Eq)]
pub enum QueryParameter
{
	Concrete(TypeId),
	Optional(TypeId),
}

impl QueryParameter
{
	pub fn inner_typeid(&self) -> TypeId
	{
		match self
		{
			QueryParameter::Concrete(type_id) => { type_id.clone() },
			QueryParameter::Optional(type_id) => { type_id.clone() }
		}
	}
}

// These should hold only the dense and not sparse set, because 
// that way we avoid unnecessary looping and searching for valid indices
// we also avoid having to copy the dense set, whereas having a sparse set
// to loop through will require us to make a new vector for the valid subset of entity ids
pub struct Archetype
{
	types_vec: Vec<QueryParameter>,
	dense_entity_vec: Vec<u64>, // should this be an i64 ? idk lets see
}

impl Archetype
{
	pub fn is_equal(&self, mut in_requested_params: Vec<QueryParameter>) -> bool
	{
		in_requested_params.sort_by(
			|first, second|
			{
				let first_typeid = first.inner_typeid();
				let second_typeid = second.inner_typeid();
				
				return first_typeid.cmp(&second_typeid)
			}
		);

		return self.types_vec == in_requested_params;
	}

	fn _sort_types_vec(&mut self)
	{
		self.types_vec.sort_by(
			|first, second|
			{
				let first_typeid = first.inner_typeid();
				let second_typeid = second.inner_typeid();
				
				return first_typeid.cmp(&second_typeid)
			}
		);
	}
}

// The manager holds Vec<Box<ComponentStorage>>, but our actual Storages are of type TypedComponentStorage
// and require a concrete Component type <C>.
trait ComponentStorage: Any
{
	fn add_entity(&mut self);
	fn get_sparse_count(&mut self) -> usize;
	fn get_dense_count(&mut self) -> usize;
	fn get_component_count(&mut self) -> usize;
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn concrete_inner_typeid(&self) -> TypeId;
}

#[derive(Debug)]
pub enum ComponentQueryError
{
	SparseIdExceedsMax(i64), // contains the sparse ID
	DenseIdExceedsMax(i64, i64), // contains the (sparse, dense) ID pairs
	EntityDoesntHaveComponent(i64), // contains the sparse ID
}

pub struct dECS
{
	entity_id_counter: u64,
	components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

// ----------------------------------

impl dECS
{
	pub fn new() -> Self
	{
		dECS { 
			entity_id_counter: 0,
			components: HashMap::new(),
		}
	}

	pub fn print_component_storage_typeids(&self)
	{
		// component_storage is (&TypeId, &Box<dyn ComponentStorage>)
		for component_storage in self.components.iter()
		{
			println!("Component Storage Type Key Value : '{:?}'", component_storage.0);
			println!("Component Storage Type : '{:?}'", component_storage.1.type_id());
			println!("Component Storage Type Deref box : '{:?}'", (*component_storage.1).type_id());
			println!("Component Storage Inner Type Deref box : '{:?}'", (*component_storage.1).concrete_inner_typeid());
			println!();
		}
	}

	pub fn add_entity(&mut self) -> u64
	{
		let out_entity_id = self.entity_id_counter;
		self.entity_id_counter += 1;

		for component_storage in self.components.iter_mut()
		{
			component_storage.1.add_entity();
		}

		return out_entity_id;
	}

	// we use u64 here instead of i64 because
	// the public interface should never enter an invalid entity_id
	// and the best I can constrain it here 
	// is to at least not be smaller than 0
	pub fn add_component<C>(&mut self, entity_id: u64, component: C)
	where 
		C: Component
	{
		let type_id = TypeId::of::<C>();
		let entity_id = entity_id as i64;

		if let Some(component_storage) = self.components.get_mut(&type_id)
		{
			println!("Adding component to Component Storage object for type '{}'", std::any::type_name::<C>());

			let concrete_storage_object: &mut TypedComponentStorage<C> = 
				component_storage.as_any_mut().downcast_mut().unwrap();

			concrete_storage_object.add_component(component, entity_id).unwrap();
		}
		else
		{
			println!("Making new Component Storage object for type '{}'", std::any::type_name::<C>());

			let mut new_component_storage = 
				TypedComponentStorage::<C>::new_sized(self.entity_id_counter as _);

			new_component_storage.add_component(component, entity_id).expect("Couldn't add component");

			self.components.insert(type_id, Box::new(new_component_storage));
		}
	}
}

// ----------------------------------

// The dense and sparse set use signed instead of unsigned because
// deletion is done via setting the index to -1 instead of performing
// some expensive memory operation or reallocation
struct TypedComponentStorage<C: Component>
{
	components: Vec<C>,
	dense_set: Vec<i64>, // stores sparse set IDs as a means of reverse search
	sparse_set: Vec<i64> // stores dense set IDs that also map to the components directly, -1 if empty
}

impl<C: Component> ComponentStorage for TypedComponentStorage<C>
{
	fn add_entity(&mut self) 
	{
		self.sparse_set.push(-1);
	}

	fn get_sparse_count(&mut self) -> usize
	{
		return self.sparse_set.len()
	}

	fn get_dense_count(&mut self) -> usize
	{
		return self.dense_set.len()
	}

	fn get_component_count(&mut self) -> usize
	{
		return self.components.len()
	}

	fn as_any(&self) -> &dyn Any 
	{
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any 
	{
		self
	}
	
	fn concrete_inner_typeid(&self) -> TypeId 
	{
		return TypeId::of::<C>()
	}
}

impl<C: Component> TypedComponentStorage<C>
{
	// pub fn new_empty() -> Self
	// {
	// 	return Self{
	// 		components: vec![],
	// 		dense_set: vec![],
	// 		sparse_set: vec![],
	// 	};
	// }

	pub fn new_sized(entity_count: usize) -> Self
	{
		return Self{
			components: vec![],
			dense_set: vec![],
			sparse_set: vec![-1i64; entity_count],
		};
	}

	// Called when we run the add_entity() through the manager class
	// pub fn reserve_entity(&mut self)
	// {
	// 	self.sparse_set.reserve(1);
	// }

	// we will have no method add_entity
	// because it will be part of the upper dECS class
	// and it will just resize our sparse set once it increments its entity counter
	pub fn add_component(&mut self, component: C, sparse_entity_id: i64) 
	-> Result<(), ()>
	{
		self.components.push(component);
		self.dense_set.push(sparse_entity_id);
		self.sparse_set[sparse_entity_id as usize] = (self.dense_set.len() - 1) as i64;

		return Ok(());
	}

	pub fn get_component(&self, sparse_entity_id: i64) 
	-> Result<&C, ComponentQueryError>
	{
		let dense_entity_id = self.sparse_set[sparse_entity_id as usize];

		if dense_entity_id <= -1
		{
			return Err(ComponentQueryError::EntityDoesntHaveComponent(sparse_entity_id))
		}

		return Ok(&self.components[dense_entity_id as usize])
	}
}

