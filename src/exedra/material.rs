use crate::vulkan::vk_bindgen::{
	VkImage,
	VkDeviceMemory, VkImageView, VkSampler,
};

#[derive(Clone, Debug)]
pub struct VulkanMaterialData
{
	pub texture_image: VkImage,
	pub texture_image_memory: VkDeviceMemory,
	pub texture_image_view: VkImageView,
	pub texture_sampler: VkSampler,
}

#[derive(Clone, Default, Debug)]
pub struct RenderingMaterial
{
	pub name: String,

	pub smooth_shading: bool,

	// ambient_factor: Vec3,
	// diffuse_factor: Vec3,
	// specular_factor: Vec3,
	// emissive_factor: Vec3,
	// specular_exponent_factor: f32,
	// refraction_index_factor: f32,
	// dissolve_factor: f32,
	// illumination_model: i32,
	
	// pub ambient_map_rel_path: Option<String>,
	pub diffuse_map_rel_path: String,
	// pub specular_map_rel_path: Option<String>,
	// pub emissive_map_rel_path: Option<String>,
	// pub bump_map_rel_path: Option<String>,

	pub vulkan_data: Option<VulkanMaterialData>,

	// pub diffuse_map: Option<(VkImage, VkDeviceMemory, VkImageView, VkSampler)>,
}

impl RenderingMaterial
{
	pub fn new(material_name: String) -> RenderingMaterial
	{
		RenderingMaterial { 
			name: material_name, 
			..Default::default()
		}
	}
}