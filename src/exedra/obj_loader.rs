use crate::cotangens::{
	vec2::*,
	vec3::*,
};
use crate::vulkan::vertex::*;
use crate::vulkan::handle::{
	VkHandle,
};

use super::material::RenderingMaterial;
use super::mesh::Mesh;
use super::model::Model;
use std::error::Error;
use std::collections::HashMap;
use std::path::Path;
use std::ptr::null_mut as nullptr;
use std::fmt;

#[derive(Debug)]
pub enum ModelLoadError 
{
    IoError(std::io::Error),
	UnsupportedFileType,
    ParseError,
    UnexpectedTokenCount,
    UnknownToken,
	MaterialFileNotFound(String, std::io::Error),
	MaterialNotFound(String),
	MaterialWithoutMaps(String),
}

impl fmt::Display for ModelLoadError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
        match self 
		{
            ModelLoadError::IoError(err) => { write!(f, "IO Error: {}", err) },
            ModelLoadError::UnsupportedFileType => { write!(f, "Unsupported file type") },
            ModelLoadError::ParseError => { write!(f, "Failed to parse a value") },
            ModelLoadError::UnexpectedTokenCount => { write!(f, "Unexpected token count") },
            ModelLoadError::UnknownToken => { write!(f, "Unknown token encountered") },
            ModelLoadError::MaterialFileNotFound(mtl_name, err) => { write!(f, "Couldn't find material file '{}' : {}", mtl_name, err) },
            ModelLoadError::MaterialNotFound(mtl_name) => { write!(f, "Couldn't find material '{}'", mtl_name) },
            ModelLoadError::MaterialWithoutMaps(mtl_name) => { write!(f, "Couldn't parse material with no maps '{}'", mtl_name) },
        }
    }
}

impl Error for ModelLoadError {}

impl From<std::io::Error> for ModelLoadError 
{
    fn from(err: std::io::Error) -> ModelLoadError 
	{
        ModelLoadError::IoError(err)
    }
}

#[derive(Default)]
struct MeshDescriptor
{
	pub mesh_name: String,
	pub mtl_name: String,
	pub smooth_shading: bool,
	pub face_vec: Vec<Vec3>,
}

pub fn load_obj<P>(model_path: P) -> Result<Model, ModelLoadError>
where P : AsRef<Path>
{
	let start = std::time::Instant::now();
	
	let obj_model_source = std::fs::read_to_string(model_path.as_ref().clone())?;

	let mut global_v_vec = vec![];
	let mut global_vt_vec = vec![];
	let mut global_vn_vec = vec![];
	let mut global_mtllib_vec = vec![];

	let mut current_mesh = MeshDescriptor { ..Default::default() };

	let mut mesh_descriptor_vec: Vec<MeshDescriptor> = vec![];

	for line in obj_model_source.lines()
	{
		if line.starts_with('#') || line.trim().is_empty()
		{
            continue;
        }

		let (head, tail) = line.split_once(' ').ok_or(ModelLoadError::UnexpectedTokenCount)?;

		match head.to_lowercase().as_str()
		{
			"v" => global_v_vec.push(parse_vec3_line(tail)?),
			"vt" => global_vt_vec.push(parse_vec2_line(tail)?),
			"vn" => global_vn_vec.push(parse_vec3_line(tail)?),
			"f" => current_mesh.face_vec.extend(parse_face_line(tail)?), // only triangle faces for now !
			"mtllib" => { global_mtllib_vec.push(tail.to_owned()) }
			"usemtl" => 
			{
				if !current_mesh.face_vec.is_empty() && current_mesh.mtl_name != "".to_owned()
				{
					current_mesh.mesh_name = tail.to_owned();
					mesh_descriptor_vec.push(current_mesh);
				}

				current_mesh = MeshDescriptor { ..Default::default() };
				current_mesh.mtl_name = tail.to_owned();
			},
			"s" => 
			{
				current_mesh.smooth_shading =
					match tail
					{
						"1" | "on" => true,
						_ => false,
					}
			}
			"o" | "g" => 
			{
				let old_mtl_name = current_mesh.mtl_name.clone();
				if !current_mesh.face_vec.is_empty()
				{
					current_mesh.mesh_name = tail.to_owned();
					mesh_descriptor_vec.push(current_mesh);
				}

				// let old_mtl_name = current_mesh.mtl_name.clone();
				current_mesh = MeshDescriptor { ..Default::default() };
				current_mesh.mtl_name = old_mtl_name;
			}

			_ => { continue }
		}
	}
	if !current_mesh.face_vec.is_empty()
	{
		mesh_descriptor_vec.push(current_mesh)
	}

	let material_map = load_all_mtls(model_path.as_ref().clone(), global_mtllib_vec)?;

	let mut mesh_vec: Vec<Mesh> = vec![];

	for mesh_descriptor in mesh_descriptor_vec
	{
		let mut current_mesh = Mesh::new(mesh_descriptor.mesh_name.clone());

		println!("entire material map :");

		for (key, value) in &material_map {
			println!("'{}': '{}'", key, value.name);
		}

		println!("attempting to get material {}", mesh_descriptor.mtl_name);
		println!("&  {}", current_mesh.material.name);
		println!("&&  {:?}", material_map.get(&mesh_descriptor.mtl_name));
		current_mesh.name = mesh_descriptor.mesh_name;
		current_mesh.material = 
			material_map.get(&mesh_descriptor.mtl_name)
			.ok_or(ModelLoadError::MaterialNotFound(mesh_descriptor.mtl_name))?
			.clone();

		for face in mesh_descriptor.face_vec
		{
			let new_vertex = 
				Vertex {
					pos: global_v_vec[face.x as usize - 1].clone(),
					uv: global_vt_vec[face.y as usize - 1].clone(),
					normal: global_vn_vec[face.z as usize - 1].clone(),
				};

			current_mesh.vertices.push(new_vertex);
			current_mesh.indices.push(current_mesh.indices.len() as _);
		}

		current_mesh.index_count = current_mesh.indices.len() as u32;
		mesh_vec.push(current_mesh);
	}

	let mut out_model = 
		Model {
			name: model_path.as_ref().file_name().and_then(|name| name.to_str()).unwrap_or("unnamed_model").to_string(),
			meshes: mesh_vec
		};

	Ok(out_model)
}

