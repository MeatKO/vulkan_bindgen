use std::ops::{
	Mul, AddAssign, SubAssign, Add, Sub, MulAssign, Div,
};
use std::cmp::{Eq, Ordering};
use std::hash::{
	Hash,
	Hasher,
};

#[repr(align(16))]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3
{
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3
{
	pub fn lerp(start: Vec3, end: Vec3, t: f32) -> Vec3
    {
        start + (end - start) * t
    }

	pub fn new(value: f32) -> Vec3
	{
		return Vec3 { 
			x: value, 
			y: value, 
			z: value 
		}
	}

	pub fn len(&self) -> f32
	{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn distance(&self, other: &Vec3) -> f32
	{
		(*self - *other).len()
	}

	pub fn normalize(&self) -> Self
	{
		let mut self_len = self.len();

		if self_len == 0.0f32
		{
			self_len = 1.0f32;
		}

		return Vec3 { 
			x: self.x / self_len, 
			y: self.y / self_len, 
			z: self.z / self_len,
		}
	}

	// does not automatically normalize !
	pub fn dot(&self, rhs: &Self) -> f32
	{
		return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn cross(&self, rhs: &Self) -> Self
	{
		return Vec3 { 
			x: (self.y * rhs.z - self.z * rhs.y), 
			y: (self.z * rhs.x - self.x * rhs.z), 
			z: (self.x * rhs.y - self.y * rhs.x), 
		}
	}

	pub fn negate(&self) -> Self
	{
		return Vec3 { 
			x: -self.x, 
			y: -self.y, 
			z: -self.z, 
		}
	}
}

impl Hash for Vec3
{
	fn hash<H: Hasher>(&self, state: &mut H) 
	{
        (self.x as u32).hash(state);
        (self.y as u32).hash(state);
        (self.z as u32).hash(state);
	}
}

impl PartialOrd for Vec3 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
	{
        self.len().partial_cmp(&other.len())
    }
}

impl Ord for Vec3 
{
    fn cmp(&self, other: &Self) -> Ordering 
	{
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Vec3 {}

impl Add<Vec3> for Vec3 
{
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 
	{
		return Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

impl Sub<Vec3> for Vec3 
{
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 
	{
		return Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}

impl<T> Mul<T> for Vec3
where
    T: Into<f32>
{
    type Output = Self;

    fn mul(self, factor: T) -> Self
    {
        let factor = factor.into();
        Vec3 
        {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl<T> Div<T> for Vec3
where
    T: Into<f32>
{
    type Output = Self;

    fn div(self, factor: T) -> Self
    {
        let factor = factor.into();
        Vec3 
        {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

impl<T> MulAssign<T> for Vec3
where
    T: Into<f32>
{
    fn mul_assign(&mut self, factor: T) 
    {
		let factor = factor.into();

        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl AddAssign<Vec3> for Vec3 
{
    fn add_assign(&mut self, rhs: Vec3) 
	{
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl SubAssign<Vec3> for Vec3 
{
    fn sub_assign(&mut self, rhs: Vec3)
	{
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}