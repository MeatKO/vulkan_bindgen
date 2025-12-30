use decs2::component_derive::Component;
use decs2::typedef::Component;

use crate::cotangens::vec3::Vec3;

#[derive(Component)]
pub struct PhysicsImpulseComponent3D
{
	pub vector: Vec3
}