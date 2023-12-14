use decs::component_derive::component;
use decs::component::Component;
use parmack::window::window_handle::WindowHandle;

use std::fmt::Debug;

use crate::cotangens::vec3::Vec3;

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

#[derive(Debug)]
pub enum RaycastObjectState
{
	Picked((usize, f32, Vec3)),
	Thrown((usize, f32, Vec3)),
	None,
}

#[component]
pub struct RaycastObject
{
	pub state: RaycastObjectState
}

#[component]
pub struct GlobalVariables
{
	pub should_run_physics: bool,
	pub focus_on_gui: bool,
}