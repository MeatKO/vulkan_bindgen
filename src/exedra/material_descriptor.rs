#[derive(Clone, Default, Debug)]
pub struct MaterialDescriptor
{
	pub name: String,

	pub smooth_shading: bool,
	
	pub albedo_path: String,
	pub normal_path: String,
}

impl MaterialDescriptor
{
	pub fn new(material_name: String) -> MaterialDescriptor
	{
		MaterialDescriptor { 
			name: material_name, 
			..Default::default()
		}
	}
}