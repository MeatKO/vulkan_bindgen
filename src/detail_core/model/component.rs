use decs2::component_derive::Component;
use decs2::typedef::Component;

#[derive(Component)]
pub struct VulkanModelComponent
{
	pub model_asset_name: String,
}

impl VulkanModelComponent
{
	pub fn new(model_asset_name: String, ) -> Self
	{
		Self
		{
			model_asset_name,
		}
	}
}