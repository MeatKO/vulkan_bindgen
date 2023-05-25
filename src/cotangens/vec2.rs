use std::cmp::Eq;
use std::hash::{
	Hash,
	Hasher,
};

#[repr(align(8))]
#[derive(Clone, Debug, PartialEq)]
pub struct Vec2
{
	pub x: f32,
	pub y: f32,
}

impl Hash for Vec2
{
	fn hash<H: Hasher>(&self, state: &mut H) 
	{
        (self.x as u32).hash(state);
        (self.y as u32).hash(state);
    }
}

impl Eq for Vec2 {}