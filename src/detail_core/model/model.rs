use std::{ptr::null_mut as nullptr, ops::{Deref, DerefMut}, path::PathBuf};

use decs::component_derive::component;
use decs::component::Component;

use crate::{cotangens::{vec3::*, vec2::Vec2}, exedra::{error::ModelLoadError, model_descriptor::ModelDescriptor}, detail_core::{texture::texture::{Texture, VulkanTexture}, phys::aabb::AABB}, vulkan::{handle::VkHandle, vertex::{create_vertex_buffer, Vertex}, index::create_index_buffer, descriptor_set::create_descriptor_sets, descriptor_pool::create_descriptor_pool, uniform_buffer::create_uniform_buffers, vk_bindgen::VkFormat, descriptor_set_wireframe::create_descriptor_sets_wireframe}};

use super::{mesh::{Mesh, VulkanMeshData}, material::Material};

// #[component]

#[derive(Debug)]
pub struct Model<T>(T);

impl <T: Component> Component for Model<T> {}

impl<T> Deref for Model<T> 
{
    type Target = T;

    fn deref(&self) -> &T 
	{
        &self.0
    }
}

impl<T> DerefMut for Model<T> 
{
    fn deref_mut(&mut self) -> &mut T 
	{
        &mut self.0
    }
}

impl Model<ModelDescriptor>
{
	pub fn new(in_file_path: PathBuf) -> Self
	{
		Model(
			ModelDescriptor::load(in_file_path).unwrap()
		)
	}

	pub fn process_meshes(self, vk_handle: &VkHandle, material_defaults: Material) -> Model<VulkanModel>
	{
		Model(
			VulkanModel::new(vk_handle, self.0, material_defaults).unwrap()
		)
	}
}

#[component]
pub struct VulkanModel
{
	pub name: String,
	pub meshes: Vec<Mesh>,
	pub scale: Vec3,
	pub translation: Vec3,
	pub rotation: Vec3,

	pub aabb: AABB,
	pub aabb_vulkan_data: Option<VulkanMeshData>,
	pub aabb_index_count: u32,
}

impl VulkanModel
{
	pub fn process_textures(&mut self, vk_handle: &VkHandle) -> Result<(), String>
	{
		unsafe 
		{
			let mut total_bytes_loaded = 0usize;

			for mesh in self.meshes.iter_mut()
			{
				let mesh_material_albedo_map = 
					match Texture::new(mesh.material.albedo_path.clone().into()).load()
					{
						Ok(loaded_texture) =>
						{
							loaded_texture
							// .downscale(0.1f32)
							.process_vk(
								vk_handle, 
								VkFormat::VK_FORMAT_R8G8B8A8_SRGB
							)
							?
						}
						Err(_) => { mesh.material.albedo_map.as_ref().unwrap().clone() }
					};

				total_bytes_loaded += mesh_material_albedo_map.byte_size;
	
				println!("loading texture {}", mesh.material.normal_path.clone());
				let mesh_material_normal_map = 
					match Texture::new(mesh.material.normal_path.clone().into()).load()
					{
						Ok(loaded_texture) => 
						// { mesh.material.normal_map.as_ref().unwrap().clone() }
						{
							loaded_texture
							.process_vk(
								vk_handle, 
								VkFormat::VK_FORMAT_R8G8B8A8_UNORM
							)
							?
						}
						Err(_) => { mesh.material.normal_map.as_ref().unwrap().clone() }
					};
	
				mesh.material.albedo_map = Some(mesh_material_albedo_map.clone());
				mesh.material.normal_map = Some(mesh_material_normal_map.clone());
	
				create_uniform_buffers(&vk_handle, &mut mesh.vulkan_data.as_mut().unwrap());
	
				let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				create_descriptor_sets(&vk_handle, &mut mesh.vulkan_data.as_mut().unwrap(), &mesh_material_albedo_map, &mesh_material_normal_map, &descriptor_pool).unwrap();
				mesh.vulkan_data.as_mut().unwrap().descriptor_pool = descriptor_pool;

				println!("Total texture size for model {} : {} bytes", self.name, total_bytes_loaded);
			}

			// println!("Total texture size for model {} : {} bytes", self.name, total_bytes_loaded);
	
			Ok(())
		}
	}

