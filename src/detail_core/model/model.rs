use std::{ptr::null_mut as nullptr, ops::{Deref, DerefMut}, path::PathBuf, collections::HashMap, rc::Rc};

use decs::component_derive::component;
use decs::component::Component;

use crate::{cotangens::{vec3::*, vec2::Vec2}, exedra::{error::ModelLoadError, model_descriptor::ModelDescriptor, mesh_descriptor::MeshDescriptor, material_descriptor::MaterialDescriptor}, detail_core::texture::texture::Texture, vulkan::{handle::VkHandle, vertex::{create_vertex_buffer, Vertex}, index::create_index_buffer, descriptor_set::{create_descriptor_sets, update_descriptor_sets}, uniform_buffer::create_uniform_buffers, vk_bindgen::VkFormat, wrappers::vk_buffer::VulkanBuffer}};
use super::{mesh::{Mesh, VulkanMeshData}, material::Material, asset::{ModelAsset, MeshAsset, MeshVulkanBuffers, MeshBuffers, MaterialAsset}};

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

	// pub fn process_meshes(self, vk_handle: &VkHandle, material_defaults: Material) -> Model<VulkanModel>
	// {
	// 	Model(
	// 		VulkanModel::new(vk_handle, self.0, material_defaults).unwrap()
	// 	)
	// }

	pub fn to_asset(self, vk_handle: &VkHandle) -> Result<ModelAsset, ModelLoadError>
	{
		let mut out_model_asset = ModelAsset::new_empty(self.name.clone());

		// processing the meshes
		for mesh in self.meshes.iter()
		{
			let mut current_mesh_asset = MeshAsset::new_empty(mesh.name.clone());

			current_mesh_asset.material_asset_name = "".to_owned();

			let buffers = unsafe { create_geometry_buffers(vk_handle, &self.0, mesh)? };

			current_mesh_asset.mesh_vulkan_buffers = buffers.0;
			current_mesh_asset.mesh_buffers = buffers.1;

			out_model_asset.meshes.push(current_mesh_asset);
		}

		Ok(out_model_asset)
	}

	// pub fn to_asset_material(self, vk_handle: &VkHandle, default_material: Rc<MaterialAsset>) -> Result<(ModelAsset, Vec<MaterialAsset>), ModelLoadError>
	pub fn to_asset_material(self, vk_handle: &VkHandle) -> Result<(ModelAsset, Vec<MaterialAsset>), ModelLoadError>
	{
		let mut out_model_asset = ModelAsset::new_empty(self.name.clone());
		let mut out_material_assets = vec![];

		// let mut used_materials_map: HashMap<String, MaterialDescriptor> = HashMap::new();
		let mut used_materials_vec: Vec<MaterialDescriptor> = vec![];

		// processing the meshes
		for mesh in self.meshes.iter()
		{
			let mut current_mesh_asset = MeshAsset::new_empty(mesh.name.clone());

			// current_mesh_asset.material_asset_name = "".to_owned();
			current_mesh_asset.material_asset_name = mesh.material.name.clone();

			used_materials_vec.push(mesh.material.clone());

			let buffers = unsafe { create_geometry_buffers(vk_handle, &self.0, mesh)? };

			current_mesh_asset.mesh_vulkan_buffers = buffers.0;
			current_mesh_asset.mesh_buffers = buffers.1;

			out_model_asset.meshes.push(current_mesh_asset);
		}

		// we need to sort and dedup the used_materials_vec
		used_materials_vec.sort_by(|a, b| a.name.cmp(&b.name));
		used_materials_vec.dedup_by(|a, b| a.name == b.name);

		for material in used_materials_vec.iter_mut()
		{
			let mut current_material_asset = MaterialAsset::new_empty(material.name.clone());
			// current_material_asset.descriptor_set = default_material.descriptor_set;

			current_material_asset.smooth_shading = material.smooth_shading;

			let albedo_texture = 
				match Texture::new(material.albedo_path.clone().into()).load()
				{
					Ok(loaded_texture) => loaded_texture,
					Err(err) => { return Err(ModelLoadError::TextureLoadingError(material.albedo_path.clone(), err.to_string())); }
				};

			let normal_texture = 
				match Texture::new(material.normal_path.clone().into()).load()
				{
					Ok(loaded_texture) => loaded_texture,
					Err(err) => { return Err(ModelLoadError::TextureLoadingError(material.normal_path.clone(), err.to_string())); }
				};

			// let normal_texture = Texture::new(material.normal_path.clone().into()).load().unwrap();
			
			let albedo_texture = albedo_texture.process_vk(vk_handle, VkFormat::VK_FORMAT_R8G8B8A8_SRGB).unwrap();
			let normal_texture = normal_texture.process_vk(vk_handle, VkFormat::VK_FORMAT_R8G8B8A8_UNORM).unwrap();

			let descriptor_set_layout = vk_handle.global_descriptor_set_layout_material;
			let descriptor_set_vec = unsafe { create_descriptor_sets(vk_handle, &vk_handle.global_descriptor_pool_material, &descriptor_set_layout, 1).unwrap() };

			unsafe { update_descriptor_sets(vk_handle, &descriptor_set_vec, &albedo_texture, &normal_texture).unwrap(); }

			current_material_asset.descriptor_set = descriptor_set_vec[0];

			out_material_assets.push(current_material_asset);
		}
		
		Ok((out_model_asset, out_material_assets))
	}
}

