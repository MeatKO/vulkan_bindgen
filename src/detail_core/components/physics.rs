use decs::component_derive::component;
use decs::component::Component;

use std::fmt::Debug;
use std::rc::Weak;

use crate::detail_core::phys::aabb::AABB;

// #[component]
// pub struct AABBComponent
// {
// 	pub aabb_asset: Weak<AABB>
// }