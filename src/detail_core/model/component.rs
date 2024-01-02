use decs::component_derive::component;
use decs::component::Component;

#[component]
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