unsafe fn create_geometry_buffers(vk_handle: &VkHandle, model_descriptor: &ModelDescriptor, mesh_descriptor: &MeshDescriptor) -> Result<(MeshVulkanBuffers, MeshBuffers), ModelLoadError>
{
	let mut vertex_vec: Vec<Vertex> = vec![];
	let mut index_vec: Vec<u32> = vec![];
		
	let face_ver_max_index = mesh_descriptor.face_vtn_vec.iter().map(|face_vtn| face_vtn[0]).max().unwrap();
	let face_uv_max_index = mesh_descriptor.face_vtn_vec.iter().map(|face_vtn| face_vtn[1]).max().unwrap();
	let face_normal_max_index = mesh_descriptor.face_vtn_vec.iter().map(|face_vtn| face_vtn[2]).max().unwrap();

	if  face_ver_max_index > model_descriptor.vertex_vec.len() ||
		face_uv_max_index > model_descriptor.uv_vec.len() ||
		face_normal_max_index > model_descriptor.normal_vec.len()
	{
		return Err(
			ModelLoadError::ValidationError(
				format!(
					"face vertex index out of bounds\nface_max_value: v{}, uv{}, n{}\nwith vector lengths: v{}, uv{}, n{}", 
					face_ver_max_index, face_uv_max_index, face_normal_max_index, 
					model_descriptor.vertex_vec.len(), model_descriptor.uv_vec.len(), model_descriptor.normal_vec.len()
				)
			)
		);
	}

	for face in mesh_descriptor.face_vtn_vec.iter()
	{
		let new_vertex = 
			Vertex {
				pos: model_descriptor.vertex_vec[face[0] - 1].clone(),
				uv: model_descriptor.uv_vec[face[1] - 1].clone(),
				normal: model_descriptor.normal_vec[face[2] - 1].clone(),
				tangent: Vec3::new(1.0f32),
				bitangent: Vec3::new(1.0f32),
			};

		vertex_vec.push(new_vertex);
		index_vec.push(index_vec.len() as _);
	}

	if vertex_vec.len() % 3 != 0
	{
		panic!("vertex vec len not divisible by 3 lo");
	}

	// calculating the TBN
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

	return Ok(
		(
			MeshVulkanBuffers{
				vertex: VulkanBuffer{ buffer: vertex_buffer, memory: vertex_buffer_memory },
				index: VulkanBuffer{ buffer: index_buffer, memory: index_buffer_memory },
				index_count: index_vec.len(),
			},
			MeshBuffers{
				vertex: vertex_vec,
				index: index_vec
			}
		)
	);
}

#[component]
pub struct VulkanModel
{
	pub name: String,
	pub meshes: Vec<Mesh>,
}