pub fn load_all_mtls<P>(model_path: P, material_file_names: Vec<String>) -> Result<HashMap<String, RenderingMaterial>, ModelLoadError>
where P : AsRef<Path>
{
    let model_path_no_filename = model_path.as_ref().parent().unwrap_or(Path::new("./"));

	let mut out_material_hashmap: HashMap<String, RenderingMaterial> = HashMap::new();

	for material_file_name in material_file_names
	{
		let current_material_path = model_path_no_filename.join(material_file_name.clone());

		println!("using '{:?}' \nand '{:?}' \nwe arrived at '{:?}'", model_path.as_ref(), model_path_no_filename, current_material_path);

		let obj_material_source = 
			std::fs::read_to_string(current_material_path.clone())
			.map_err(|err| ModelLoadError::MaterialFileNotFound(current_material_path.clone().to_str().unwrap_or("unparseable_file_path").to_owned(), err))?;

		let mut current_material = RenderingMaterial::new("".to_owned());

		for line in obj_material_source.lines()
		{
			if line.starts_with('#') || line.trim().is_empty()
			{
				continue;
			}

			let (head, tail) = line.split_once(' ').ok_or(ModelLoadError::UnexpectedTokenCount)?;
			let tail = tail.replace(&[' ', '\t'], "");

			match head.to_lowercase().as_str()
			{
				// add error handling here later
				// "map_kd" => current_material.diffuse_map_rel_path = tail.to_owned(),
				"map_kd" => current_material.diffuse_map_rel_path = model_path_no_filename.join(tail.to_owned()).to_str().unwrap_or("unparseable_file_path").to_owned(),
				"newmtl" => 
				{
					if current_material.name != "".to_owned()
					{
						if current_material.diffuse_map_rel_path.trim().is_empty()
						{
							return Err(ModelLoadError::MaterialWithoutMaps(current_material.name.clone()))
						}

						out_material_hashmap.insert(current_material.name.clone(), current_material.clone());
					}

					current_material = RenderingMaterial::new(tail.to_owned());
				}

				_ => { continue }
			}
		}

		out_material_hashmap.insert(current_material.name.clone(), current_material.clone());
	}

	Ok(out_material_hashmap)
	// Err(ModelLoadError::ParseError)
}

fn parse_vec3_line(tail: &str) -> Result<Vec3, ModelLoadError>
{
	let tokens: Vec<&str> = tail.split_whitespace().collect();
    if tokens.len() != 3 
	{
        return Err(ModelLoadError::UnexpectedTokenCount);
    }

    Ok(Vec3 {
        x: tokens[0].parse().map_err(|_| ModelLoadError::ParseError)?,
        y: tokens[1].parse().map_err(|_| ModelLoadError::ParseError)?,
        z: tokens[2].parse().map_err(|_| ModelLoadError::ParseError)?,
    })
}

fn parse_vec2_line(tail: &str) -> Result<Vec2, ModelLoadError>
{
	let tokens: Vec<&str> = tail.split_whitespace().collect();
    if tokens.len() != 2
	{
        return Err(ModelLoadError::UnexpectedTokenCount);
    }

    Ok(Vec2 {
        x: tokens[0].parse().map_err(|_| ModelLoadError::ParseError)?,
        y: tokens[1].parse().map_err(|_| ModelLoadError::ParseError)?,
    })
}

fn parse_face_line(tail: &str) -> Result<Vec<Vec3>, ModelLoadError>
{
	let tokens: Vec<&str> = tail.split_whitespace().collect();
    if tokens.len() != 3 
	{
        return Err(ModelLoadError::UnexpectedTokenCount);
    }

	let mut out_face = vec![];

	for token in tokens
	{
		let indices: Vec<&str> = token.split('/').collect();
		if indices.len() != 3 
		{
			return Err(ModelLoadError::UnexpectedTokenCount);
		}

		out_face.push(
			Vec3 { 
				x: indices[0].parse().map_err(|_| ModelLoadError::ParseError)?, 
				y: indices[1].parse().map_err(|_| ModelLoadError::ParseError)?, 
				z: indices[2].parse().map_err(|_| ModelLoadError::ParseError)?
			}
		)
	}

	Ok(out_face)
}