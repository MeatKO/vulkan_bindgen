use decs::component_derive::component;
use decs::component::Component;
use parmack::window::window_handle::WindowHandle;

use std::fmt::Debug;

#[component]
pub struct StringComponent
{
	pub string: String
}

#[component]
pub struct Float32Component
{
	pub float: f32
}

#[component]
pub struct Time
{
	pub time_stamp: std::time::Instant
}

#[derive(Clone)]
#[component]
pub struct DeltaTime
{
	pub last_delta_time: f32,
	pub last_time_stamp: std::time::Instant
}

#[component]
pub struct WindowComponent
{
	pub window: WindowHandle
}

#[component]
pub struct MainLoopComponent
{
	pub should_quit: bool
}