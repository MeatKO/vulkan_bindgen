use crate::cotangens::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RenderingMaterial
{
	name: String,

	ambient: Vec3,
	diffuse: Vec3,
	specular: Vec3,
	emissive: Vec3,
	specular_exponent: f32,
	refraction_index: f32,
	dissolve: f32,
	illumination_model: i32,
	
	ambient_map_path: Option<String>,
	diffuse_map_path: Option<String>,
	specular_map_path: Option<String>,
	emissive_map_path: Option<String>,
	bump_map_path: Option<String>,
}

impl RenderingMaterial
{
	pub fn new(material_name: String) -> RenderingMaterial
	{
		RenderingMaterial { 
			name: material_name, 
			dissolve: 1.0f32,
			..Default::default()
		}
	}
}