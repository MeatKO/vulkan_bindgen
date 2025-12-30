use decs2::component_derive::Component;
use decs2::typedef::Component;
use parmack::window::window_handle::WindowHandle;

use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::cotangens::vec3::Vec3;

#[derive(Component)]
pub struct StringComponent
{
	pub string: String
}

#[derive(Component)]
pub struct Float32Component
{
	pub float: f32
}

#[derive(Component)]
pub struct Time
{
	pub time_stamp: std::time::Instant
}

#[derive(Clone, Component)]
pub struct DeltaTime
{
	pub last_delta_time_sec: f32,
	pub last_time_stamp: std::time::Instant
}

#[derive(Component)]
pub struct WindowComponent
{
	pub window: WindowHandle
}

// #[derive(Component)]
// pub struct MainLoopComponent
// {
// 	pub should_quit: bool,
// 	pub delta_time: DeltaTime,
// }

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

#[derive(Component, Clone)]
pub struct CameraRaycastObject
{
	pub state: CameraRaycastObjectState
}

#[derive(Component)]
pub struct GlobalVariables
{
	pub should_run_physics: bool,
	pub focus_on_gui: bool,
	pub render_wireframe: bool,
	pub global_start_time: std::time::Instant,
	pub global_env_map: HashMap<String, Box::<dyn Any>>,
	pub global_raycast_object: CameraRaycastObject,

	pub should_quit: bool,
	pub delta_time: DeltaTime,
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
			global_env_map: HashMap::new(),
			global_raycast_object: CameraRaycastObject{ state: CameraRaycastObjectState::None },

			should_quit: false,
			delta_time: DeltaTime{ last_delta_time_sec: 0.0f32, last_time_stamp: std::time::Instant::now() },
		}
	}
}