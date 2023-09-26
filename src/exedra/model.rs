use crate::cotangens::{
	vec2::*,
	vec3::*,
};
use crate::vulkan::uniform_buffer::UniformBufferObject;
use crate::vulkan::vertex::*;
use crate::vulkan::handle::{
	VkHandle,
};
use crate::vulkan::vk_bindgen::{
	VkBuffer, 
	VkDeviceMemory,
	vkDestroyBuffer,
	vkFreeMemory,
	VkImage,
	VkSampler,
	VkImageView, 
	VkDescriptorPool, 
	VkDescriptorSet, vkDestroySampler, vkDestroyImageView, vkDestroyImage, vkDestroyDescriptorPool, vkDestroyDescriptorSetLayout,
};

use std::collections::HashMap;
use std::ptr::null_mut as nullptr;
use std::time;

pub struct Model
{
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,

	pub index_count: u32,

	pub vertex_buffer: VkBuffer,
	pub vertex_buffer_memory: VkDeviceMemory,

	pub index_buffer: VkBuffer,
	pub index_buffer_memory: VkDeviceMemory,

	pub texture_image: VkImage,
	pub texture_image_memory: VkDeviceMemory,
	pub texture_image_view: VkImageView,
	pub texture_sampler: VkSampler,

	pub uniform_buffers: Vec<VkBuffer>,
	pub uniform_buffers_memory: Vec<VkDeviceMemory>,
	pub uniform_buffers_mapped: Vec<*mut UniformBufferObject>,

	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_sets: Vec<VkDescriptorSet>,
}

impl Model
{
	pub fn destroy(&self, vk_handle: &mut VkHandle)
	{
		unsafe
		{
			vkDestroyBuffer(vk_handle.logical_device, self.vertex_buffer, nullptr());
			vkFreeMemory(vk_handle.logical_device, self.vertex_buffer_memory, nullptr());
		
			vkDestroyBuffer(vk_handle.logical_device, self.index_buffer, nullptr());
			vkFreeMemory(vk_handle.logical_device, self.index_buffer_memory, nullptr());

			vkDestroySampler(vk_handle.logical_device, self.texture_sampler, nullptr());
			vkDestroyImageView(vk_handle.logical_device, self.texture_image_view, nullptr());
			
			vkDestroyImage(vk_handle.logical_device, self.texture_image, nullptr());
			vkFreeMemory(vk_handle.logical_device, self.texture_image_memory, nullptr());

			for i in 0..vk_handle.frames_in_flight
			{
				vkDestroyBuffer(vk_handle.logical_device, self.uniform_buffers[i], nullptr());
				vkFreeMemory(vk_handle.logical_device, self.uniform_buffers_memory[i], nullptr());
			}

			vkDestroyDescriptorPool(vk_handle.logical_device, self.descriptor_pool, nullptr());
			// vkDestroyDescriptorSetLayout(vk_handle.logical_device, vk_handle.descriptor_set_layout, nullptr());
		}
	}

	// not doing normals for now ?
	pub fn load<T: ToString>(model_path: T) -> Result<Model, String>
	{
		let start = time::Instant::now();

		let mut out_model = 
			Model {
				vertices: vec![],
				indices: vec![],
				index_count: 0,
				vertex_buffer: nullptr(),
				vertex_buffer_memory: nullptr(),
				index_buffer: nullptr(),
				index_buffer_memory: nullptr(),
				texture_image_view: nullptr(),
				texture_image: nullptr(),
				texture_image_memory: nullptr(),
				texture_sampler: nullptr(),
				uniform_buffers: vec![],
				uniform_buffers_memory: vec![],
				uniform_buffers_mapped: vec![],
				descriptor_pool: nullptr(),
				descriptor_sets: vec![],
			};
		
		let obj_model_source = 
			std::fs::read_to_string(model_path.to_string()).map_err(|err| err.to_string())?;

		let mut v_vec = vec![];
		let mut vt_vec = vec![];
		let mut f_vec = vec![];

		for line in obj_model_source.lines()
		{
			// println!("{}", line);
			let tokens: Vec<&str> = line.split(' ').collect();

			let line_header = 
				match tokens.first().copied()
				{
					Some(header) => { header }
					None => { continue }
				};

			match line_header
			{
				"v" =>
				{
					if tokens.len() != 4
					{
						return Err(format!("expected 4 vertex tokens, received {}", tokens.len()).to_owned())
					}

					let new_vertex_pos =
						Vec3{
							x: tokens[1].parse().unwrap(),
							y: tokens[2].parse().unwrap(),
							z: tokens[3].parse().unwrap(),
						};

					v_vec.push(new_vertex_pos);
				}
				"vt" =>
				{
					if tokens.len() != 3
					{
						return Err(format!("expected 3 uv tokens, received {}", tokens.len()).to_owned())
					}

					let new_vertex_uv =
						Vec2{
							x: tokens[1].parse().unwrap(),
							y: tokens[2].parse().unwrap(),
						};

					vt_vec.push(new_vertex_uv);
				}
				"f" =>
				{
					if tokens.len() != 4
					{
						return Err(format!("expected 4 face tokens, received {}", tokens.len()).to_owned())
					}

					for i in 1..4
					{
						let tokens: Vec<&str> = tokens[i].split('/').collect();

						// println!("tok {:?}", tokens);

						let new_face =
							Vec3{
								x: tokens[0].parse::<u32>().unwrap() as _,
								y: tokens[1].parse::<u32>().unwrap() as _,
								z: tokens[2].parse::<u32>().unwrap() as _,
							};

						f_vec.push(new_face);
					}
				}
				_ => { continue }
			}
		}

		// for faces in f_vec.chunks(3)
		// {
		// 	println!("f {:?} {:?} {:?}", faces[0], faces[1], faces[2]);
		// }

		// let mut vertex_duplication_map: HashMap<Vertex, u32> = HashMap::new();

		for face in f_vec
		{
			let new_vertex = 
				Vertex {
					pos: v_vec[face.x as usize - 1].clone(),
					uv: vt_vec[face.y as usize - 1].clone(),
					normal: Vec3 { 
						x: 1.0f32, 
						y: 1.0f32, 
						z: 1.0f32, 
					},
				};

			// if vertex_duplication_map.contains_key(&new_vertex)
			// {
			// 	out_model.indices.push(vertex_duplication_map[&new_vertex]);
			// }
			// else 
			// {
			// 	vertex_duplication_map.insert(new_vertex.clone(), out_model.vertices.len() as _);
			// 	out_model.indices.push(out_model.vertices.len() as _);
			// 	out_model.vertices.push(new_vertex);
			// }

			out_model.vertices.push(new_vertex);
			out_model.indices.push(out_model.indices.len() as _);
		}

		out_model.index_count = out_model.indices.len() as u32;

		let end = std::time::Instant::now();

		println!("Model loading time : {}s", end.duration_since(start).as_secs_f32());
		println!("Vertex count : {}\nIndex count : {}", out_model.vertices.len(), out_model.indices.len());

		Ok(out_model)
	}
}