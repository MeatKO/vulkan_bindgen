use crate::{cotangens::vec3::Vec3, vulkan::vertex::Vertex};

#[derive(Debug)]
pub struct AABB
{
	pub position: Vec3,
	pub size: Vec3,
	pub color: Vec3,
	pub index_count: u32,
}

impl AABB
{
	pub fn new() -> AABB
	{
		AABB {
			position: Vec3::new(1.0f32),
			size: Vec3::new(1.0f32),
			color: Vec3::new(1.0f32),
			index_count: 36,
		}
	}

	pub fn check_collision(&self, other: &AABB) -> bool 
	{
		let self_min = Vec3
        {
            x: self.position.x - self.size.x,
            y: self.position.y - self.size.y,
            z: self.position.z - self.size.z,
        };

        let self_max = Vec3
        {
            x: self.position.x + self.size.x,
            y: self.position.y + self.size.y,
            z: self.position.z + self.size.z,
        };

        let other_min = Vec3
        {
            x: other.position.x - other.size.x,
            y: other.position.y - other.size.y,
            z: other.position.z - other.size.z,
        };

        let other_max = Vec3
        {
            x: other.position.x + other.size.x,
            y: other.position.y + other.size.y,
            z: other.position.z + other.size.z,
        };

        // Check for overlap along the x-axis
        if self_max.x < other_min.x || self_min.x > other_max.x 
		{
            return false;
        }

        // Check for overlap along the y-axis
        if self_max.y < other_min.y || self_min.y > other_max.y 
		{
            return false;
        }

        // Check for overlap along the z-axis
        if self_max.z < other_min.z || self_min.z > other_max.z 
		{
            return false;
        }

        // If we have overlap along all three axes, the AABBs are colliding
        true
    }

	pub fn get_geometry(&self) -> (Vec<Vertex>, Vec<u32>)
	{
		let out_vertices = 
			vec![
				Vertex::from_position( Vec3 { x: self.size.x, y: self.size.y, z: self.size.z } ),
				Vertex::from_position( Vec3 { x: -self.size.x, y: self.size.y, z: self.size.z } ),
				Vertex::from_position( Vec3 { x: -self.size.x, y: -self.size.y, z: self.size.z } ),
				Vertex::from_position( Vec3 { x: self.size.x, y: -self.size.y, z: self.size.z } ),
				Vertex::from_position( Vec3 { x: self.size.x, y: self.size.y, z: -self.size.z } ),
				Vertex::from_position( Vec3 { x: -self.size.x, y: self.size.y, z: -self.size.z } ),
				Vertex::from_position( Vec3 { x: -self.size.x, y: -self.size.y, z: -self.size.z } ),
				Vertex::from_position( Vec3 { x: self.size.x, y: -self.size.y, z: -self.size.z } ),
			];

		let out_indices = 
			vec![
				// Front face
				0, 1, 2, 0, 2, 3,
				// Back face
				4, 6, 5, 4, 7, 6,
				// Top face
				4, 5, 1, 4, 1, 0,
				// Bottom face
				3, 2, 6, 3, 6, 7,
				// Right face
				0, 3, 7, 0, 7, 4,
				// Left face
				5, 6, 2, 5, 2, 1,
			];

		(out_vertices, out_indices)
	}
}