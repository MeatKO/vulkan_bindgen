use decs::component_derive::component;
use decs::component::Component;

use crate::{cotangens::vec3::Vec3, vulkan::vertex::Vertex};

#[component]
pub struct AABB 
{
	pub color: Vec3,

    pub translation: Vec3,
    pub scale: Vec3,
    pub velocity: Vec3,
    pub pressure_force: Vec3,
    pub mass: f32,
    pub is_static: bool,
    pub damping: f32,
    pub restitution: f32, // The bounciness of the object
}

impl AABB 
{
    // Constructor that also initializes the half-size
    pub fn new(translation: &Vec3, size: &Vec3, velocity: Vec3, pressure: Vec3, mass: f32, is_static: bool, damping: f32, restitution: f32) -> AABB 
    {
        AABB 
        {
			color: Vec3::new(1.0f32),
            translation: translation.clone(),
            scale: size.clone(),
            velocity: velocity,
            pressure_force: pressure,
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
            pressure_force: Vec3::new(0.0f32),
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

	// gives camera-relative coordinate of the nearest penetration
	pub fn raycast_hit(&self, ray_origin: Vec3, ray_direction: Vec3) -> Option<Vec3> 
    {
        let inv_direction = Vec3 
        {
            x: 1.0 / ray_direction.x,
            y: 1.0 / ray_direction.y,
            z: 1.0 / ray_direction.z,
        };

        let bounds = [self.translation - self.scale, self.translation + self.scale];
        let mut t_min = (bounds[(ray_direction.x < 0.0) as usize].x - ray_origin.x) * inv_direction.x;
        let mut t_max = (bounds[(ray_direction.x >= 0.0) as usize].x - ray_origin.x) * inv_direction.x;
        let t_y_min = (bounds[(ray_direction.y < 0.0) as usize].y - ray_origin.y) * inv_direction.y;
        let t_y_max = (bounds[(ray_direction.y >= 0.0) as usize].y - ray_origin.y) * inv_direction.y;

        if (t_min > t_y_max) || (t_y_min > t_max) 
        {
            return None;
        }

        if t_y_min > t_min 
        {
            t_min = t_y_min;
        }

        if t_y_max < t_max 
        {
            t_max = t_y_max;
        }

        let t_z_min = (bounds[(ray_direction.z < 0.0) as usize].z - ray_origin.z) * inv_direction.z;
        let t_z_max = (bounds[(ray_direction.z >= 0.0) as usize].z - ray_origin.z) * inv_direction.z;

        if (t_min > t_z_max) || (t_z_min > t_max) 
        {
            return None;
        }

        if t_z_min > t_min 
        {
            t_min = t_z_min;
        }

        if t_z_max < t_max 
        {
            t_max = t_z_max;
        }

        if t_min < 0.0 && t_max < 0.0 
        {
            return None;
        }

        let t = if t_min < 0.0 { t_max } else { t_min };
        Some(ray_direction * t)
    }
}