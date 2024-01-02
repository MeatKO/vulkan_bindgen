use decs::component_derive::component;
use decs::component::Component;

use crate::vulkan::{vk_bindgen::{
	VkBuffer, 
	VkDeviceMemory,
	VkDescriptorPool, 
	VkDescriptorSet,
}, uniform_buffer::UniformBufferObject};

use std::ptr::null_mut as nullptr;

use crate::{cotangens::vec3::Vec3, vulkan::{vertex::{Vertex, create_vertex_buffer}, handle::VkHandle, index::create_index_buffer, uniform_buffer::create_uniform_buffers, descriptor_set_wireframe::create_descriptor_sets_wireframe, wrappers::vk_descriptor_pool::VkDescriptorPoolBuilder, vk_bindgen::VkDescriptorType}};

#[derive(Debug)]
pub struct VulkanMeshData
{
	pub vertex_buffer: VkBuffer,
	pub vertex_buffer_memory: VkDeviceMemory,

	pub index_buffer: VkBuffer,
	pub index_buffer_memory: VkDeviceMemory,

	pub uniform_buffers: Vec<VkBuffer>,
	pub uniform_buffers_memory: Vec<VkDeviceMemory>,
	pub uniform_buffers_mapped: Vec<*mut UniformBufferObject>,

	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_sets: Vec<VkDescriptorSet>,
}

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

	pub aabb_vulkan_data: Option<VulkanMeshData>,
	pub aabb_index_count: u32,
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
            aabb_vulkan_data: None,
            aabb_index_count: 0,
			
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
			aabb_vulkan_data: None,
            aabb_index_count: 0,
        }
    }

	pub fn new_nonverbose(translation: Vec3, scale: Vec3, is_static: bool) -> AABB 
    {
        AABB 
        {
			color: Vec3::new(1.0f32),
            translation: translation,
            scale: scale,
            velocity: Vec3::new(0.0f32),
            pressure_force: Vec3::new(0.0f32),
            mass: 1.0f32,
            is_static: is_static,
            damping: 0.5,
            restitution: 0.9,
			aabb_vulkan_data: None,
            aabb_index_count: 0,
        }
    }

	pub unsafe fn process_vulkan(&mut self, vk_handle: &VkHandle)
	{
		let (vertex_vec, index_vec) = AABB::new_empty().get_geometry();

		let (vertex_buffer, vertex_buffer_memory) =
			create_vertex_buffer(&vk_handle, &vertex_vec)
			.unwrap();

		let (index_buffer, index_buffer_memory) =
			create_index_buffer(&vk_handle, &index_vec)
			.unwrap();

		let mut mesh_data =
			VulkanMeshData{
				vertex_buffer: vertex_buffer,
				vertex_buffer_memory: vertex_buffer_memory,
				index_buffer: index_buffer,
				index_buffer_memory: index_buffer_memory,
				uniform_buffers: vec![],
				uniform_buffers_memory: vec![],
				uniform_buffers_mapped: vec![],
				descriptor_pool: nullptr(),
				descriptor_sets: vec![],
			};

		// create_uniform_buffers(&vk_handle, &mut mesh_data);
		let uniform_buffers = create_uniform_buffers(&vk_handle, vk_handle.frames_in_flight).unwrap();
		mesh_data.uniform_buffers = uniform_buffers.0;
		mesh_data.uniform_buffers_memory = uniform_buffers.1;
		mesh_data.uniform_buffers_mapped = uniform_buffers.2;

		// let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();

		let descriptor_pool = 
			VkDescriptorPoolBuilder::new()
			.add_pool_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, vk_handle.frames_in_flight)
			.build(vk_handle.logical_device, vk_handle.frames_in_flight)
			.unwrap();

		mesh_data.descriptor_pool = descriptor_pool;
		create_descriptor_sets_wireframe(&vk_handle, &mut mesh_data).unwrap();

		self.aabb_vulkan_data = Some(mesh_data);
		self.aabb_index_count = index_vec.len() as _;
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