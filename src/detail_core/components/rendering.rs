use decs::component_derive::component;
use decs::component::Component;

use std::fmt::Debug;
use std::rc::Weak;

use crate::detail_core::model::model::{Model, VulkanModel};

#[component]
pub struct ModelComponent
{
	pub model_asset: Weak<Model<VulkanModel>>
}