	// currently not returning any errors, its an unwrap and non-handling shitfest but its cozy
	fn new(vk_handle: &VkHandle, model_descriptor: ModelDescriptor, material_defaults: Material) -> Result<VulkanModel, ModelLoadError>
	{
		unsafe
		{
			let mut out_model = 
				VulkanModel{
					name: model_descriptor.name,
					meshes: vec![],
					scale: Vec3::new(1.0f32),
					translation: Vec3::new(0.0f32),
					rotation: Vec3::new(0.0f32),
					aabb: AABB::new_empty(),
					aabb_vulkan_data: None,
					aabb_index_count: 0u32,
				};
			
			// process the aabb box of the model
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

				create_uniform_buffers(&vk_handle, &mut mesh_data);

				let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				create_descriptor_sets_wireframe(&vk_handle, &mut mesh_data, &descriptor_pool).unwrap();
				mesh_data.descriptor_pool = descriptor_pool;

				out_model.aabb_vulkan_data = Some(mesh_data);
				out_model.aabb_index_count = index_vec.len() as _;
			}

			// process all the meshes of the model
			for mesh_descriptor in model_descriptor.meshes.into_iter()
			{
				let mesh_material_albedo_map = material_defaults.albedo_map.as_ref().unwrap().clone();

				let mesh_material_normal_map = material_defaults.normal_map.as_ref().unwrap().clone();

				let mut vertex_vec: Vec<Vertex> = vec![];
				let mut index_vec: Vec<u32> = vec![];
					
				for face in mesh_descriptor.face_vec
				{
					let new_vertex = 
						Vertex {
							pos: model_descriptor.vertex_vec[face.x as usize - 1].clone(),
							uv: model_descriptor.uv_vec[face.y as usize - 1].clone(),
							normal: model_descriptor.normal_vec[face.z as usize - 1].clone(),
							tangent: Vec3::new(1.0f32),
							bitangent: Vec3::new(1.0f32),
						};

					vertex_vec.push(new_vertex);
					index_vec.push(index_vec.len() as _);
				}

				if vertex_vec.len() % 3 != 0
				{
					panic!("vertex len not divisible by 3 lo");
				}

				for triangle_points in vertex_vec.chunks_mut(3)
				{
					let edge_1: Vec3 = triangle_points[1].pos - triangle_points[0].pos;
					let edge_2: Vec3 = triangle_points[2].pos - triangle_points[0].pos;

					let delta_uv_1: Vec2 = triangle_points[1].uv - triangle_points[0].uv;
					let delta_uv_2: Vec2 = triangle_points[2].uv - triangle_points[0].uv;

					let f = 1.0f32 / (delta_uv_1.x * delta_uv_2.y - delta_uv_2.x * delta_uv_1.y);
					let mut tangent = Vec3::new(0.0f32);
					tangent.x = f * (delta_uv_2.y * edge_1.x - delta_uv_1.y * edge_2.x);
					tangent.y = f * (delta_uv_2.y * edge_1.y - delta_uv_1.y * edge_2.y);
					tangent.z = f * (delta_uv_2.y * edge_1.z - delta_uv_1.y * edge_2.z);

					let mut bitangent = Vec3::new(0.0f32);
					bitangent.x = f * (-delta_uv_2.x * edge_1.x + delta_uv_1.x * edge_2.x);
					bitangent.y = f * (-delta_uv_2.x * edge_1.y + delta_uv_1.x * edge_2.y);
					bitangent.z = f * (-delta_uv_2.x * edge_1.z + delta_uv_1.x * edge_2.z);

					for point in triangle_points.iter_mut()
					{
						point.tangent = tangent.clone();
						point.bitangent = bitangent.clone();
					}
				}

				let (vertex_buffer, vertex_buffer_memory) =
					create_vertex_buffer(&vk_handle, &mut vertex_vec)
					.unwrap();
		
				let (index_buffer, index_buffer_memory) =
					create_index_buffer(&vk_handle, &mut index_vec)
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

				create_uniform_buffers(&vk_handle, &mut mesh_data);

				let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				create_descriptor_sets(&vk_handle, &mut mesh_data, &mesh_material_albedo_map, &mesh_material_normal_map, &descriptor_pool).unwrap();
				mesh_data.descriptor_pool = descriptor_pool;

				out_model.meshes.push(
					Mesh{
						name: mesh_descriptor.name,
						material: Material {
							name: mesh_descriptor.mtl_name, 
							albedo_path: mesh_descriptor.material.albedo_path,
							normal_path: mesh_descriptor.material.normal_path,
							albedo_map: Some(mesh_material_albedo_map),
							normal_map: Some(mesh_material_normal_map),
						},
						index_count: index_vec.len() as u32,
						vulkan_data: Some(mesh_data)
					}
				);
			};

			Ok(out_model)
		}
	}
}