impl VulkanModel
{
	pub fn process_textures(&mut self, vk_handle: &VkHandle) -> Result<(), String>
	{
		unsafe 
		{
			// Ok(())
			let mut total_bytes_loaded = 0usize;

			for mesh in self.meshes.iter_mut()
			{
				let mesh_material_albedo_map = 
					match Texture::new(mesh.material.albedo_path.clone().into()).load()
					{
						Ok(loaded_texture) =>
						{
							loaded_texture
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

				let uniform_buffers = create_uniform_buffers(&vk_handle, vk_handle.frames_in_flight).unwrap();
				let mesh_data = mesh.vulkan_data.as_mut().unwrap();
				mesh_data.uniform_buffers = uniform_buffers.0;
				mesh_data.uniform_buffers_memory = uniform_buffers.1;
				mesh_data.uniform_buffers_mapped = uniform_buffers.2;

				// let descriptor_sets = create_descriptor_sets(&vk_handle, &descriptor_pool).unwrap();

				let descriptor_sets = 
					create_descriptor_sets(
						&vk_handle, 
						&vk_handle.global_descriptor_pool_material, 
						&vk_handle.global_descriptor_set_layout_material, 
						1
					).unwrap();

				mesh.vulkan_data.as_mut().unwrap().descriptor_sets = descriptor_sets;
				
				update_descriptor_sets(&vk_handle, &mesh.vulkan_data.as_mut().unwrap().descriptor_sets, &mesh_material_albedo_map, &mesh_material_normal_map).unwrap();

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
				};
			
			// process all the meshes of the model
			for mesh_descriptor in model_descriptor.meshes.into_iter()
			{
				let mesh_material_albedo_map = material_defaults.albedo_map.as_ref().unwrap().clone();
				let mesh_material_normal_map = material_defaults.normal_map.as_ref().unwrap().clone();

				let mut vertex_vec: Vec<Vertex> = vec![];
				let mut index_vec: Vec<u32> = vec![];
					
				let face_x_max_value = mesh_descriptor.face_vtn_vec.iter().map(|face| face[0]).max().unwrap();
				let face_y_max_value = mesh_descriptor.face_vtn_vec.iter().map(|face| face[1]).max().unwrap();
				let face_z_max_value = mesh_descriptor.face_vtn_vec.iter().map(|face| face[2]).max().unwrap();

				if  face_x_max_value > model_descriptor.vertex_vec.len() ||
					face_y_max_value > model_descriptor.uv_vec.len() ||
					face_z_max_value > model_descriptor.normal_vec.len()
				{
					return Err(
						ModelLoadError::ValidationError(
							format!(
								"face vertex index out of bounds\nface_max_value: v{}, uv{}, n{}\nwith vector lengths: v{}, uv{}, n{}", 
								face_x_max_value, face_y_max_value, face_z_max_value, 
								model_descriptor.vertex_vec.len(), model_descriptor.uv_vec.len(), model_descriptor.normal_vec.len()
							)
						)
					);
				}

				for face in mesh_descriptor.face_vtn_vec
				{
					let new_vertex = 
						Vertex {
							pos: model_descriptor.vertex_vec[face[0] - 1].clone(),
							uv: model_descriptor.uv_vec[face[1] - 1].clone(),
							normal: model_descriptor.normal_vec[face[2] - 1].clone(),
							tangent: Vec3::new(1.0f32),
							bitangent: Vec3::new(1.0f32),
						};

					vertex_vec.push(new_vertex);
					index_vec.push(index_vec.len() as _);
				}

				if vertex_vec.len() % 3 != 0
				{
					panic!("vertex vec len not divisible by 3 lo");
				}

				// calculating the TBN
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

				let uniform_buffers = create_uniform_buffers(&vk_handle, vk_handle.frames_in_flight).unwrap();
				mesh_data.uniform_buffers = uniform_buffers.0;
				mesh_data.uniform_buffers_memory = uniform_buffers.1;
				mesh_data.uniform_buffers_mapped = uniform_buffers.2;

				// let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				// create_descriptor_sets(&vk_handle, &mut mesh_data, &mesh_material_albedo_map, &mesh_material_normal_map, &descriptor_pool).unwrap();
				// mesh_data.descriptor_pool = descriptor_pool;

				// let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				// let descriptor_sets = create_descriptor_sets(&vk_handle, &descriptor_pool).unwrap();

				// mesh.vulkan_data.as_mut().unwrap().descriptor_pool = descriptor_pool;
				// mesh_data.descriptor_sets = descriptor_sets;
				
				// update_descriptor_sets(&vk_handle, &mut mesh_data, &mesh_material_albedo_map, &mesh_material_normal_map).unwrap();

				out_model.meshes.push(
					Mesh{
						name: mesh_descriptor.name,
						material: Material {
							name: mesh_descriptor.mtl_name, 
							albedo_path: mesh_descriptor.material.albedo_path,
							normal_path: mesh_descriptor.material.normal_path,
							albedo_map: Some(mesh_material_albedo_map),
							normal_map: Some(mesh_material_normal_map),
							descriptor_set: nullptr(),
						},
						// material: Material::new(mesh_descriptor.mtl_name.clone()),
						index_count: index_vec.len() as u32,
						vulkan_data: Some(mesh_data)
					}
				);
			};

			Ok(out_model)
		}
	}
}