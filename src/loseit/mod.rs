// The opposite of "winit" or something... whatever
pub mod window;
pub mod window_events;

mod xcb_bindgen;
mod xcb_vk_bindgen;
mod xcb_functions;
mod xcb_window;
mod xcb_events;

mod win32_bindgen;
mod win32_vk_bindgen;
mod win32_window;
mod win32;

mod window_traits;