use std::cmp::Eq;
use std::hash::{
	Hash,
	Hasher,
};
use std::ops::{Sub, Add};

#[repr(align(8))]
#[derive(Clone, Debug, PartialEq)]
pub struct Vec2
{
	pub x: f32,
	pub y: f32,
}

impl Vec2
{
	pub fn new(value: f32) -> Vec2
	{
		return Vec2 { 
			x: value, 
			y: value, 
		}
	}
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

impl Add<&Vec2> for &Vec2 
{
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Vec2
	{
		return Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Sub<&Vec2> for &Vec2 
{
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Vec2
	{
		return Vec2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}