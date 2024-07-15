use decs::component_derive::component;
use decs::component::Component;

use std::fmt::Debug;

use crate::cotangens::vec3::Vec3;

#[component]
pub struct PhysicsImpulseComponent3D
{
	pub vector: Vec3
}