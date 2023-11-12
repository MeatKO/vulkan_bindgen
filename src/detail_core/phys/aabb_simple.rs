use crate::{cotangens::vec3::Vec3, vulkan::vertex::Vertex};

#[derive(Debug)]
pub struct AABB 
{
	pub color: Vec3,

    pub translation: Vec3,
    pub scale: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub is_static: bool,
    pub damping: f32,
    pub restitution: f32, // The bounciness of the object
}

impl AABB 
{
    // Constructor that also initializes the half-size
    pub fn new(translation: &Vec3, size: &Vec3, velocity: Vec3, mass: f32, is_static: bool, damping: f32, restitution: f32) -> AABB 
    {
        AABB 
        {
			color: Vec3::new(1.0f32),
            translation: translation.clone(),
            scale: size.clone(),
            velocity: velocity,
            mass: mass,
            is_static: is_static,
            damping: damping,
            restitution: restitution,
        }
    }

	pub fn new_empty() -> AABB 
    {
        AABB 
        {
			color: Vec3::new(1.0f32),
            translation: Vec3::new(0.0f32),
            scale: Vec3::new(1.0f32),
            velocity: Vec3::new(0.0f32),
            mass: 1.0f32,
            is_static: false,
            damping: 0.5,
            restitution: 0.9,
        }
    }

	pub fn get_geometry(&self) -> (Vec<Vertex>, Vec<u32>)
	{
		let out_vertices = 
			vec![
				Vertex::from_position( Vec3 { x: self.scale.x, y: self.scale.y, z: self.scale.z } ),
				Vertex::from_position( Vec3 { x: -self.scale.x, y: self.scale.y, z: self.scale.z } ),
				Vertex::from_position( Vec3 { x: -self.scale.x, y: -self.scale.y, z: self.scale.z } ),
				Vertex::from_position( Vec3 { x: self.scale.x, y: -self.scale.y, z: self.scale.z } ),
				Vertex::from_position( Vec3 { x: self.scale.x, y: self.scale.y, z: -self.scale.z } ),
				Vertex::from_position( Vec3 { x: -self.scale.x, y: self.scale.y, z: -self.scale.z } ),
				Vertex::from_position( Vec3 { x: -self.scale.x, y: -self.scale.y, z: -self.scale.z } ),
				Vertex::from_position( Vec3 { x: self.scale.x, y: -self.scale.y, z: -self.scale.z } ),
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

    // Method to compute penetration depth
    // fn compute_penetration(&self, other: &AABB) -> Vec3 
    pub fn compute_penetration(&self, other: &AABB) -> Vec3 
    {
        Vec3 
        {
            x: (other.translation.x - self.translation.x).abs() - self.scale.x - other.scale.x,
            y: (other.translation.y - self.translation.y).abs() - self.scale.y - other.scale.y,
            z: (other.translation.z - self.translation.z).abs() - self.scale.z - other.scale.z,
        }.negate()
    }

    // New collision response method
    pub fn check_and_respond_collision(&mut self, other: &AABB) 
    {
        if self.is_static 
        {
            // Static objects do not move or respond to collisions
            return;
        }

        let penetration = self.compute_penetration(other);

        if penetration.x > 0.0 && penetration.y > 0.0 && penetration.z > 0.0 
        {
            // Find the axis with the minimal penetration
            let axis_of_least_penetration = 
				if penetration.x.min(penetration.y).min(penetration.z) == penetration.x 
				{ 
					"x" 
				} 
				else if penetration.x.min(penetration.y).min(penetration.z) == penetration.y 
				{ 
					"y" 
				} 
				else 
				{ 
					"z" 
				};

            // Reflect velocity vector based on the axis of collision
            match axis_of_least_penetration 
            {
                "x" => self.velocity.x = -self.velocity.x * self.restitution,
                "y" => self.velocity.y = -self.velocity.y * self.restitution,
                "z" => self.velocity.z = -self.velocity.z * self.restitution,
                _ => {},
            }

            // Correct the translation to avoid sinking into the other AABB due to penetration
            // match axis_of_least_penetration 
            // {
            //     "x" => self.translation.x += penetration.x,
            //     "y" => self.translation.y += penetration.y,
            //     "z" => self.translation.z += penetration.z,
            //     _ => {},
            // }
        }
    }

	pub fn apply_gravity(&mut self, delta_time_ms: f32)
    {
        let gravity = Vec3 { x: 0.0, y: -9.81, z: 0.0 };

        if !self.is_static
        {
            self.velocity += gravity * (delta_time_ms / 1000f32);
        }
    }

    // Method to update the AABB considering gravity and collisions
    pub fn update(&mut self, other: &AABB, dt: f32) 
    {
        // Apply gravity and update translation only if the object is not static
        if !self.is_static 
        {
            // Assuming apply_gravity is a method that updates self.velocity
            self.apply_gravity(dt); 
            // Apply damping linearly instead of using the power function
            self.velocity *= 1.0 - (self.damping * dt);
            self.translation += self.velocity * dt;
            self.check_and_respond_collision(other);
        }
    }
}