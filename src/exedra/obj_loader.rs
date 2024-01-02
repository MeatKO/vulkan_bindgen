use crate::cotangens::vecn::VecN;
use crate::cotangens::{
	vec2::*,
	vec3::*,
};

use super::material_descriptor::MaterialDescriptor;
use super::mesh_descriptor::MeshDescriptor;
use super::model_descriptor::ModelDescriptor;
use super::error::ModelLoadError;

use std::collections::HashMap;
use std::path::Path;

pub fn load_obj<P>(model_path: P) -> Result<ModelDescriptor, ModelLoadError>
where P : AsRef<Path>
{
	let obj_model_source = std::fs::read_to_string(model_path.as_ref())?;

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
			"f" => current_mesh.face_vtn_vec.extend(parse_face_line(tail)?), // only triangle faces for now !
			"mtllib" => { global_mtllib_vec.push(tail.to_owned()) }
			"usemtl" => 
			{
				if !current_mesh.face_vtn_vec.is_empty() && current_mesh.mtl_name != "".to_owned()
				{
					current_mesh.name = tail.to_owned();
					mesh_descriptor_vec.push(current_mesh);
				}

				current_mesh = MeshDescriptor { ..Default::default() };
				current_mesh.mtl_name = tail.to_owned();
			},
			// "s" => 
			// {
			// 	current_mesh.smooth_shading =
			// 		match tail
			// 		{
			// 			"1" | "on" => true,
			// 			_ => false,
			// 		}
			// }
			"o" | "g" => 
			{
				let old_mtl_name = current_mesh.mtl_name.clone();
				if !current_mesh.face_vtn_vec.is_empty()
				{
					current_mesh.name = tail.to_owned();
					mesh_descriptor_vec.push(current_mesh);
				}

				current_mesh = MeshDescriptor { ..Default::default() };
				current_mesh.mtl_name = old_mtl_name;
			}

			_ => { continue }
		}
	}
	if !current_mesh.face_vtn_vec.is_empty()
	{
		mesh_descriptor_vec.push(current_mesh)
	}

	let material_map = load_all_mtls(model_path.as_ref(), global_mtllib_vec)?;

	for mesh_descriptor in mesh_descriptor_vec.iter_mut()
	{
		// for face in mesh_descriptor.face_vec
		// {
		// 	// let new_vertex = 
		// 	// 	Vertex {
		// 	// 		pos: global_v_vec[face.x as usize - 1].clone(),
		// 	// 		uv: global_vt_vec[face.y as usize - 1].clone(),
		// 	// 		normal: global_vn_vec[face.z as usize - 1].clone(),
		// 	// 	};

		// 	// current_mesh.vertices.push(new_vertex);
		// 	// current_mesh.indices.push(current_mesh.indices.len() as _);
		// }

		mesh_descriptor.material = 
			material_map.get(&mesh_descriptor.mtl_name)
			.ok_or(ModelLoadError::MaterialNotFound(mesh_descriptor.mtl_name.clone()))?
			.clone();
	}

	let out_model = 
		ModelDescriptor {
			name: model_path.as_ref().file_name().and_then(|name| name.to_str()).unwrap_or("unnamed_model").to_string(),
			meshes: mesh_descriptor_vec,
			vertex_vec: global_v_vec,
			uv_vec: global_vt_vec,
			normal_vec: global_vn_vec,
		};

	Ok(out_model)
}

pub fn load_all_mtls<P>(model_path: P, material_file_names: Vec<String>) -> Result<HashMap<String, MaterialDescriptor>, ModelLoadError>
where P : AsRef<Path>
{
    let model_path_no_filename = model_path.as_ref().parent().unwrap_or(Path::new("./"));

	let mut out_material_hashmap: HashMap<String, MaterialDescriptor> = HashMap::new();

	for material_file_name in material_file_names
	{
		let current_material_path = model_path_no_filename.join(material_file_name.clone());

		let obj_material_source = 
			std::fs::read_to_string(current_material_path.clone())
			.map_err(|err| ModelLoadError::MaterialFileNotFound(current_material_path.clone().to_str().unwrap_or("unparseable_file_path").to_owned(), err))?;

		let mut current_material = MaterialDescriptor::new("".to_owned());

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
				"kd" | "map_kd" => current_material.albedo_path = model_path_no_filename.join(tail.to_owned()).to_str().unwrap_or("unparseable_file_path").to_owned(),
				"bump" | "map_bump" => current_material.normal_path = model_path_no_filename.join(tail.to_owned()).to_str().unwrap_or("unparseable_file_path").to_owned(),
				"newmtl" => 
				{
					if current_material.name != "".to_owned()
					{
						if current_material.albedo_path.trim().is_empty()
						{
							return Err(ModelLoadError::MaterialWithoutMaps(current_material.name.clone()))
						}

						out_material_hashmap.insert(current_material.name.clone(), current_material.clone());
					}

					current_material = MaterialDescriptor::new(tail.to_owned());
				}

				_ => { continue }
			}
		}

		out_material_hashmap.insert(current_material.name.clone(), current_material.clone());
	}

	Ok(out_material_hashmap)
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

// fn parse_face_line(tail: &str) -> Result<Vec<Vec3>, ModelLoadError>
fn parse_face_line(tail: &str) -> Result<Vec<VecN<usize, 3>>, ModelLoadError>
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
			VecN { 
				components: [
					indices[0].parse().map_err(|_| ModelLoadError::ParseError)?, 
					indices[1].parse().map_err(|_| ModelLoadError::ParseError)?, 
					indices[2].parse().map_err(|_| ModelLoadError::ParseError)?
				]
			}
		)
	}

	Ok(out_face)
}