#![feature(get_many_mut)]

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::any::{self, Any, TypeId};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use decs::component_derive::{system, component, component2};
use decs::component::{Component, AsAny};
use decs::manager::{dECS, QueryParameter::*};
use proc_macro2::{TokenStream, TokenTree};

#[component]
pub struct TestComponent0
{
	pub velocity: f32
}

#[component]
pub struct TestComponent1
{
	pub velocity: f32
}

#[component2]
pub struct TestComponent2
{
	pub name: String
}

#[component]
pub struct TestComponent3
{
	pub velocity: f32
}

#[system]
pub fn test_system_0()
{
	decs.add_component(0, TestComponent1{velocity: 10f32});
}

fn main() 
{
	// let mut decs_manager = dECS::new();

	// let new_entity = decs_manager.add_entity();

	// println!("Added new entity {}", new_entity);

	// decs_manager.add_component(new_entity, PhysBox{velocity: 1337.0f32});
	// decs_manager.add_component(new_entity, NameTag{name: "huyo".to_owned()});

	// println!("PhysBox type_name : {}", std::any::type_name::<PhysBox>());
	// println!("PhysBox type_id : {:?}", PhysBox{velocity: 1337.0f32}.type_id());
	// println!("NameTag type_name : {}", std::any::type_name::<NameTag>());
	// println!("NameTag type_id : {:?}", NameTag{name: "huyo".to_owned()}.type_id());
	// println!();
	// decs_manager.print_component_storage_typeids();

	let mut test_type_vec: Vec<decs::manager::QueryParameter> = vec![];

	test_type_vec.push(decs::manager::QueryParameter::Concrete(TypeId::of::<TestComponent2>()));
	test_type_vec.push(decs::manager::QueryParameter::Concrete(TypeId::of::<TestComponent1>()));
	test_type_vec.push(decs::manager::QueryParameter::Concrete(TypeId::of::<TestComponent0>()));
	test_type_vec.push(decs::manager::QueryParameter::Concrete(TypeId::of::<TestComponent0>()));
	test_type_vec.push(decs::manager::QueryParameter::Concrete(TypeId::of::<TestComponent3>()));
	test_type_vec.push(decs::manager::QueryParameter::Concrete(TypeId::of::<TestComponent3>()));

	println!("unsorted");
	for typeid in test_type_vec.iter()
	{
		println!("{:?}", typeid.inner_typeid());
	}

	test_type_vec.sort_by(
		|first, second|
		{
			let first_typeid = first.inner_typeid();
			let second_typeid = second.inner_typeid();
			
			return first_typeid.cmp(&second_typeid)
		}
	);

	println!("sorted");
	for typeid in test_type_vec.iter()
	{
		println!("{:?}", typeid.inner_typeid());
	}
}
