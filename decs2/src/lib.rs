#![feature(downcast_unchecked)]
#![feature(duration_millis_float)]

pub mod component_derive {
	pub use component_derive::*;
}

pub mod decs;
pub use decs::typedef;
pub use decs::manager;
pub use decs::archetype;
pub use decs::query;
pub use decs::component_storage;
pub use decs::component_registry;
// pub use decs::util;