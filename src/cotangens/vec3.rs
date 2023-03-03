#[repr(align(16))]
pub struct Vec3
{
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3
{
	pub fn len(&self) -> f32
	{
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}

	pub fn normalize(&self) -> Self
	{
		let self_len = self.len();

		return Vec3 { 
			x: self.x / self_len, 
			y: self.y / self_len, 
			z: self.z / self_len 
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