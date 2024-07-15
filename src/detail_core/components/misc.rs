use decs::component_derive::component;
use decs::component::Component;
use parmack::window::window_handle::WindowHandle;

use std::any::Any;
use std::collections::HashMap;
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
	pub last_delta_time_sec: f32,
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

#[derive(Debug, Clone)]
pub struct CameraRaycastInfo
{
	pub index: usize,
	pub length: f32,
	pub obj_relative_hit: Vec3,
}

#[derive(Debug, Clone)]
pub enum CameraRaycastObjectState
{
	Picked(CameraRaycastInfo),
	Thrown(CameraRaycastInfo),
	None,
}

#[component]
pub struct CameraRaycastObject
{
	pub state: CameraRaycastObjectState
}

#[component]
pub struct GlobalVariables
{
	pub should_run_physics: bool,
	pub focus_on_gui: bool,
	pub render_wireframe: bool,
	pub global_start_time: std::time::Instant,
	pub global_env_map: HashMap<String, Box::<dyn Any>>
}

impl GlobalVariables
{
	pub fn new() -> Self
	{
		return GlobalVariables {
			should_run_physics: false, 
			focus_on_gui: false, 
			render_wireframe: true,
			global_start_time: std::time::Instant::now(),
			global_env_map: HashMap::new()
		}
	}
}