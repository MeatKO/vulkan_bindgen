#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_connection_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_generic_iterator_t {
    pub data: *mut ::std::os::raw::c_void,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_generic_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_generic_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_generic_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_generic_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_generic_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_generic_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_generic_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
}
#[test]
fn bindgen_test_layout_xcb_generic_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_generic_reply_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_generic_reply_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_generic_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_generic_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_generic_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_reply_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_generic_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7usize],
    pub full_sequence: u32,
}
#[test]
fn bindgen_test_layout_xcb_generic_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_generic_event_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_generic_event_t>(),
        36usize,
        concat!("Size of: ", stringify!(xcb_generic_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_generic_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_generic_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_event_t),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).full_sequence) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_event_t),
            "::",
            stringify!(full_sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_raw_generic_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7usize],
}
#[test]
fn bindgen_test_layout_xcb_raw_generic_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_raw_generic_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_raw_generic_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_raw_generic_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_raw_generic_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_raw_generic_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_raw_generic_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_raw_generic_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_raw_generic_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_raw_generic_event_t),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ge_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad1: u16,
    pub pad: [u32; 5usize],
    pub full_sequence: u32,
}
#[test]
fn bindgen_test_layout_xcb_ge_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ge_event_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ge_event_t>(),
        36usize,
        concat!("Size of: ", stringify!(xcb_ge_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ge_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_ge_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_type) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(event_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).full_sequence) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_event_t),
            "::",
            stringify!(full_sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_generic_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5usize],
    pub full_sequence: u32,
}
#[test]
fn bindgen_test_layout_xcb_generic_error_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_generic_error_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_generic_error_t>(),
        36usize,
        concat!("Size of: ", stringify!(xcb_generic_error_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_generic_error_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_generic_error_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error_code) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(error_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resource_id) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(resource_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minor_code) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(minor_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_code) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(major_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).full_sequence) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_generic_error_t),
            "::",
            stringify!(full_sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_void_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_void_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_void_cookie_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_void_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_void_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_void_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_void_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_void_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_char2b_t {
    pub byte1: u8,
    pub byte2: u8,
}
#[test]
fn bindgen_test_layout_xcb_char2b_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_char2b_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_char2b_t>(),
        2usize,
        concat!("Size of: ", stringify!(xcb_char2b_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_char2b_t>(),
        1usize,
        concat!("Alignment of ", stringify!(xcb_char2b_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).byte1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_char2b_t),
            "::",
            stringify!(byte1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).byte2) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_char2b_t),
            "::",
            stringify!(byte2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_char2b_iterator_t {
    pub data: *mut xcb_char2b_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_char2b_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_char2b_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_char2b_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_char2b_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_char2b_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_char2b_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_char2b_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_char2b_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_char2b_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_window_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_window_iterator_t {
    pub data: *mut xcb_window_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_window_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_window_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_window_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_window_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_window_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_window_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_window_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_window_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_window_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_pixmap_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_pixmap_iterator_t {
    pub data: *mut xcb_pixmap_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_pixmap_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_pixmap_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_pixmap_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_pixmap_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_pixmap_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_pixmap_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_pixmap_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_pixmap_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_pixmap_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_cursor_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_cursor_iterator_t {
    pub data: *mut xcb_cursor_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_cursor_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_cursor_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_cursor_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_cursor_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_cursor_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_cursor_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_cursor_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_cursor_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_cursor_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_font_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_font_iterator_t {
    pub data: *mut xcb_font_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_font_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_font_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_font_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_font_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_font_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_font_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_font_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_font_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_font_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_gcontext_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_gcontext_iterator_t {
    pub data: *mut xcb_gcontext_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_gcontext_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_gcontext_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_gcontext_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_gcontext_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_gcontext_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_gcontext_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gcontext_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gcontext_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gcontext_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_colormap_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_colormap_iterator_t {
    pub data: *mut xcb_colormap_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_colormap_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_colormap_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_colormap_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_colormap_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_colormap_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_colormap_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_atom_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_atom_iterator_t {
    pub data: *mut xcb_atom_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_atom_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_atom_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_atom_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_atom_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_atom_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_atom_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_atom_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_atom_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_atom_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_drawable_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_drawable_iterator_t {
    pub data: *mut xcb_drawable_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_drawable_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_drawable_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_drawable_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_drawable_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_drawable_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_drawable_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_drawable_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_drawable_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_drawable_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_fontable_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fontable_iterator_t {
    pub data: *mut xcb_fontable_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_fontable_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_fontable_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_fontable_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_fontable_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_fontable_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_fontable_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontable_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontable_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontable_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_bool32_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_bool32_iterator_t {
    pub data: *mut xcb_bool32_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_bool32_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_bool32_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_bool32_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_bool32_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_bool32_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_bool32_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_bool32_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_bool32_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_bool32_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_visualid_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visualid_iterator_t {
    pub data: *mut xcb_visualid_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_visualid_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_visualid_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_visualid_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_visualid_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_visualid_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_visualid_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualid_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualid_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualid_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_timestamp_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_timestamp_iterator_t {
    pub data: *mut xcb_timestamp_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_timestamp_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_timestamp_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_timestamp_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_timestamp_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_timestamp_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_timestamp_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timestamp_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timestamp_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timestamp_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_keysym_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keysym_iterator_t {
    pub data: *mut xcb_keysym_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_keysym_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_keysym_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_keysym_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_keysym_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_keysym_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_keysym_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keysym_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keysym_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keysym_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_keycode_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keycode_iterator_t {
    pub data: *mut xcb_keycode_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_keycode_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_keycode_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_keycode_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_keycode_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_keycode_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_keycode_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keycode_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keycode_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keycode_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_keycode32_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keycode32_iterator_t {
    pub data: *mut xcb_keycode32_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_keycode32_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_keycode32_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_keycode32_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_keycode32_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_keycode32_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_keycode32_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keycode32_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keycode32_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keycode32_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub type xcb_button_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_button_iterator_t {
    pub data: *mut xcb_button_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_button_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_button_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_button_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_button_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_button_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_button_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_point_t {
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_point_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_point_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_point_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_point_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_point_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_point_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_point_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_point_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_point_iterator_t {
    pub data: *mut xcb_point_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_point_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_point_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_point_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_point_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_point_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_point_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_point_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_point_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_point_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rectangle_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_rectangle_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_rectangle_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_rectangle_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_rectangle_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_rectangle_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_rectangle_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_t),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rectangle_iterator_t {
    pub data: *mut xcb_rectangle_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_rectangle_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_rectangle_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_rectangle_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_rectangle_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_rectangle_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_rectangle_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rectangle_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_arc_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}
#[test]
fn bindgen_test_layout_xcb_arc_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_arc_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_arc_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_arc_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_arc_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_arc_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).angle1) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_t),
            "::",
            stringify!(angle1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).angle2) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_t),
            "::",
            stringify!(angle2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_arc_iterator_t {
    pub data: *mut xcb_arc_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_arc_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_arc_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_arc_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_arc_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_arc_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_arc_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_arc_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_format_t {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
    pub pad0: [u8; 5usize],
}
#[test]
fn bindgen_test_layout_xcb_format_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_format_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_format_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_format_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_format_t>(),
        1usize,
        concat!("Alignment of ", stringify!(xcb_format_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits_per_pixel) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_t),
            "::",
            stringify!(bits_per_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scanline_pad) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_t),
            "::",
            stringify!(scanline_pad)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_format_iterator_t {
    pub data: *mut xcb_format_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_format_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_format_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_format_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_format_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_format_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_format_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_format_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub const xcb_visual_class_t_XCB_VISUAL_CLASS_STATIC_GRAY: xcb_visual_class_t = 0;
pub const xcb_visual_class_t_XCB_VISUAL_CLASS_GRAY_SCALE: xcb_visual_class_t = 1;
pub const xcb_visual_class_t_XCB_VISUAL_CLASS_STATIC_COLOR: xcb_visual_class_t = 2;
pub const xcb_visual_class_t_XCB_VISUAL_CLASS_PSEUDO_COLOR: xcb_visual_class_t = 3;
pub const xcb_visual_class_t_XCB_VISUAL_CLASS_TRUE_COLOR: xcb_visual_class_t = 4;
pub const xcb_visual_class_t_XCB_VISUAL_CLASS_DIRECT_COLOR: xcb_visual_class_t = 5;
pub type xcb_visual_class_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_xcb_visualtype_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_visualtype_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_visualtype_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_visualtype_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_visualtype_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_visualtype_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(visual_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._class) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(_class)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits_per_rgb_value) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(bits_per_rgb_value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colormap_entries) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(colormap_entries)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).red_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(red_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).green_mask) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(green_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blue_mask) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(blue_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_visualtype_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_visualtype_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_visualtype_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_visualtype_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_visualtype_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_visualtype_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visualtype_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_depth_t {
    pub depth: u8,
    pub pad0: u8,
    pub visuals_len: u16,
    pub pad1: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_xcb_depth_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_depth_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_depth_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_depth_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_depth_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_depth_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visuals_len) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_t),
            "::",
            stringify!(visuals_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_depth_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_depth_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_depth_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_depth_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_depth_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_depth_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_depth_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub const xcb_event_mask_t_XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
pub const xcb_event_mask_t_XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const xcb_event_mask_t_XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const xcb_event_mask_t_XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const xcb_event_mask_t_XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const xcb_event_mask_t_XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const xcb_event_mask_t_XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const xcb_event_mask_t_XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const xcb_event_mask_t_XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const xcb_event_mask_t_XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const xcb_event_mask_t_XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const xcb_event_mask_t_XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const xcb_event_mask_t_XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const xcb_event_mask_t_XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const xcb_event_mask_t_XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const xcb_event_mask_t_XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const xcb_event_mask_t_XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const xcb_event_mask_t_XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub type xcb_event_mask_t = ::std::os::raw::c_uint;
pub const xcb_backing_store_t_XCB_BACKING_STORE_NOT_USEFUL: xcb_backing_store_t = 0;
pub const xcb_backing_store_t_XCB_BACKING_STORE_WHEN_MAPPED: xcb_backing_store_t = 1;
pub const xcb_backing_store_t_XCB_BACKING_STORE_ALWAYS: xcb_backing_store_t = 2;
pub type xcb_backing_store_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}
#[test]
fn bindgen_test_layout_xcb_screen_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_screen_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_screen_t>(),
        40usize,
        concat!("Size of: ", stringify!(xcb_screen_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_screen_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_screen_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).default_colormap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(default_colormap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).white_pixel) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(white_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).black_pixel) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(black_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).current_input_masks) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(current_input_masks)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width_in_pixels) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(width_in_pixels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height_in_pixels) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(height_in_pixels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width_in_millimeters) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(width_in_millimeters)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height_in_millimeters) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(height_in_millimeters)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_installed_maps) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(min_installed_maps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_installed_maps) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(max_installed_maps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_visual) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(root_visual)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_stores) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(backing_stores)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).save_unders) as usize - ptr as usize },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(save_unders)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_depth) as usize - ptr as usize },
        38usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(root_depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).allowed_depths_len) as usize - ptr as usize },
        39usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_t),
            "::",
            stringify!(allowed_depths_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_screen_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_screen_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_screen_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_screen_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_screen_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_screen_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_screen_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_request_t {
    pub byte_order: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name_len: u16,
    pub authorization_protocol_data_len: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_setup_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_request_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_setup_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_setup_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).byte_order) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(byte_order)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).protocol_major_version) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(protocol_major_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).protocol_minor_version) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(protocol_minor_version)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).authorization_protocol_name_len) as usize - ptr as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(authorization_protocol_name_len)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).authorization_protocol_data_len) as usize - ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(authorization_protocol_data_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_request_iterator_t {
    pub data: *mut xcb_setup_request_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_setup_request_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_request_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_request_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_setup_request_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_request_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_setup_request_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_request_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_failed_t {
    pub status: u8,
    pub reason_len: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_setup_failed_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_failed_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_failed_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_setup_failed_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_failed_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_setup_failed_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reason_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_t),
            "::",
            stringify!(reason_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).protocol_major_version) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_t),
            "::",
            stringify!(protocol_major_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).protocol_minor_version) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_t),
            "::",
            stringify!(protocol_minor_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_failed_iterator_t {
    pub data: *mut xcb_setup_failed_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_setup_failed_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_failed_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_failed_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_setup_failed_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_failed_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_setup_failed_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_failed_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_authenticate_t {
    pub status: u8,
    pub pad0: [u8; 5usize],
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_setup_authenticate_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_authenticate_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_authenticate_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_setup_authenticate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_authenticate_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_setup_authenticate_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_authenticate_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_authenticate_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_authenticate_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_authenticate_iterator_t {
    pub data: *mut xcb_setup_authenticate_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_setup_authenticate_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_authenticate_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_authenticate_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_setup_authenticate_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_authenticate_iterator_t>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_setup_authenticate_iterator_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_authenticate_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_authenticate_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_authenticate_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub const xcb_image_order_t_XCB_IMAGE_ORDER_LSB_FIRST: xcb_image_order_t = 0;
pub const xcb_image_order_t_XCB_IMAGE_ORDER_MSB_FIRST: xcb_image_order_t = 1;
pub type xcb_image_order_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_t {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_xcb_setup_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_t>(),
        40usize,
        concat!("Size of: ", stringify!(xcb_setup_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_setup_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).protocol_major_version) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(protocol_major_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).protocol_minor_version) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(protocol_minor_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).release_number) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(release_number)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resource_id_base) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(resource_id_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resource_id_mask) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(resource_id_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).motion_buffer_size) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(motion_buffer_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vendor_len) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(vendor_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maximum_request_length) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(maximum_request_length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).roots_len) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(roots_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixmap_formats_len) as usize - ptr as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(pixmap_formats_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).image_byte_order) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(image_byte_order)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bitmap_format_bit_order) as usize - ptr as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(bitmap_format_bit_order)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bitmap_format_scanline_unit) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(bitmap_format_scanline_unit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bitmap_format_scanline_pad) as usize - ptr as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(bitmap_format_scanline_pad)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_keycode) as usize - ptr as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(min_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_keycode) as usize - ptr as usize },
        35usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(max_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_setup_iterator_t {
    pub data: *mut xcb_setup_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_setup_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_setup_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_setup_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_setup_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_setup_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_setup_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_setup_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
pub const xcb_mod_mask_t_XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
pub const xcb_mod_mask_t_XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
pub const xcb_mod_mask_t_XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
pub const xcb_mod_mask_t_XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
pub const xcb_mod_mask_t_XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
pub const xcb_mod_mask_t_XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
pub const xcb_mod_mask_t_XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
pub const xcb_mod_mask_t_XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
pub const xcb_mod_mask_t_XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;
pub type xcb_mod_mask_t = ::std::os::raw::c_uint;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 2;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 4;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_MOD_1: xcb_key_but_mask_t = 8;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_MOD_2: xcb_key_but_mask_t = 16;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_MOD_3: xcb_key_but_mask_t = 32;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_MOD_4: xcb_key_but_mask_t = 64;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_MOD_5: xcb_key_but_mask_t = 128;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_BUTTON_1: xcb_key_but_mask_t = 256;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_BUTTON_2: xcb_key_but_mask_t = 512;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_BUTTON_3: xcb_key_but_mask_t = 1024;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_BUTTON_4: xcb_key_but_mask_t = 2048;
pub const xcb_key_but_mask_t_XCB_KEY_BUT_MASK_BUTTON_5: xcb_key_but_mask_t = 4096;
pub type xcb_key_but_mask_t = ::std::os::raw::c_uint;
pub const xcb_window_enum_t_XCB_WINDOW_NONE: xcb_window_enum_t = 0;
pub type xcb_window_enum_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_key_press_event_t {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}
#[test]
fn bindgen_test_layout_xcb_key_press_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_key_press_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_key_press_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_key_press_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_key_press_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_key_press_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).detail) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(detail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).child) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(root_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(root_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_x) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(event_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_y) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(event_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).same_screen) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(same_screen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_key_press_event_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub type xcb_key_release_event_t = xcb_key_press_event_t;
pub const xcb_button_mask_t_XCB_BUTTON_MASK_1: xcb_button_mask_t = 256;
pub const xcb_button_mask_t_XCB_BUTTON_MASK_2: xcb_button_mask_t = 512;
pub const xcb_button_mask_t_XCB_BUTTON_MASK_3: xcb_button_mask_t = 1024;
pub const xcb_button_mask_t_XCB_BUTTON_MASK_4: xcb_button_mask_t = 2048;
pub const xcb_button_mask_t_XCB_BUTTON_MASK_5: xcb_button_mask_t = 4096;
pub const xcb_button_mask_t_XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 32768;
pub type xcb_button_mask_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_button_press_event_t {
    pub response_type: u8,
    pub detail: xcb_button_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}
#[test]
fn bindgen_test_layout_xcb_button_press_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_button_press_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_button_press_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_button_press_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_button_press_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_button_press_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).detail) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(detail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).child) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(root_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(root_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_x) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(event_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_y) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(event_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).same_screen) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(same_screen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_button_press_event_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub type xcb_button_release_event_t = xcb_button_press_event_t;
pub const xcb_motion_t_XCB_MOTION_NORMAL: xcb_motion_t = 0;
pub const xcb_motion_t_XCB_MOTION_HINT: xcb_motion_t = 1;
pub type xcb_motion_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_motion_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}
#[test]
fn bindgen_test_layout_xcb_motion_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_motion_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_motion_notify_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_motion_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_motion_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_motion_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).detail) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(detail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).child) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(root_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(root_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_x) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(event_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_y) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(event_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).same_screen) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(same_screen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_motion_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_ANCESTOR: xcb_notify_detail_t = 0;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_VIRTUAL: xcb_notify_detail_t = 1;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_INFERIOR: xcb_notify_detail_t = 2;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_NONLINEAR: xcb_notify_detail_t = 3;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_notify_detail_t = 4;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_POINTER: xcb_notify_detail_t = 5;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_POINTER_ROOT: xcb_notify_detail_t = 6;
pub const xcb_notify_detail_t_XCB_NOTIFY_DETAIL_NONE: xcb_notify_detail_t = 7;
pub type xcb_notify_detail_t = ::std::os::raw::c_uint;
pub const xcb_notify_mode_t_XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;
pub const xcb_notify_mode_t_XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
pub const xcb_notify_mode_t_XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
pub const xcb_notify_mode_t_XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;
pub type xcb_notify_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_enter_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub mode: u8,
    pub same_screen_focus: u8,
}
#[test]
fn bindgen_test_layout_xcb_enter_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_enter_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_enter_notify_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_enter_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_enter_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_enter_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).detail) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(detail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).child) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(root_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(root_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_x) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(event_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_y) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(event_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).same_screen_focus) as usize - ptr as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_enter_notify_event_t),
            "::",
            stringify!(same_screen_focus)
        )
    );
}
pub type xcb_leave_notify_event_t = xcb_enter_notify_event_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_focus_in_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub mode: u8,
    pub pad0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_focus_in_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_focus_in_event_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_focus_in_event_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_focus_in_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_focus_in_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_focus_in_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_focus_in_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).detail) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_focus_in_event_t),
            "::",
            stringify!(detail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_focus_in_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_focus_in_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_focus_in_event_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_focus_in_event_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub type xcb_focus_out_event_t = xcb_focus_in_event_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_keymap_notify_event_t {
    pub response_type: u8,
    pub keys: [u8; 31usize],
}
#[test]
fn bindgen_test_layout_xcb_keymap_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_keymap_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_keymap_notify_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_keymap_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_keymap_notify_event_t>(),
        1usize,
        concat!("Alignment of ", stringify!(xcb_keymap_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keymap_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keys) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_keymap_notify_event_t),
            "::",
            stringify!(keys)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_expose_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_expose_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_expose_event_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_expose_event_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_expose_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_expose_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_expose_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_expose_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_graphics_exposure_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
    pub pad1: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_graphics_exposure_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_graphics_exposure_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_graphics_exposure_event_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_graphics_exposure_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_graphics_exposure_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_graphics_exposure_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minor_opcode) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(minor_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_graphics_exposure_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_no_exposure_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad1: u8,
}
#[test]
fn bindgen_test_layout_xcb_no_exposure_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_no_exposure_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_no_exposure_event_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_no_exposure_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_no_exposure_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_no_exposure_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minor_opcode) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(minor_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_exposure_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
pub const xcb_visibility_t_XCB_VISIBILITY_UNOBSCURED: xcb_visibility_t = 0;
pub const xcb_visibility_t_XCB_VISIBILITY_PARTIALLY_OBSCURED: xcb_visibility_t = 1;
pub const xcb_visibility_t_XCB_VISIBILITY_FULLY_OBSCURED: xcb_visibility_t = 2;
pub type xcb_visibility_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_visibility_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub state: u8,
    pub pad1: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_visibility_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_visibility_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_visibility_notify_event_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_visibility_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_visibility_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_visibility_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visibility_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visibility_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visibility_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visibility_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visibility_notify_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_visibility_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}
#[test]
fn bindgen_test_layout_xcb_create_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_notify_event_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_create_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_width) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(border_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_destroy_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_destroy_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_destroy_notify_event_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_destroy_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_destroy_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_destroy_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_notify_event_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: u8,
    pub pad1: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_unmap_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_unmap_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_unmap_notify_event_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_unmap_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_unmap_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_unmap_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).from_configure) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(from_configure)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: u8,
    pub pad1: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_map_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_map_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_map_notify_event_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_map_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_map_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_map_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_map_request_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_map_request_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_map_request_event_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_map_request_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_map_request_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_map_request_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_request_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_request_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_request_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_request_event_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_request_event_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_reparent_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub override_redirect: u8,
    pub pad1: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_reparent_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_reparent_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_reparent_notify_event_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_reparent_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_reparent_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_reparent_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}
#[test]
fn bindgen_test_layout_xcb_configure_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_configure_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_configure_notify_event_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_configure_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_configure_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_configure_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).above_sibling) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(above_sibling)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_width) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(border_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_request_event_t {
    pub response_type: u8,
    pub stack_mode: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub value_mask: u16,
}
#[test]
fn bindgen_test_layout_xcb_configure_request_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_configure_request_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_configure_request_event_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_configure_request_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_configure_request_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_configure_request_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stack_mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(stack_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sibling) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(sibling)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_width) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(border_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_request_event_t),
            "::",
            stringify!(value_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_gravity_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_gravity_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_gravity_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_gravity_notify_event_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_gravity_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_gravity_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_gravity_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_gravity_notify_event_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_resize_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_resize_request_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_resize_request_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_resize_request_event_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_resize_request_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_resize_request_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_resize_request_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_resize_request_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_resize_request_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_resize_request_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_resize_request_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_resize_request_event_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_resize_request_event_t),
            "::",
            stringify!(height)
        )
    );
}
pub const xcb_place_t_XCB_PLACE_ON_TOP: xcb_place_t = 0;
pub const xcb_place_t_XCB_PLACE_ON_BOTTOM: xcb_place_t = 1;
pub type xcb_place_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_circulate_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub pad1: [u8; 4usize],
    pub place: u8,
    pub pad2: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_circulate_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_circulate_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_circulate_notify_event_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_circulate_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_circulate_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_circulate_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).place) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(place)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad2) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_notify_event_t),
            "::",
            stringify!(pad2)
        )
    );
}
pub type xcb_circulate_request_event_t = xcb_circulate_notify_event_t;
pub const xcb_property_t_XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
pub const xcb_property_t_XCB_PROPERTY_DELETE: xcb_property_t = 1;
pub type xcb_property_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_property_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: u8,
    pub pad1: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_property_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_property_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_property_notify_event_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_property_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_property_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_property_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atom) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(atom)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_property_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selection_clear_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_selection_clear_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_selection_clear_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_selection_clear_event_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_selection_clear_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_selection_clear_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_selection_clear_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_clear_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_clear_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_clear_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_clear_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_clear_event_t),
            "::",
            stringify!(owner)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).selection) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_clear_event_t),
            "::",
            stringify!(selection)
        )
    );
}
pub const xcb_time_t_XCB_TIME_CURRENT_TIME: xcb_time_t = 0;
pub type xcb_time_t = ::std::os::raw::c_uint;
pub const xcb_atom_enum_t_XCB_ATOM_NONE: xcb_atom_enum_t = 0;
pub const xcb_atom_enum_t_XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const xcb_atom_enum_t_XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const xcb_atom_enum_t_XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const xcb_atom_enum_t_XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const xcb_atom_enum_t_XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const xcb_atom_enum_t_XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const xcb_atom_enum_t_XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const xcb_atom_enum_t_XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const xcb_atom_enum_t_XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const xcb_atom_enum_t_XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const xcb_atom_enum_t_XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const xcb_atom_enum_t_XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const xcb_atom_enum_t_XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const xcb_atom_enum_t_XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const xcb_atom_enum_t_XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const xcb_atom_enum_t_XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const xcb_atom_enum_t_XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const xcb_atom_enum_t_XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const xcb_atom_enum_t_XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const xcb_atom_enum_t_XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const xcb_atom_enum_t_XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const xcb_atom_enum_t_XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const xcb_atom_enum_t_XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const xcb_atom_enum_t_XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const xcb_atom_enum_t_XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const xcb_atom_enum_t_XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const xcb_atom_enum_t_XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const xcb_atom_enum_t_XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const xcb_atom_enum_t_XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const xcb_atom_enum_t_XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const xcb_atom_enum_t_XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const xcb_atom_enum_t_XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const xcb_atom_enum_t_XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const xcb_atom_enum_t_XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const xcb_atom_enum_t_XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const xcb_atom_enum_t_XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const xcb_atom_enum_t_XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const xcb_atom_enum_t_XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const xcb_atom_enum_t_XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const xcb_atom_enum_t_XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const xcb_atom_enum_t_XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const xcb_atom_enum_t_XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const xcb_atom_enum_t_XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const xcb_atom_enum_t_XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const xcb_atom_enum_t_XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const xcb_atom_enum_t_XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const xcb_atom_enum_t_XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const xcb_atom_enum_t_XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const xcb_atom_enum_t_XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const xcb_atom_enum_t_XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const xcb_atom_enum_t_XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const xcb_atom_enum_t_XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const xcb_atom_enum_t_XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const xcb_atom_enum_t_XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const xcb_atom_enum_t_XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const xcb_atom_enum_t_XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub type xcb_atom_enum_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selection_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_selection_request_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_selection_request_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_selection_request_event_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_selection_request_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_selection_request_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_selection_request_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(owner)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).requestor) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(requestor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).selection) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(selection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).property) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_request_event_t),
            "::",
            stringify!(property)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_selection_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_selection_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_selection_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_selection_notify_event_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_selection_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_selection_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_selection_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).requestor) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(requestor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).selection) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(selection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).property) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_selection_notify_event_t),
            "::",
            stringify!(property)
        )
    );
}
pub const xcb_colormap_state_t_XCB_COLORMAP_STATE_UNINSTALLED: xcb_colormap_state_t = 0;
pub const xcb_colormap_state_t_XCB_COLORMAP_STATE_INSTALLED: xcb_colormap_state_t = 1;
pub type xcb_colormap_state_t = ::std::os::raw::c_uint;
pub const xcb_colormap_enum_t_XCB_COLORMAP_NONE: xcb_colormap_enum_t = 0;
pub type xcb_colormap_enum_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_colormap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub colormap: xcb_colormap_t,
    pub _new: u8,
    pub state: u8,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_colormap_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_colormap_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_colormap_notify_event_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_colormap_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_colormap_notify_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_colormap_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colormap) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(colormap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._new) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(_new)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_colormap_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union xcb_client_message_data_t {
    pub data8: [u8; 20usize],
    pub data16: [u16; 10usize],
    pub data32: [u32; 5usize],
}
#[test]
fn bindgen_test_layout_xcb_client_message_data_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_client_message_data_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_client_message_data_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_client_message_data_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_client_message_data_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_client_message_data_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data8) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_data_t),
            "::",
            stringify!(data8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data16) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_data_t),
            "::",
            stringify!(data16)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data32) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_data_t),
            "::",
            stringify!(data32)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_client_message_data_iterator_t {
    pub data: *mut xcb_client_message_data_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_client_message_data_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_client_message_data_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_client_message_data_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_client_message_data_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_client_message_data_iterator_t>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_client_message_data_iterator_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_data_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_data_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_data_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcb_client_message_event_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub type_: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
#[test]
fn bindgen_test_layout_xcb_client_message_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_client_message_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_client_message_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_client_message_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_client_message_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_client_message_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_event_t),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_event_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_event_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_client_message_event_t),
            "::",
            stringify!(data)
        )
    );
}
pub const xcb_mapping_t_XCB_MAPPING_MODIFIER: xcb_mapping_t = 0;
pub const xcb_mapping_t_XCB_MAPPING_KEYBOARD: xcb_mapping_t = 1;
pub const xcb_mapping_t_XCB_MAPPING_POINTER: xcb_mapping_t = 2;
pub type xcb_mapping_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_mapping_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub request: u8,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
    pub pad1: u8,
}
#[test]
fn bindgen_test_layout_xcb_mapping_notify_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_mapping_notify_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_mapping_notify_event_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_mapping_notify_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_mapping_notify_event_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_mapping_notify_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).request) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(request)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).first_keycode) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(first_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_mapping_notify_event_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ge_generic_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 22usize],
    pub full_sequence: u32,
}
#[test]
fn bindgen_test_layout_xcb_ge_generic_event_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ge_generic_event_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ge_generic_event_t>(),
        36usize,
        concat!("Size of: ", stringify!(xcb_ge_generic_event_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ge_generic_event_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_ge_generic_event_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extension) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(extension)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_type) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(event_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).full_sequence) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ge_generic_event_t),
            "::",
            stringify!(full_sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_request_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: u8,
}
#[test]
fn bindgen_test_layout_xcb_request_error_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_request_error_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_request_error_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_request_error_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_request_error_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_request_error_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error_code) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(error_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bad_value) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(bad_value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minor_opcode) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(minor_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_request_error_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_value_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: u8,
}
#[test]
fn bindgen_test_layout_xcb_value_error_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_value_error_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_value_error_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_value_error_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_value_error_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_value_error_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error_code) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(error_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bad_value) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(bad_value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minor_opcode) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(minor_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_value_error_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub type xcb_window_error_t = xcb_value_error_t;
pub type xcb_pixmap_error_t = xcb_value_error_t;
pub type xcb_atom_error_t = xcb_value_error_t;
pub type xcb_cursor_error_t = xcb_value_error_t;
pub type xcb_font_error_t = xcb_value_error_t;
pub type xcb_match_error_t = xcb_request_error_t;
pub type xcb_drawable_error_t = xcb_value_error_t;
pub type xcb_access_error_t = xcb_request_error_t;
pub type xcb_alloc_error_t = xcb_request_error_t;
pub type xcb_colormap_error_t = xcb_value_error_t;
pub type xcb_g_context_error_t = xcb_value_error_t;
pub type xcb_id_choice_error_t = xcb_value_error_t;
pub type xcb_name_error_t = xcb_request_error_t;
pub type xcb_length_error_t = xcb_request_error_t;
pub type xcb_implementation_error_t = xcb_request_error_t;
pub const xcb_window_class_t_XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub const xcb_window_class_t_XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const xcb_window_class_t_XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub type xcb_window_class_t = ::std::os::raw::c_uint;
pub const xcb_cw_t_XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
pub const xcb_cw_t_XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const xcb_cw_t_XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const xcb_cw_t_XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const xcb_cw_t_XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const xcb_cw_t_XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const xcb_cw_t_XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const xcb_cw_t_XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const xcb_cw_t_XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const xcb_cw_t_XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const xcb_cw_t_XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const xcb_cw_t_XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const xcb_cw_t_XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const xcb_cw_t_XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const xcb_cw_t_XCB_CW_CURSOR: xcb_cw_t = 16384;
pub type xcb_cw_t = ::std::os::raw::c_uint;
pub const xcb_back_pixmap_t_XCB_BACK_PIXMAP_NONE: xcb_back_pixmap_t = 0;
pub const xcb_back_pixmap_t_XCB_BACK_PIXMAP_PARENT_RELATIVE: xcb_back_pixmap_t = 1;
pub type xcb_back_pixmap_t = ::std::os::raw::c_uint;
pub const xcb_gravity_t_XCB_GRAVITY_BIT_FORGET: xcb_gravity_t = 0;
pub const xcb_gravity_t_XCB_GRAVITY_WIN_UNMAP: xcb_gravity_t = 0;
pub const xcb_gravity_t_XCB_GRAVITY_NORTH_WEST: xcb_gravity_t = 1;
pub const xcb_gravity_t_XCB_GRAVITY_NORTH: xcb_gravity_t = 2;
pub const xcb_gravity_t_XCB_GRAVITY_NORTH_EAST: xcb_gravity_t = 3;
pub const xcb_gravity_t_XCB_GRAVITY_WEST: xcb_gravity_t = 4;
pub const xcb_gravity_t_XCB_GRAVITY_CENTER: xcb_gravity_t = 5;
pub const xcb_gravity_t_XCB_GRAVITY_EAST: xcb_gravity_t = 6;
pub const xcb_gravity_t_XCB_GRAVITY_SOUTH_WEST: xcb_gravity_t = 7;
pub const xcb_gravity_t_XCB_GRAVITY_SOUTH: xcb_gravity_t = 8;
pub const xcb_gravity_t_XCB_GRAVITY_SOUTH_EAST: xcb_gravity_t = 9;
pub const xcb_gravity_t_XCB_GRAVITY_STATIC: xcb_gravity_t = 10;
pub type xcb_gravity_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_window_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}
#[test]
fn bindgen_test_layout_xcb_create_window_value_list_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_window_value_list_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_window_value_list_t>(),
        60usize,
        concat!("Size of: ", stringify!(xcb_create_window_value_list_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_window_value_list_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_window_value_list_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).background_pixmap) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(background_pixmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).background_pixel) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(background_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_pixmap) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(border_pixmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_pixel) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(border_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bit_gravity) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(bit_gravity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).win_gravity) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(win_gravity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_store) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(backing_store)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_planes) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(backing_planes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_pixel) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(backing_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).save_under) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(save_under)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mask) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).do_not_propogate_mask) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(do_not_propogate_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colormap) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(colormap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_value_list_t),
            "::",
            stringify!(cursor)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_window_request_t {
    pub major_opcode: u8,
    pub depth: u8,
    pub length: u16,
    pub wid: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub _class: u16,
    pub visual: xcb_visualid_t,
    pub value_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_create_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_window_request_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_create_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(wid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_width) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(border_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._class) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(_class)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(visual)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_window_request_t),
            "::",
            stringify!(value_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_window_attributes_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}
#[test]
fn bindgen_test_layout_xcb_change_window_attributes_value_list_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_window_attributes_value_list_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_window_attributes_value_list_t>(),
        60usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_window_attributes_value_list_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_window_attributes_value_list_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_window_attributes_value_list_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).background_pixmap) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(background_pixmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).background_pixel) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(background_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_pixmap) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(border_pixmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_pixel) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(border_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bit_gravity) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(bit_gravity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).win_gravity) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(win_gravity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_store) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(backing_store)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_planes) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(backing_planes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_pixel) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(backing_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).save_under) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(save_under)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mask) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).do_not_propogate_mask) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(do_not_propogate_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colormap) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(colormap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_value_list_t),
            "::",
            stringify!(cursor)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_window_attributes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub value_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_change_window_attributes_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_window_attributes_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_window_attributes_request_t>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_window_attributes_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_window_attributes_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_window_attributes_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_window_attributes_request_t),
            "::",
            stringify!(value_mask)
        )
    );
}
pub const xcb_map_state_t_XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
pub const xcb_map_state_t_XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
pub const xcb_map_state_t_XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;
pub type xcb_map_state_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_window_attributes_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_window_attributes_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_window_attributes_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_window_attributes_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_window_attributes_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_window_attributes_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_window_attributes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_get_window_attributes_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_window_attributes_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_window_attributes_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_get_window_attributes_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_window_attributes_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_window_attributes_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: u8,
    pub backing_store: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub _class: u16,
    pub bit_gravity: u8,
    pub win_gravity: u8,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: u8,
    pub map_is_installed: u8,
    pub map_state: u8,
    pub override_redirect: u8,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: u32,
    pub your_event_mask: u32,
    pub do_not_propagate_mask: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_get_window_attributes_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_window_attributes_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_window_attributes_reply_t>(),
        44usize,
        concat!("Size of: ", stringify!(xcb_get_window_attributes_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_window_attributes_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_window_attributes_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_store) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(backing_store)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(visual)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._class) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(_class)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bit_gravity) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(bit_gravity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).win_gravity) as usize - ptr as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(win_gravity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_planes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(backing_planes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).backing_pixel) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(backing_pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).save_under) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(save_under)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).map_is_installed) as usize - ptr as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(map_is_installed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).map_state) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(map_state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).override_redirect) as usize - ptr as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(override_redirect)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colormap) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(colormap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).all_event_masks) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(all_event_masks)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).your_event_mask) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(your_event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).do_not_propagate_mask) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(do_not_propagate_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_window_attributes_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_destroy_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_destroy_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_destroy_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_destroy_window_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_destroy_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_destroy_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_destroy_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_window_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_window_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_destroy_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_destroy_subwindows_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_destroy_subwindows_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_destroy_subwindows_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_destroy_subwindows_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_destroy_subwindows_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_destroy_subwindows_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_subwindows_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_subwindows_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_subwindows_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_destroy_subwindows_request_t),
            "::",
            stringify!(window)
        )
    );
}
pub const xcb_set_mode_t_XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
pub const xcb_set_mode_t_XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;
pub type xcb_set_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_save_set_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_change_save_set_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_save_set_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_save_set_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_change_save_set_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_save_set_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_change_save_set_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_save_set_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_save_set_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_save_set_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_save_set_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_reparent_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_reparent_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_reparent_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_reparent_window_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_reparent_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_reparent_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_reparent_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_reparent_window_request_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_map_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_map_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_map_window_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_map_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_map_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_map_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_window_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_window_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_map_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_map_subwindows_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_map_subwindows_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_map_subwindows_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_map_subwindows_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_map_subwindows_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_map_subwindows_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_subwindows_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_subwindows_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_subwindows_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_map_subwindows_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_unmap_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_unmap_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_unmap_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_unmap_window_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_unmap_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_unmap_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_unmap_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_window_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_window_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_unmap_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_unmap_subwindows_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_unmap_subwindows_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_unmap_subwindows_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_unmap_subwindows_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_unmap_subwindows_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_unmap_subwindows_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_subwindows_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_subwindows_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_subwindows_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_unmap_subwindows_request_t),
            "::",
            stringify!(window)
        )
    );
}
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const xcb_config_window_t_XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;
pub type xcb_config_window_t = ::std::os::raw::c_uint;
pub const xcb_stack_mode_t_XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
pub const xcb_stack_mode_t_XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const xcb_stack_mode_t_XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const xcb_stack_mode_t_XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const xcb_stack_mode_t_XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;
pub type xcb_stack_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_window_value_list_t {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub sibling: xcb_window_t,
    pub stack_mode: u32,
}
#[test]
fn bindgen_test_layout_xcb_configure_window_value_list_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_configure_window_value_list_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_configure_window_value_list_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_configure_window_value_list_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_configure_window_value_list_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_configure_window_value_list_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(border_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sibling) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(sibling)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stack_mode) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_value_list_t),
            "::",
            stringify!(stack_mode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_configure_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub value_mask: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_configure_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_configure_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_configure_window_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_configure_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_configure_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_configure_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_request_t),
            "::",
            stringify!(value_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_configure_window_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
pub const xcb_circulate_t_XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
pub const xcb_circulate_t_XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;
pub type xcb_circulate_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_circulate_window_request_t {
    pub major_opcode: u8,
    pub direction: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_circulate_window_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_circulate_window_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_circulate_window_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_circulate_window_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_circulate_window_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_circulate_window_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_window_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).direction) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_window_request_t),
            "::",
            stringify!(direction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_window_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_circulate_window_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_geometry_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_geometry_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_geometry_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_geometry_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_geometry_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_geometry_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}
#[test]
fn bindgen_test_layout_xcb_get_geometry_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_geometry_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_geometry_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_get_geometry_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_geometry_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_geometry_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_request_t),
            "::",
            stringify!(drawable)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_get_geometry_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_geometry_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_geometry_reply_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_get_geometry_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_geometry_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_geometry_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).border_width) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(border_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_geometry_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_tree_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_tree_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_tree_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_tree_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_tree_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_tree_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_tree_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_tree_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_query_tree_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_tree_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_tree_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_query_tree_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_tree_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_tree_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_tree_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: u16,
    pub pad1: [u8; 14usize],
}
#[test]
fn bindgen_test_layout_xcb_query_tree_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_tree_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_tree_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_query_tree_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_tree_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_tree_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children_len) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(children_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_tree_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_intern_atom_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_intern_atom_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_intern_atom_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_intern_atom_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_intern_atom_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_intern_atom_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_request_t {
    pub major_opcode: u8,
    pub only_if_exists: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_intern_atom_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_intern_atom_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_intern_atom_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_intern_atom_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_intern_atom_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_intern_atom_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).only_if_exists) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_request_t),
            "::",
            stringify!(only_if_exists)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_request_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_intern_atom_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_intern_atom_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_intern_atom_reply_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_intern_atom_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_intern_atom_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_intern_atom_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atom) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_intern_atom_reply_t),
            "::",
            stringify!(atom)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_atom_name_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_atom_name_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_atom_name_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_atom_name_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_atom_name_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_atom_name_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_atom_name_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub atom: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_get_atom_name_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_atom_name_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_atom_name_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_get_atom_name_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_atom_name_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_atom_name_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atom) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_request_t),
            "::",
            stringify!(atom)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub name_len: u16,
    pub pad1: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_get_atom_name_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_atom_name_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_atom_name_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_atom_name_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_atom_name_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_atom_name_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_reply_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_atom_name_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
pub const xcb_prop_mode_t_XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
pub const xcb_prop_mode_t_XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const xcb_prop_mode_t_XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub type xcb_prop_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_property_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub pad0: [u8; 3usize],
    pub data_len: u32,
}
#[test]
fn bindgen_test_layout_xcb_change_property_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_property_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_property_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_change_property_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_property_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_change_property_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).property) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data_len) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_property_request_t),
            "::",
            stringify!(data_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_delete_property_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_delete_property_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_delete_property_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_delete_property_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_delete_property_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_delete_property_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_delete_property_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_delete_property_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_delete_property_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_delete_property_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_delete_property_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).property) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_delete_property_request_t),
            "::",
            stringify!(property)
        )
    );
}
pub const xcb_get_property_type_t_XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;
pub type xcb_get_property_type_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_property_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_property_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_property_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_property_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_property_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_property_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_property_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_property_request_t {
    pub major_opcode: u8,
    pub _delete: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
}
#[test]
fn bindgen_test_layout_xcb_get_property_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_property_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_property_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_get_property_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_property_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_property_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._delete) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(_delete)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).property) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).long_offset) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(long_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).long_length) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_request_t),
            "::",
            stringify!(long_length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub value_len: u32,
    pub pad0: [u8; 12usize],
}
#[test]
fn bindgen_test_layout_xcb_get_property_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_property_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_property_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_property_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_property_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_property_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bytes_after) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(bytes_after)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_len) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(value_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_property_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_properties_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_list_properties_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_properties_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_properties_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_properties_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_properties_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_properties_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_properties_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_list_properties_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_properties_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_properties_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_list_properties_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_properties_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_properties_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms_len: u16,
    pub pad1: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_list_properties_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_properties_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_properties_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_list_properties_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_properties_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_properties_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atoms_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_reply_t),
            "::",
            stringify!(atoms_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_properties_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_selection_owner_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_set_selection_owner_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_selection_owner_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_selection_owner_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_set_selection_owner_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_selection_owner_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_selection_owner_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_selection_owner_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_selection_owner_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_selection_owner_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_selection_owner_request_t),
            "::",
            stringify!(owner)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).selection) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_selection_owner_request_t),
            "::",
            stringify!(selection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_selection_owner_request_t),
            "::",
            stringify!(time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_selection_owner_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_selection_owner_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_selection_owner_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_selection_owner_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_selection_owner_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_selection_owner_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_selection_owner_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_selection_owner_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}
#[test]
fn bindgen_test_layout_xcb_get_selection_owner_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_selection_owner_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_selection_owner_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_get_selection_owner_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_selection_owner_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_selection_owner_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).selection) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_request_t),
            "::",
            stringify!(selection)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_get_selection_owner_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_selection_owner_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_selection_owner_reply_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_get_selection_owner_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_selection_owner_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_selection_owner_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_selection_owner_reply_t),
            "::",
            stringify!(owner)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_convert_selection_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_convert_selection_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_convert_selection_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_convert_selection_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_convert_selection_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_convert_selection_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_convert_selection_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).requestor) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(requestor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).selection) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(selection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).target) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).property) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_convert_selection_request_t),
            "::",
            stringify!(time)
        )
    );
}
pub const xcb_send_event_dest_t_XCB_SEND_EVENT_DEST_POINTER_WINDOW: xcb_send_event_dest_t = 0;
pub const xcb_send_event_dest_t_XCB_SEND_EVENT_DEST_ITEM_FOCUS: xcb_send_event_dest_t = 1;
pub type xcb_send_event_dest_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_send_event_request_t {
    pub major_opcode: u8,
    pub propagate: u8,
    pub length: u16,
    pub destination: xcb_window_t,
    pub event_mask: u32,
    pub event: [::std::os::raw::c_char; 32usize],
}
#[test]
fn bindgen_test_layout_xcb_send_event_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_send_event_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_send_event_request_t>(),
        44usize,
        concat!("Size of: ", stringify!(xcb_send_event_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_send_event_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_send_event_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_send_event_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).propagate) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_send_event_request_t),
            "::",
            stringify!(propagate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_send_event_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).destination) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_send_event_request_t),
            "::",
            stringify!(destination)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_send_event_request_t),
            "::",
            stringify!(event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_send_event_request_t),
            "::",
            stringify!(event)
        )
    );
}
pub const xcb_grab_mode_t_XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
pub const xcb_grab_mode_t_XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;
pub type xcb_grab_mode_t = ::std::os::raw::c_uint;
pub const xcb_grab_status_t_XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
pub const xcb_grab_status_t_XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
pub const xcb_grab_status_t_XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
pub const xcb_grab_status_t_XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
pub const xcb_grab_status_t_XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;
pub type xcb_grab_status_t = ::std::os::raw::c_uint;
pub const xcb_cursor_enum_t_XCB_CURSOR_NONE: xcb_cursor_enum_t = 0;
pub type xcb_cursor_enum_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_grab_pointer_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_pointer_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_pointer_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_grab_pointer_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_pointer_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_pointer_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub event_mask: u16,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_grab_pointer_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_pointer_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_pointer_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_grab_pointer_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_pointer_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_pointer_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner_events) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(owner_events)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grab_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(grab_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_mode) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(pointer_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keyboard_mode) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(keyboard_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).confine_to) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(confine_to)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(cursor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_request_t),
            "::",
            stringify!(time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}
#[test]
fn bindgen_test_layout_xcb_grab_pointer_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_pointer_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_pointer_reply_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_grab_pointer_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_pointer_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_pointer_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_reply_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_pointer_reply_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_ungrab_pointer_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ungrab_pointer_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ungrab_pointer_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_ungrab_pointer_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ungrab_pointer_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_ungrab_pointer_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_pointer_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_pointer_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_pointer_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_pointer_request_t),
            "::",
            stringify!(time)
        )
    );
}
pub const xcb_button_index_t_XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
pub const xcb_button_index_t_XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const xcb_button_index_t_XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const xcb_button_index_t_XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const xcb_button_index_t_XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const xcb_button_index_t_XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;
pub type xcb_button_index_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_button_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub event_mask: u16,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub button: u8,
    pub pad0: u8,
    pub modifiers: u16,
}
#[test]
fn bindgen_test_layout_xcb_grab_button_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_button_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_button_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_grab_button_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_button_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_button_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner_events) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(owner_events)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grab_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(grab_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_mode) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(pointer_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keyboard_mode) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(keyboard_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).confine_to) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(confine_to)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(cursor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).button) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(button)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).modifiers) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_button_request_t),
            "::",
            stringify!(modifiers)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_button_request_t {
    pub major_opcode: u8,
    pub button: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_ungrab_button_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ungrab_button_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ungrab_button_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_ungrab_button_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ungrab_button_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_ungrab_button_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_button_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).button) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_button_request_t),
            "::",
            stringify!(button)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_button_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grab_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_button_request_t),
            "::",
            stringify!(grab_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).modifiers) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_button_request_t),
            "::",
            stringify!(modifiers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_button_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_active_pointer_grab_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
    pub event_mask: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_change_active_pointer_grab_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_active_pointer_grab_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_active_pointer_grab_request_t>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_active_pointer_grab_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_active_pointer_grab_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_active_pointer_grab_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(cursor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_mask) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(event_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_active_pointer_grab_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_keyboard_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_grab_keyboard_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_keyboard_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_keyboard_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_grab_keyboard_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_keyboard_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_keyboard_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_keyboard_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_grab_keyboard_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_keyboard_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_keyboard_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_grab_keyboard_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_keyboard_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_keyboard_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner_events) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(owner_events)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grab_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(grab_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_mode) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(pointer_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keyboard_mode) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(keyboard_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}
#[test]
fn bindgen_test_layout_xcb_grab_keyboard_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_keyboard_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_keyboard_reply_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_grab_keyboard_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_keyboard_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_keyboard_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_reply_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_keyboard_reply_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_keyboard_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_ungrab_keyboard_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ungrab_keyboard_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ungrab_keyboard_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_ungrab_keyboard_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ungrab_keyboard_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_ungrab_keyboard_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_keyboard_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_keyboard_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_keyboard_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_keyboard_request_t),
            "::",
            stringify!(time)
        )
    );
}
pub const xcb_grab_t_XCB_GRAB_ANY: xcb_grab_t = 0;
pub type xcb_grab_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_key_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub key: xcb_keycode_t,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub pad0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_xcb_grab_key_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_key_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_key_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_grab_key_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_key_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_grab_key_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner_events) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(owner_events)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grab_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(grab_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).modifiers) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(modifiers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_mode) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(pointer_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keyboard_mode) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(keyboard_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_key_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_key_request_t {
    pub major_opcode: u8,
    pub key: xcb_keycode_t,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_ungrab_key_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ungrab_key_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ungrab_key_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_ungrab_key_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ungrab_key_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_ungrab_key_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_key_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_key_request_t),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_key_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).grab_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_key_request_t),
            "::",
            stringify!(grab_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).modifiers) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_key_request_t),
            "::",
            stringify!(modifiers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_key_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub const xcb_allow_t_XCB_ALLOW_ASYNC_POINTER: xcb_allow_t = 0;
pub const xcb_allow_t_XCB_ALLOW_SYNC_POINTER: xcb_allow_t = 1;
pub const xcb_allow_t_XCB_ALLOW_REPLAY_POINTER: xcb_allow_t = 2;
pub const xcb_allow_t_XCB_ALLOW_ASYNC_KEYBOARD: xcb_allow_t = 3;
pub const xcb_allow_t_XCB_ALLOW_SYNC_KEYBOARD: xcb_allow_t = 4;
pub const xcb_allow_t_XCB_ALLOW_REPLAY_KEYBOARD: xcb_allow_t = 5;
pub const xcb_allow_t_XCB_ALLOW_ASYNC_BOTH: xcb_allow_t = 6;
pub const xcb_allow_t_XCB_ALLOW_SYNC_BOTH: xcb_allow_t = 7;
pub type xcb_allow_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_allow_events_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_allow_events_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_allow_events_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_allow_events_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_allow_events_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_allow_events_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_allow_events_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_allow_events_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_allow_events_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_allow_events_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_allow_events_request_t),
            "::",
            stringify!(time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_grab_server_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_grab_server_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_grab_server_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_grab_server_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_grab_server_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_grab_server_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_grab_server_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_server_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_server_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_grab_server_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_ungrab_server_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_ungrab_server_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_ungrab_server_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_ungrab_server_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_ungrab_server_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_ungrab_server_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_ungrab_server_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_server_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_server_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_ungrab_server_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_pointer_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_pointer_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_pointer_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_pointer_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_pointer_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_pointer_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_query_pointer_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_pointer_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_pointer_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_query_pointer_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_pointer_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_pointer_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_query_pointer_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_pointer_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_pointer_reply_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_query_pointer_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_pointer_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_pointer_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).same_screen) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(same_screen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).child) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(root_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).root_y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(root_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).win_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(win_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).win_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(win_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mask) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_pointer_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_timecoord_t {
    pub time: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_timecoord_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_timecoord_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_timecoord_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_timecoord_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_timecoord_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_timecoord_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timecoord_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timecoord_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timecoord_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_timecoord_iterator_t {
    pub data: *mut xcb_timecoord_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_timecoord_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_timecoord_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_timecoord_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_timecoord_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_timecoord_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_timecoord_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timecoord_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timecoord_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_timecoord_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_motion_events_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_motion_events_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_motion_events_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_motion_events_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_motion_events_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_motion_events_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_motion_events_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_motion_events_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub start: xcb_timestamp_t,
    pub stop: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_get_motion_events_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_motion_events_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_motion_events_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_get_motion_events_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_motion_events_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_motion_events_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).start) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_request_t),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stop) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_request_t),
            "::",
            stringify!(stop)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_motion_events_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub events_len: u32,
    pub pad1: [u8; 20usize],
}
#[test]
fn bindgen_test_layout_xcb_get_motion_events_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_motion_events_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_motion_events_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_motion_events_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_motion_events_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_motion_events_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).events_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_reply_t),
            "::",
            stringify!(events_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_motion_events_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_translate_coordinates_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_translate_coordinates_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_translate_coordinates_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_translate_coordinates_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_translate_coordinates_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_translate_coordinates_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_translate_coordinates_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_translate_coordinates_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: i16,
    pub src_y: i16,
}
#[test]
fn bindgen_test_layout_xcb_translate_coordinates_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_translate_coordinates_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_translate_coordinates_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_translate_coordinates_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_translate_coordinates_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_translate_coordinates_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(src_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(dst_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(src_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_request_t),
            "::",
            stringify!(src_y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub child: xcb_window_t,
    pub dst_x: i16,
    pub dst_y: i16,
}
#[test]
fn bindgen_test_layout_xcb_translate_coordinates_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_translate_coordinates_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_translate_coordinates_reply_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_translate_coordinates_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_translate_coordinates_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_translate_coordinates_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).same_screen) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(same_screen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).child) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(dst_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_translate_coordinates_reply_t),
            "::",
            stringify!(dst_y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_warp_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: i16,
    pub src_y: i16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
}
#[test]
fn bindgen_test_layout_xcb_warp_pointer_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_warp_pointer_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_warp_pointer_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_warp_pointer_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_warp_pointer_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_warp_pointer_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(src_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(dst_window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(src_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(src_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(src_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_height) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(src_height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(dst_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_warp_pointer_request_t),
            "::",
            stringify!(dst_y)
        )
    );
}
pub const xcb_input_focus_t_XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
pub const xcb_input_focus_t_XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
pub const xcb_input_focus_t_XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
pub const xcb_input_focus_t_XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;
pub type xcb_input_focus_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_input_focus_request_t {
    pub major_opcode: u8,
    pub revert_to: u8,
    pub length: u16,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
}
#[test]
fn bindgen_test_layout_xcb_set_input_focus_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_input_focus_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_input_focus_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_set_input_focus_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_input_focus_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_set_input_focus_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_input_focus_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).revert_to) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_input_focus_request_t),
            "::",
            stringify!(revert_to)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_input_focus_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).focus) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_input_focus_request_t),
            "::",
            stringify!(focus)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_input_focus_request_t),
            "::",
            stringify!(time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_input_focus_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_input_focus_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_input_focus_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_input_focus_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_input_focus_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_input_focus_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_input_focus_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_input_focus_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_input_focus_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_input_focus_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_input_focus_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_input_focus_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_get_input_focus_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: u8,
    pub revert_to: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_get_input_focus_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_input_focus_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_input_focus_reply_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_get_input_focus_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_input_focus_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_input_focus_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).revert_to) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_reply_t),
            "::",
            stringify!(revert_to)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).focus) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_input_focus_reply_t),
            "::",
            stringify!(focus)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_keymap_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_keymap_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_keymap_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_keymap_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_keymap_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_keymap_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_keymap_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_keymap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_query_keymap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_keymap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_keymap_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_keymap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_keymap_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_query_keymap_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_keymap_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub keys: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_xcb_query_keymap_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_keymap_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_keymap_reply_t>(),
        40usize,
        concat!("Size of: ", stringify!(xcb_query_keymap_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_keymap_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_keymap_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keys) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_keymap_reply_t),
            "::",
            stringify!(keys)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_open_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub fid: xcb_font_t,
    pub name_len: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_open_font_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_open_font_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_open_font_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_open_font_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_open_font_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_open_font_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_open_font_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_open_font_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_open_font_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_open_font_request_t),
            "::",
            stringify!(fid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_open_font_request_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_open_font_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_close_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font: xcb_font_t,
}
#[test]
fn bindgen_test_layout_xcb_close_font_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_close_font_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_close_font_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_close_font_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_close_font_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_close_font_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_close_font_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_close_font_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_close_font_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_close_font_request_t),
            "::",
            stringify!(font)
        )
    );
}
pub const xcb_font_draw_t_XCB_FONT_DRAW_LEFT_TO_RIGHT: xcb_font_draw_t = 0;
pub const xcb_font_draw_t_XCB_FONT_DRAW_RIGHT_TO_LEFT: xcb_font_draw_t = 1;
pub type xcb_font_draw_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fontprop_t {
    pub name: xcb_atom_t,
    pub value: u32,
}
#[test]
fn bindgen_test_layout_xcb_fontprop_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_fontprop_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_fontprop_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_fontprop_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_fontprop_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_fontprop_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontprop_t),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontprop_t),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fontprop_iterator_t {
    pub data: *mut xcb_fontprop_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_fontprop_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_fontprop_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_fontprop_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_fontprop_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_fontprop_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_fontprop_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontprop_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontprop_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fontprop_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}
#[test]
fn bindgen_test_layout_xcb_charinfo_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_charinfo_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_charinfo_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_charinfo_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_charinfo_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_charinfo_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).left_side_bearing) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_t),
            "::",
            stringify!(left_side_bearing)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).right_side_bearing) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_t),
            "::",
            stringify!(right_side_bearing)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).character_width) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_t),
            "::",
            stringify!(character_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ascent) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_t),
            "::",
            stringify!(ascent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).descent) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_t),
            "::",
            stringify!(descent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_t),
            "::",
            stringify!(attributes)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_charinfo_iterator_t {
    pub data: *mut xcb_charinfo_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_charinfo_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_charinfo_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_charinfo_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_charinfo_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_charinfo_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_charinfo_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_charinfo_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_font_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_font_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_font_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_font_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_font_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_font_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_font_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font: xcb_fontable_t,
}
#[test]
fn bindgen_test_layout_xcb_query_font_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_font_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_font_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_query_font_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_font_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_font_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_request_t),
            "::",
            stringify!(font)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_font_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4usize],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [u8; 4usize],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub char_infos_len: u32,
}
#[test]
fn bindgen_test_layout_xcb_query_font_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_font_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_font_reply_t>(),
        60usize,
        concat!("Size of: ", stringify!(xcb_query_font_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_font_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_font_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_bounds) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(min_bounds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_bounds) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(max_bounds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad2) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(pad2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_char_or_byte2) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(min_char_or_byte2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_char_or_byte2) as usize - ptr as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(max_char_or_byte2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).default_char) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(default_char)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).properties_len) as usize - ptr as usize },
        46usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(properties_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).draw_direction) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(draw_direction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_byte1) as usize - ptr as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(min_byte1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_byte1) as usize - ptr as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(max_byte1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).all_chars_exist) as usize - ptr as usize },
        51usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(all_chars_exist)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_ascent) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(font_ascent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_descent) as usize - ptr as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(font_descent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).char_infos_len) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_font_reply_t),
            "::",
            stringify!(char_infos_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_text_extents_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_text_extents_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_text_extents_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_text_extents_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_text_extents_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_text_extents_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_text_extents_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_text_extents_request_t {
    pub major_opcode: u8,
    pub odd_length: u8,
    pub length: u16,
    pub font: xcb_fontable_t,
}
#[test]
fn bindgen_test_layout_xcb_query_text_extents_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_text_extents_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_text_extents_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_query_text_extents_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_text_extents_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_query_text_extents_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).odd_length) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_request_t),
            "::",
            stringify!(odd_length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_request_t),
            "::",
            stringify!(font)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type: u8,
    pub draw_direction: u8,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub overall_ascent: i16,
    pub overall_descent: i16,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
}
#[test]
fn bindgen_test_layout_xcb_query_text_extents_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_text_extents_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_text_extents_reply_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_query_text_extents_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_text_extents_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_text_extents_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).draw_direction) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(draw_direction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_ascent) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(font_ascent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_descent) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(font_descent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overall_ascent) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(overall_ascent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overall_descent) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(overall_descent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overall_width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(overall_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overall_left) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(overall_left)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overall_right) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_text_extents_reply_t),
            "::",
            stringify!(overall_right)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_str_t {
    pub name_len: u8,
}
#[test]
fn bindgen_test_layout_xcb_str_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_str_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_str_t>(),
        1usize,
        concat!("Size of: ", stringify!(xcb_str_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_str_t>(),
        1usize,
        concat!("Alignment of ", stringify!(xcb_str_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_str_t),
            "::",
            stringify!(name_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_str_iterator_t {
    pub data: *mut xcb_str_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_str_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_str_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_str_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_str_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_str_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_str_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_str_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_str_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_str_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_list_fonts_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_fonts_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_fonts_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_fonts_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_fonts_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_fonts_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub max_names: u16,
    pub pattern_len: u16,
}
#[test]
fn bindgen_test_layout_xcb_list_fonts_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_fonts_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_fonts_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_list_fonts_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_fonts_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_list_fonts_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_names) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_request_t),
            "::",
            stringify!(max_names)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pattern_len) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_request_t),
            "::",
            stringify!(pattern_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub names_len: u16,
    pub pad1: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_list_fonts_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_fonts_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_fonts_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_list_fonts_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_fonts_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_fonts_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).names_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_reply_t),
            "::",
            stringify!(names_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_with_info_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_list_fonts_with_info_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_fonts_with_info_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_fonts_with_info_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_fonts_with_info_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_fonts_with_info_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_list_fonts_with_info_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_with_info_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub max_names: u16,
    pub pattern_len: u16,
}
#[test]
fn bindgen_test_layout_xcb_list_fonts_with_info_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_fonts_with_info_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_fonts_with_info_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_list_fonts_with_info_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_fonts_with_info_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_list_fonts_with_info_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_names) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_request_t),
            "::",
            stringify!(max_names)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pattern_len) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_request_t),
            "::",
            stringify!(pattern_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_fonts_with_info_reply_t {
    pub response_type: u8,
    pub name_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad0: [u8; 4usize],
    pub max_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4usize],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub replies_hint: u32,
}
#[test]
fn bindgen_test_layout_xcb_list_fonts_with_info_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_fonts_with_info_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_fonts_with_info_reply_t>(),
        60usize,
        concat!("Size of: ", stringify!(xcb_list_fonts_with_info_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_fonts_with_info_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_list_fonts_with_info_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_bounds) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(min_bounds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_bounds) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(max_bounds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_char_or_byte2) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(min_char_or_byte2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_char_or_byte2) as usize - ptr as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(max_char_or_byte2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).default_char) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(default_char)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).properties_len) as usize - ptr as usize },
        46usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(properties_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).draw_direction) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(draw_direction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_byte1) as usize - ptr as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(min_byte1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_byte1) as usize - ptr as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(max_byte1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).all_chars_exist) as usize - ptr as usize },
        51usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(all_chars_exist)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_ascent) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(font_ascent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_descent) as usize - ptr as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(font_descent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).replies_hint) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_fonts_with_info_reply_t),
            "::",
            stringify!(replies_hint)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_font_path_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font_qty: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_set_font_path_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_font_path_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_font_path_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_set_font_path_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_font_path_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_set_font_path_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_font_path_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_font_path_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_font_path_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font_qty) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_font_path_request_t),
            "::",
            stringify!(font_qty)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_font_path_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_font_path_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_font_path_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_font_path_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_font_path_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_font_path_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_font_path_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_font_path_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_font_path_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_font_path_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_font_path_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_font_path_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_font_path_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_font_path_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_get_font_path_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_font_path_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub path_len: u16,
    pub pad1: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_get_font_path_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_font_path_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_font_path_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_font_path_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_font_path_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_font_path_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).path_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_reply_t),
            "::",
            stringify!(path_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_font_path_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_pixmap_request_t {
    pub major_opcode: u8,
    pub depth: u8,
    pub length: u16,
    pub pid: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_create_pixmap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_pixmap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_pixmap_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_create_pixmap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_pixmap_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_pixmap_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(pid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_pixmap_request_t),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_pixmap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
}
#[test]
fn bindgen_test_layout_xcb_free_pixmap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_free_pixmap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_free_pixmap_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_free_pixmap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_free_pixmap_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_free_pixmap_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_pixmap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_pixmap_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_pixmap_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_pixmap_request_t),
            "::",
            stringify!(pixmap)
        )
    );
}
pub const xcb_gc_t_XCB_GC_FUNCTION: xcb_gc_t = 1;
pub const xcb_gc_t_XCB_GC_PLANE_MASK: xcb_gc_t = 2;
pub const xcb_gc_t_XCB_GC_FOREGROUND: xcb_gc_t = 4;
pub const xcb_gc_t_XCB_GC_BACKGROUND: xcb_gc_t = 8;
pub const xcb_gc_t_XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
pub const xcb_gc_t_XCB_GC_LINE_STYLE: xcb_gc_t = 32;
pub const xcb_gc_t_XCB_GC_CAP_STYLE: xcb_gc_t = 64;
pub const xcb_gc_t_XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
pub const xcb_gc_t_XCB_GC_FILL_STYLE: xcb_gc_t = 256;
pub const xcb_gc_t_XCB_GC_FILL_RULE: xcb_gc_t = 512;
pub const xcb_gc_t_XCB_GC_TILE: xcb_gc_t = 1024;
pub const xcb_gc_t_XCB_GC_STIPPLE: xcb_gc_t = 2048;
pub const xcb_gc_t_XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
pub const xcb_gc_t_XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
pub const xcb_gc_t_XCB_GC_FONT: xcb_gc_t = 16384;
pub const xcb_gc_t_XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
pub const xcb_gc_t_XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
pub const xcb_gc_t_XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
pub const xcb_gc_t_XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
pub const xcb_gc_t_XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
pub const xcb_gc_t_XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
pub const xcb_gc_t_XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
pub const xcb_gc_t_XCB_GC_ARC_MODE: xcb_gc_t = 4194304;
pub type xcb_gc_t = ::std::os::raw::c_uint;
pub const xcb_gx_t_XCB_GX_CLEAR: xcb_gx_t = 0;
pub const xcb_gx_t_XCB_GX_AND: xcb_gx_t = 1;
pub const xcb_gx_t_XCB_GX_AND_REVERSE: xcb_gx_t = 2;
pub const xcb_gx_t_XCB_GX_COPY: xcb_gx_t = 3;
pub const xcb_gx_t_XCB_GX_AND_INVERTED: xcb_gx_t = 4;
pub const xcb_gx_t_XCB_GX_NOOP: xcb_gx_t = 5;
pub const xcb_gx_t_XCB_GX_XOR: xcb_gx_t = 6;
pub const xcb_gx_t_XCB_GX_OR: xcb_gx_t = 7;
pub const xcb_gx_t_XCB_GX_NOR: xcb_gx_t = 8;
pub const xcb_gx_t_XCB_GX_EQUIV: xcb_gx_t = 9;
pub const xcb_gx_t_XCB_GX_INVERT: xcb_gx_t = 10;
pub const xcb_gx_t_XCB_GX_OR_REVERSE: xcb_gx_t = 11;
pub const xcb_gx_t_XCB_GX_COPY_INVERTED: xcb_gx_t = 12;
pub const xcb_gx_t_XCB_GX_OR_INVERTED: xcb_gx_t = 13;
pub const xcb_gx_t_XCB_GX_NAND: xcb_gx_t = 14;
pub const xcb_gx_t_XCB_GX_SET: xcb_gx_t = 15;
pub type xcb_gx_t = ::std::os::raw::c_uint;
pub const xcb_line_style_t_XCB_LINE_STYLE_SOLID: xcb_line_style_t = 0;
pub const xcb_line_style_t_XCB_LINE_STYLE_ON_OFF_DASH: xcb_line_style_t = 1;
pub const xcb_line_style_t_XCB_LINE_STYLE_DOUBLE_DASH: xcb_line_style_t = 2;
pub type xcb_line_style_t = ::std::os::raw::c_uint;
pub const xcb_cap_style_t_XCB_CAP_STYLE_NOT_LAST: xcb_cap_style_t = 0;
pub const xcb_cap_style_t_XCB_CAP_STYLE_BUTT: xcb_cap_style_t = 1;
pub const xcb_cap_style_t_XCB_CAP_STYLE_ROUND: xcb_cap_style_t = 2;
pub const xcb_cap_style_t_XCB_CAP_STYLE_PROJECTING: xcb_cap_style_t = 3;
pub type xcb_cap_style_t = ::std::os::raw::c_uint;
pub const xcb_join_style_t_XCB_JOIN_STYLE_MITER: xcb_join_style_t = 0;
pub const xcb_join_style_t_XCB_JOIN_STYLE_ROUND: xcb_join_style_t = 1;
pub const xcb_join_style_t_XCB_JOIN_STYLE_BEVEL: xcb_join_style_t = 2;
pub type xcb_join_style_t = ::std::os::raw::c_uint;
pub const xcb_fill_style_t_XCB_FILL_STYLE_SOLID: xcb_fill_style_t = 0;
pub const xcb_fill_style_t_XCB_FILL_STYLE_TILED: xcb_fill_style_t = 1;
pub const xcb_fill_style_t_XCB_FILL_STYLE_STIPPLED: xcb_fill_style_t = 2;
pub const xcb_fill_style_t_XCB_FILL_STYLE_OPAQUE_STIPPLED: xcb_fill_style_t = 3;
pub type xcb_fill_style_t = ::std::os::raw::c_uint;
pub const xcb_fill_rule_t_XCB_FILL_RULE_EVEN_ODD: xcb_fill_rule_t = 0;
pub const xcb_fill_rule_t_XCB_FILL_RULE_WINDING: xcb_fill_rule_t = 1;
pub type xcb_fill_rule_t = ::std::os::raw::c_uint;
pub const xcb_subwindow_mode_t_XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN: xcb_subwindow_mode_t = 0;
pub const xcb_subwindow_mode_t_XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS: xcb_subwindow_mode_t = 1;
pub type xcb_subwindow_mode_t = ::std::os::raw::c_uint;
pub const xcb_arc_mode_t_XCB_ARC_MODE_CHORD: xcb_arc_mode_t = 0;
pub const xcb_arc_mode_t_XCB_ARC_MODE_PIE_SLICE: xcb_arc_mode_t = 1;
pub type xcb_arc_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_gc_value_list_t {
    pub function: u32,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub tile: xcb_pixmap_t,
    pub stipple: xcb_pixmap_t,
    pub tile_stipple_x_origin: i32,
    pub tile_stipple_y_origin: i32,
    pub font: xcb_font_t,
    pub subwindow_mode: u32,
    pub graphics_exposures: xcb_bool32_t,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: xcb_pixmap_t,
    pub dash_offset: u32,
    pub dashes: u32,
    pub arc_mode: u32,
}
#[test]
fn bindgen_test_layout_xcb_create_gc_value_list_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_gc_value_list_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_gc_value_list_t>(),
        92usize,
        concat!("Size of: ", stringify!(xcb_create_gc_value_list_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_gc_value_list_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_gc_value_list_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).function) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).plane_mask) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(plane_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foreground) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(foreground)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).background) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(background)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line_width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(line_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line_style) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(line_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cap_style) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(cap_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).join_style) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(join_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fill_style) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(fill_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fill_rule) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(fill_rule)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tile) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(tile)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stipple) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(stipple)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tile_stipple_x_origin) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(tile_stipple_x_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tile_stipple_y_origin) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(tile_stipple_y_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(font)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subwindow_mode) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(subwindow_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).graphics_exposures) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(graphics_exposures)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_x_origin) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(clip_x_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_y_origin) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(clip_y_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_mask) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(clip_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dash_offset) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(dash_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dashes) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(dashes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arc_mode) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_value_list_t),
            "::",
            stringify!(arc_mode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_gcontext_t,
    pub drawable: xcb_drawable_t,
    pub value_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_create_gc_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_gc_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_gc_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_create_gc_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_gc_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_gc_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_request_t),
            "::",
            stringify!(cid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_gc_request_t),
            "::",
            stringify!(value_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_gc_value_list_t {
    pub function: u32,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub tile: xcb_pixmap_t,
    pub stipple: xcb_pixmap_t,
    pub tile_stipple_x_origin: i32,
    pub tile_stipple_y_origin: i32,
    pub font: xcb_font_t,
    pub subwindow_mode: u32,
    pub graphics_exposures: xcb_bool32_t,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: xcb_pixmap_t,
    pub dash_offset: u32,
    pub dashes: u32,
    pub arc_mode: u32,
}
#[test]
fn bindgen_test_layout_xcb_change_gc_value_list_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_gc_value_list_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_gc_value_list_t>(),
        92usize,
        concat!("Size of: ", stringify!(xcb_change_gc_value_list_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_gc_value_list_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_change_gc_value_list_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).function) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).plane_mask) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(plane_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foreground) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(foreground)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).background) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(background)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line_width) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(line_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line_style) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(line_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cap_style) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(cap_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).join_style) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(join_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fill_style) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(fill_style)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fill_rule) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(fill_rule)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tile) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(tile)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stipple) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(stipple)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tile_stipple_x_origin) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(tile_stipple_x_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tile_stipple_y_origin) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(tile_stipple_y_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).font) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(font)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subwindow_mode) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(subwindow_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).graphics_exposures) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(graphics_exposures)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_x_origin) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(clip_x_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_y_origin) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(clip_y_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_mask) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(clip_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dash_offset) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(dash_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dashes) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(dashes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arc_mode) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_value_list_t),
            "::",
            stringify!(arc_mode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub value_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_change_gc_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_gc_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_gc_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_change_gc_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_gc_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_change_gc_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_gc_request_t),
            "::",
            stringify!(value_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_gc: xcb_gcontext_t,
    pub dst_gc: xcb_gcontext_t,
    pub value_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_copy_gc_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_copy_gc_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_copy_gc_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_copy_gc_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_copy_gc_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_copy_gc_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_gc_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_gc_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_gc_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_gc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_gc_request_t),
            "::",
            stringify!(src_gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_gc_request_t),
            "::",
            stringify!(dst_gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_gc_request_t),
            "::",
            stringify!(value_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_dashes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub dash_offset: u16,
    pub dashes_len: u16,
}
#[test]
fn bindgen_test_layout_xcb_set_dashes_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_dashes_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_dashes_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_set_dashes_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_dashes_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_set_dashes_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_dashes_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_dashes_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_dashes_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_dashes_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dash_offset) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_dashes_request_t),
            "::",
            stringify!(dash_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dashes_len) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_dashes_request_t),
            "::",
            stringify!(dashes_len)
        )
    );
}
pub const xcb_clip_ordering_t_XCB_CLIP_ORDERING_UNSORTED: xcb_clip_ordering_t = 0;
pub const xcb_clip_ordering_t_XCB_CLIP_ORDERING_Y_SORTED: xcb_clip_ordering_t = 1;
pub const xcb_clip_ordering_t_XCB_CLIP_ORDERING_YX_SORTED: xcb_clip_ordering_t = 2;
pub const xcb_clip_ordering_t_XCB_CLIP_ORDERING_YX_BANDED: xcb_clip_ordering_t = 3;
pub type xcb_clip_ordering_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_clip_rectangles_request_t {
    pub major_opcode: u8,
    pub ordering: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
}
#[test]
fn bindgen_test_layout_xcb_set_clip_rectangles_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_clip_rectangles_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_clip_rectangles_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_set_clip_rectangles_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_clip_rectangles_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_clip_rectangles_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_clip_rectangles_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ordering) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_clip_rectangles_request_t),
            "::",
            stringify!(ordering)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_clip_rectangles_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_clip_rectangles_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_x_origin) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_clip_rectangles_request_t),
            "::",
            stringify!(clip_x_origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clip_y_origin) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_clip_rectangles_request_t),
            "::",
            stringify!(clip_y_origin)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_free_gc_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_free_gc_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_free_gc_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_free_gc_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_free_gc_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_free_gc_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_gc_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_gc_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_gc_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_gc_request_t),
            "::",
            stringify!(gc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_clear_area_request_t {
    pub major_opcode: u8,
    pub exposures: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_clear_area_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_clear_area_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_clear_area_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_clear_area_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_clear_area_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_clear_area_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exposures) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(exposures)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_clear_area_request_t),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_area_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_copy_area_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_copy_area_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_copy_area_request_t>(),
        28usize,
        concat!("Size of: ", stringify!(xcb_copy_area_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_copy_area_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_copy_area_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(src_drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_drawable) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(dst_drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(src_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(src_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(dst_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(dst_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_area_request_t),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_plane_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
    pub bit_plane: u32,
}
#[test]
fn bindgen_test_layout_xcb_copy_plane_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_copy_plane_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_copy_plane_request_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_copy_plane_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_copy_plane_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_copy_plane_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(src_drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_drawable) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(dst_drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(src_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(src_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_x) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(dst_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_y) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(dst_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bit_plane) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_plane_request_t),
            "::",
            stringify!(bit_plane)
        )
    );
}
pub const xcb_coord_mode_t_XCB_COORD_MODE_ORIGIN: xcb_coord_mode_t = 0;
pub const xcb_coord_mode_t_XCB_COORD_MODE_PREVIOUS: xcb_coord_mode_t = 1;
pub type xcb_coord_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_point_request_t {
    pub major_opcode: u8,
    pub coordinate_mode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_point_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_point_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_point_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_point_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_point_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_point_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_point_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).coordinate_mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_point_request_t),
            "::",
            stringify!(coordinate_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_point_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_point_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_point_request_t),
            "::",
            stringify!(gc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_line_request_t {
    pub major_opcode: u8,
    pub coordinate_mode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_line_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_line_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_line_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_line_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_line_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_line_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_line_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).coordinate_mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_line_request_t),
            "::",
            stringify!(coordinate_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_line_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_line_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_line_request_t),
            "::",
            stringify!(gc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_segment_t {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}
#[test]
fn bindgen_test_layout_xcb_segment_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_segment_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_segment_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_segment_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_segment_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_segment_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_t),
            "::",
            stringify!(x1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y1) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_t),
            "::",
            stringify!(y1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x2) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_t),
            "::",
            stringify!(x2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y2) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_t),
            "::",
            stringify!(y2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_segment_iterator_t {
    pub data: *mut xcb_segment_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_segment_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_segment_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_segment_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_segment_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_segment_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_segment_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_segment_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_segment_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_segment_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_segment_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_segment_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_segment_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_segment_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_segment_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_segment_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_segment_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_segment_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_segment_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_segment_request_t),
            "::",
            stringify!(gc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_rectangle_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_rectangle_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_rectangle_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_rectangle_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_rectangle_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_rectangle_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_rectangle_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_rectangle_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_rectangle_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_rectangle_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_rectangle_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_rectangle_request_t),
            "::",
            stringify!(gc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_arc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_arc_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_arc_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_arc_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_arc_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_arc_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_arc_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_arc_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_arc_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_arc_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_arc_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_arc_request_t),
            "::",
            stringify!(gc)
        )
    );
}
pub const xcb_poly_shape_t_XCB_POLY_SHAPE_COMPLEX: xcb_poly_shape_t = 0;
pub const xcb_poly_shape_t_XCB_POLY_SHAPE_NONCONVEX: xcb_poly_shape_t = 1;
pub const xcb_poly_shape_t_XCB_POLY_SHAPE_CONVEX: xcb_poly_shape_t = 2;
pub type xcb_poly_shape_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_fill_poly_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub shape: u8,
    pub coordinate_mode: u8,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_fill_poly_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_fill_poly_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_fill_poly_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_fill_poly_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_fill_poly_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_fill_poly_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shape) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(shape)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).coordinate_mode) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(coordinate_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_fill_poly_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_fill_rectangle_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_fill_rectangle_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_fill_rectangle_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_fill_rectangle_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_fill_rectangle_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_fill_rectangle_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_poly_fill_rectangle_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_rectangle_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_rectangle_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_rectangle_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_rectangle_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_rectangle_request_t),
            "::",
            stringify!(gc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_fill_arc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}
#[test]
fn bindgen_test_layout_xcb_poly_fill_arc_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_fill_arc_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_fill_arc_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_poly_fill_arc_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_fill_arc_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_fill_arc_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_arc_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_arc_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_arc_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_arc_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_fill_arc_request_t),
            "::",
            stringify!(gc)
        )
    );
}
pub const xcb_image_format_t_XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
pub const xcb_image_format_t_XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const xcb_image_format_t_XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;
pub type xcb_image_format_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_put_image_request_t {
    pub major_opcode: u8,
    pub format: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub width: u16,
    pub height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub left_pad: u8,
    pub depth: u8,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_put_image_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_put_image_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_put_image_request_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_put_image_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_put_image_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_put_image_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_x) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(dst_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_y) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(dst_y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).left_pad) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(left_pad)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_put_image_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_image_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_image_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_image_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_image_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_image_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_image_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_image_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_image_request_t {
    pub major_opcode: u8,
    pub format: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_get_image_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_image_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_image_request_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_get_image_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_image_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_image_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).plane_mask) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_request_t),
            "::",
            stringify!(plane_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_image_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub pad0: [u8; 20usize],
}
#[test]
fn bindgen_test_layout_xcb_get_image_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_image_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_image_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_image_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_image_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_image_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_reply_t),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_reply_t),
            "::",
            stringify!(visual)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_image_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_text_8_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_poly_text_8_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_text_8_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_text_8_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_poly_text_8_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_text_8_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_text_8_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_8_request_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_poly_text_16_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_poly_text_16_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_poly_text_16_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_poly_text_16_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_poly_text_16_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_poly_text_16_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_poly_text_16_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_poly_text_16_request_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_image_text_8_request_t {
    pub major_opcode: u8,
    pub string_len: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_image_text_8_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_image_text_8_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_image_text_8_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_image_text_8_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_image_text_8_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_image_text_8_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).string_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(string_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_8_request_t),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_image_text_16_request_t {
    pub major_opcode: u8,
    pub string_len: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}
#[test]
fn bindgen_test_layout_xcb_image_text_16_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_image_text_16_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_image_text_16_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_image_text_16_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_image_text_16_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_image_text_16_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).string_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(string_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(gc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_image_text_16_request_t),
            "::",
            stringify!(y)
        )
    );
}
pub const xcb_colormap_alloc_t_XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
pub const xcb_colormap_alloc_t_XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub type xcb_colormap_alloc_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_colormap_request_t {
    pub major_opcode: u8,
    pub alloc: u8,
    pub length: u16,
    pub mid: xcb_colormap_t,
    pub window: xcb_window_t,
    pub visual: xcb_visualid_t,
}
#[test]
fn bindgen_test_layout_xcb_create_colormap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_colormap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_colormap_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_create_colormap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_colormap_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_colormap_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_colormap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alloc) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_colormap_request_t),
            "::",
            stringify!(alloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_colormap_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_colormap_request_t),
            "::",
            stringify!(mid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_colormap_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_colormap_request_t),
            "::",
            stringify!(visual)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}
#[test]
fn bindgen_test_layout_xcb_free_colormap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_free_colormap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_free_colormap_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_free_colormap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_free_colormap_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_free_colormap_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colormap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colormap_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colormap_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colormap_request_t),
            "::",
            stringify!(cmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_copy_colormap_and_free_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub mid: xcb_colormap_t,
    pub src_cmap: xcb_colormap_t,
}
#[test]
fn bindgen_test_layout_xcb_copy_colormap_and_free_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_copy_colormap_and_free_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_copy_colormap_and_free_request_t>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(xcb_copy_colormap_and_free_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_copy_colormap_and_free_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_copy_colormap_and_free_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_colormap_and_free_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_colormap_and_free_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_colormap_and_free_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_colormap_and_free_request_t),
            "::",
            stringify!(mid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_cmap) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_copy_colormap_and_free_request_t),
            "::",
            stringify!(src_cmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_install_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}
#[test]
fn bindgen_test_layout_xcb_install_colormap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_install_colormap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_install_colormap_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_install_colormap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_install_colormap_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_install_colormap_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_install_colormap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_install_colormap_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_install_colormap_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_install_colormap_request_t),
            "::",
            stringify!(cmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_uninstall_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}
#[test]
fn bindgen_test_layout_xcb_uninstall_colormap_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_uninstall_colormap_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_uninstall_colormap_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_uninstall_colormap_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_uninstall_colormap_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_uninstall_colormap_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_uninstall_colormap_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_uninstall_colormap_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_uninstall_colormap_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_uninstall_colormap_request_t),
            "::",
            stringify!(cmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_installed_colormaps_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_list_installed_colormaps_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_installed_colormaps_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_installed_colormaps_cookie_t>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(xcb_list_installed_colormaps_cookie_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_installed_colormaps_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_list_installed_colormaps_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_installed_colormaps_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_xcb_list_installed_colormaps_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_installed_colormaps_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_installed_colormaps_request_t>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(xcb_list_installed_colormaps_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_installed_colormaps_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_list_installed_colormaps_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_request_t),
            "::",
            stringify!(window)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_installed_colormaps_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub cmaps_len: u16,
    pub pad1: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_list_installed_colormaps_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_installed_colormaps_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_installed_colormaps_reply_t>(),
        32usize,
        concat!(
            "Size of: ",
            stringify!(xcb_list_installed_colormaps_reply_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_installed_colormaps_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_list_installed_colormaps_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmaps_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_reply_t),
            "::",
            stringify!(cmaps_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_installed_colormaps_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).red) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).green) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blue) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2usize],
    pub pixel: u32,
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_reply_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).red) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).green) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blue) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixel) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_reply_t),
            "::",
            stringify!(pixel)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_named_color_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_alloc_named_color_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_named_color_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_named_color_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_alloc_named_color_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_named_color_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_named_color_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_named_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub name_len: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_alloc_named_color_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_named_color_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_named_color_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_alloc_named_color_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_named_color_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_named_color_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_request_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_named_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixel: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}
#[test]
fn bindgen_test_layout_xcb_alloc_named_color_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_named_color_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_named_color_reply_t>(),
        24usize,
        concat!("Size of: ", stringify!(xcb_alloc_named_color_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_named_color_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_named_color_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixel) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exact_red) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(exact_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exact_green) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(exact_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exact_blue) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(exact_blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_red) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(visual_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_green) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(visual_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_blue) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_named_color_reply_t),
            "::",
            stringify!(visual_blue)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cells_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_cells_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_cells_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_cells_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_cells_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_cells_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_cells_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cells_request_t {
    pub major_opcode: u8,
    pub contiguous: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub colors: u16,
    pub planes: u16,
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_cells_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_cells_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_cells_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_cells_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_cells_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_cells_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contiguous) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_request_t),
            "::",
            stringify!(contiguous)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colors) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_request_t),
            "::",
            stringify!(colors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).planes) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_request_t),
            "::",
            stringify!(planes)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_cells_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels_len: u16,
    pub masks_len: u16,
    pub pad1: [u8; 20usize],
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_cells_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_cells_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_cells_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_cells_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_cells_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_cells_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixels_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(pixels_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).masks_len) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(masks_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_cells_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_planes_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_planes_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_planes_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_planes_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_planes_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_planes_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_planes_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_planes_request_t {
    pub major_opcode: u8,
    pub contiguous: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub colors: u16,
    pub reds: u16,
    pub greens: u16,
    pub blues: u16,
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_planes_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_planes_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_planes_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_planes_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_planes_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_alloc_color_planes_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contiguous) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(contiguous)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colors) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(colors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reds) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(reds)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).greens) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(greens)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blues) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_request_t),
            "::",
            stringify!(blues)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_alloc_color_planes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels_len: u16,
    pub pad1: [u8; 2usize],
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad2: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_xcb_alloc_color_planes_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_alloc_color_planes_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_alloc_color_planes_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_alloc_color_planes_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_alloc_color_planes_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_alloc_color_planes_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixels_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(pixels_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).red_mask) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(red_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).green_mask) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(green_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blue_mask) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(blue_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad2) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_alloc_color_planes_reply_t),
            "::",
            stringify!(pad2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub plane_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_free_colors_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_free_colors_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_free_colors_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_free_colors_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_free_colors_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_free_colors_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colors_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colors_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colors_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colors_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).plane_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_colors_request_t),
            "::",
            stringify!(plane_mask)
        )
    );
}
pub const xcb_color_flag_t_XCB_COLOR_FLAG_RED: xcb_color_flag_t = 1;
pub const xcb_color_flag_t_XCB_COLOR_FLAG_GREEN: xcb_color_flag_t = 2;
pub const xcb_color_flag_t_XCB_COLOR_FLAG_BLUE: xcb_color_flag_t = 4;
pub type xcb_color_flag_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_coloritem_t {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: u8,
    pub pad0: u8,
}
#[test]
fn bindgen_test_layout_xcb_coloritem_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_coloritem_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_coloritem_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_coloritem_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_coloritem_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_coloritem_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixel) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_t),
            "::",
            stringify!(pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).red) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_t),
            "::",
            stringify!(red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).green) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_t),
            "::",
            stringify!(green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blue) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_t),
            "::",
            stringify!(blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_t),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_coloritem_iterator_t {
    pub data: *mut xcb_coloritem_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_coloritem_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_coloritem_iterator_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_coloritem_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_coloritem_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_coloritem_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_coloritem_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_coloritem_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_store_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}
#[test]
fn bindgen_test_layout_xcb_store_colors_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_store_colors_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_store_colors_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_store_colors_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_store_colors_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_store_colors_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_colors_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_colors_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_colors_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_colors_request_t),
            "::",
            stringify!(cmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_store_named_color_request_t {
    pub major_opcode: u8,
    pub flags: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub pixel: u32,
    pub name_len: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_store_named_color_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_store_named_color_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_store_named_color_request_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_store_named_color_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_store_named_color_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_store_named_color_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixel) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(pixel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_store_named_color_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rgb_t {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_rgb_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_rgb_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_rgb_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_rgb_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_rgb_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_rgb_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).red) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_t),
            "::",
            stringify!(red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).green) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_t),
            "::",
            stringify!(green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).blue) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_t),
            "::",
            stringify!(blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rgb_iterator_t {
    pub data: *mut xcb_rgb_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_rgb_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_rgb_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_rgb_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_rgb_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_rgb_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_rgb_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rgb_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_colors_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_colors_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_colors_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_colors_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_colors_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_colors_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_colors_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}
#[test]
fn bindgen_test_layout_xcb_query_colors_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_colors_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_colors_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_query_colors_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_colors_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_colors_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_request_t),
            "::",
            stringify!(cmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_colors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub colors_len: u16,
    pub pad1: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_query_colors_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_colors_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_colors_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_query_colors_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_colors_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_colors_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).colors_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_reply_t),
            "::",
            stringify!(colors_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_colors_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_lookup_color_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_lookup_color_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_lookup_color_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_lookup_color_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_lookup_color_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_lookup_color_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_lookup_color_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_lookup_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub name_len: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_lookup_color_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_lookup_color_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_lookup_color_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_lookup_color_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_lookup_color_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_lookup_color_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmap) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_request_t),
            "::",
            stringify!(cmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_request_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_lookup_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}
#[test]
fn bindgen_test_layout_xcb_lookup_color_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_lookup_color_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_lookup_color_reply_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_lookup_color_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_lookup_color_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_lookup_color_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exact_red) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(exact_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exact_green) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(exact_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exact_blue) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(exact_blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_red) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(visual_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_green) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(visual_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).visual_blue) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_lookup_color_reply_t),
            "::",
            stringify!(visual_blue)
        )
    );
}
pub const xcb_pixmap_enum_t_XCB_PIXMAP_NONE: xcb_pixmap_enum_t = 0;
pub type xcb_pixmap_enum_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source: xcb_pixmap_t,
    pub mask: xcb_pixmap_t,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
    pub x: u16,
    pub y: u16,
}
#[test]
fn bindgen_test_layout_xcb_create_cursor_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_cursor_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_cursor_request_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_create_cursor_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_cursor_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_create_cursor_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(cid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mask) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_red) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(fore_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_green) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(fore_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_blue) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(fore_blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_red) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(back_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_green) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(back_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_blue) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(back_blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_cursor_request_t),
            "::",
            stringify!(y)
        )
    );
}
pub const xcb_font_enum_t_XCB_FONT_NONE: xcb_font_enum_t = 0;
pub type xcb_font_enum_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_create_glyph_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source_font: xcb_font_t,
    pub mask_font: xcb_font_t,
    pub source_char: u16,
    pub mask_char: u16,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}
#[test]
fn bindgen_test_layout_xcb_create_glyph_cursor_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_create_glyph_cursor_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_create_glyph_cursor_request_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_create_glyph_cursor_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_create_glyph_cursor_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_create_glyph_cursor_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(cid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source_font) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(source_font)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mask_font) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(mask_font)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source_char) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(source_char)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mask_char) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(mask_char)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_red) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(fore_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_green) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(fore_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_blue) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(fore_blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_red) as usize - ptr as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(back_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_green) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(back_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_blue) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_create_glyph_cursor_request_t),
            "::",
            stringify!(back_blue)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_free_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
}
#[test]
fn bindgen_test_layout_xcb_free_cursor_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_free_cursor_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_free_cursor_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_free_cursor_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_free_cursor_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_free_cursor_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_cursor_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_cursor_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_cursor_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_free_cursor_request_t),
            "::",
            stringify!(cursor)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_recolor_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}
#[test]
fn bindgen_test_layout_xcb_recolor_cursor_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_recolor_cursor_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_recolor_cursor_request_t>(),
        20usize,
        concat!("Size of: ", stringify!(xcb_recolor_cursor_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_recolor_cursor_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_recolor_cursor_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cursor) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(cursor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_red) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(fore_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_green) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(fore_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fore_blue) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(fore_blue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_red) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(back_red)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_green) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(back_green)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).back_blue) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_recolor_cursor_request_t),
            "::",
            stringify!(back_blue)
        )
    );
}
pub const xcb_query_shape_of_t_XCB_QUERY_SHAPE_OF_LARGEST_CURSOR: xcb_query_shape_of_t = 0;
pub const xcb_query_shape_of_t_XCB_QUERY_SHAPE_OF_FASTEST_TILE: xcb_query_shape_of_t = 1;
pub const xcb_query_shape_of_t_XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE: xcb_query_shape_of_t = 2;
pub type xcb_query_shape_of_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_best_size_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_best_size_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_best_size_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_best_size_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_best_size_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_best_size_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_best_size_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_best_size_request_t {
    pub major_opcode: u8,
    pub _class: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_query_best_size_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_best_size_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_best_size_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_query_best_size_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_best_size_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_best_size_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._class) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_request_t),
            "::",
            stringify!(_class)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drawable) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_request_t),
            "::",
            stringify!(drawable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_request_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_request_t),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_best_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
}
#[test]
fn bindgen_test_layout_xcb_query_best_size_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_best_size_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_best_size_reply_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_query_best_size_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_best_size_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_best_size_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_reply_t),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_best_size_reply_t),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_extension_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_query_extension_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_extension_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_extension_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_query_extension_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_extension_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_extension_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_extension_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad1: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_query_extension_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_extension_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_extension_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_query_extension_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_extension_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_query_extension_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name_len) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_request_t),
            "::",
            stringify!(name_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_request_t),
            "::",
            stringify!(pad1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_query_extension_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u8,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}
#[test]
fn bindgen_test_layout_xcb_query_extension_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_query_extension_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_query_extension_reply_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_query_extension_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_query_extension_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_query_extension_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).present) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(present)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).first_event) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(first_event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).first_error) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_query_extension_reply_t),
            "::",
            stringify!(first_error)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_extensions_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_list_extensions_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_extensions_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_extensions_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_extensions_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_extensions_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_extensions_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_extensions_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_list_extensions_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_extensions_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_extensions_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_extensions_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_extensions_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_list_extensions_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_extensions_reply_t {
    pub response_type: u8,
    pub names_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24usize],
}
#[test]
fn bindgen_test_layout_xcb_list_extensions_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_extensions_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_extensions_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_list_extensions_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_extensions_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_extensions_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).names_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_reply_t),
            "::",
            stringify!(names_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_extensions_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub keycode_count: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub keysyms_per_keycode: u8,
    pub pad0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_xcb_change_keyboard_mapping_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_keyboard_mapping_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_keyboard_mapping_request_t>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_keyboard_mapping_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_keyboard_mapping_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_keyboard_mapping_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_mapping_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keycode_count) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_mapping_request_t),
            "::",
            stringify!(keycode_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_mapping_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).first_keycode) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_mapping_request_t),
            "::",
            stringify!(first_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keysyms_per_keycode) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_mapping_request_t),
            "::",
            stringify!(keysyms_per_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_mapping_request_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_keyboard_mapping_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_keyboard_mapping_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_keyboard_mapping_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_keyboard_mapping_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_keyboard_mapping_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_keyboard_mapping_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
}
#[test]
fn bindgen_test_layout_xcb_get_keyboard_mapping_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_keyboard_mapping_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_keyboard_mapping_request_t>(),
        6usize,
        concat!("Size of: ", stringify!(xcb_get_keyboard_mapping_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_keyboard_mapping_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_keyboard_mapping_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).first_keycode) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_request_t),
            "::",
            stringify!(first_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_request_t),
            "::",
            stringify!(count)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_mapping_reply_t {
    pub response_type: u8,
    pub keysyms_per_keycode: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24usize],
}
#[test]
fn bindgen_test_layout_xcb_get_keyboard_mapping_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_keyboard_mapping_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_keyboard_mapping_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_keyboard_mapping_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_keyboard_mapping_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_keyboard_mapping_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keysyms_per_keycode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_reply_t),
            "::",
            stringify!(keysyms_per_keycode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_mapping_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub const xcb_kb_t_XCB_KB_KEY_CLICK_PERCENT: xcb_kb_t = 1;
pub const xcb_kb_t_XCB_KB_BELL_PERCENT: xcb_kb_t = 2;
pub const xcb_kb_t_XCB_KB_BELL_PITCH: xcb_kb_t = 4;
pub const xcb_kb_t_XCB_KB_BELL_DURATION: xcb_kb_t = 8;
pub const xcb_kb_t_XCB_KB_LED: xcb_kb_t = 16;
pub const xcb_kb_t_XCB_KB_LED_MODE: xcb_kb_t = 32;
pub const xcb_kb_t_XCB_KB_KEY: xcb_kb_t = 64;
pub const xcb_kb_t_XCB_KB_AUTO_REPEAT_MODE: xcb_kb_t = 128;
pub type xcb_kb_t = ::std::os::raw::c_uint;
pub const xcb_led_mode_t_XCB_LED_MODE_OFF: xcb_led_mode_t = 0;
pub const xcb_led_mode_t_XCB_LED_MODE_ON: xcb_led_mode_t = 1;
pub type xcb_led_mode_t = ::std::os::raw::c_uint;
pub const xcb_auto_repeat_mode_t_XCB_AUTO_REPEAT_MODE_OFF: xcb_auto_repeat_mode_t = 0;
pub const xcb_auto_repeat_mode_t_XCB_AUTO_REPEAT_MODE_ON: xcb_auto_repeat_mode_t = 1;
pub const xcb_auto_repeat_mode_t_XCB_AUTO_REPEAT_MODE_DEFAULT: xcb_auto_repeat_mode_t = 2;
pub type xcb_auto_repeat_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_keyboard_control_value_list_t {
    pub key_click_percent: i32,
    pub bell_percent: i32,
    pub bell_pitch: i32,
    pub bell_duration: i32,
    pub led: u32,
    pub led_mode: u32,
    pub key: xcb_keycode32_t,
    pub auto_repeat_mode: u32,
}
#[test]
fn bindgen_test_layout_xcb_change_keyboard_control_value_list_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_keyboard_control_value_list_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_keyboard_control_value_list_t>(),
        32usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_keyboard_control_value_list_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_keyboard_control_value_list_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_keyboard_control_value_list_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key_click_percent) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(key_click_percent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bell_percent) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(bell_percent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bell_pitch) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(bell_pitch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bell_duration) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(bell_duration)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).led) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(led)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).led_mode) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(led_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).auto_repeat_mode) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_value_list_t),
            "::",
            stringify!(auto_repeat_mode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_keyboard_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub value_mask: u32,
}
#[test]
fn bindgen_test_layout_xcb_change_keyboard_control_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_keyboard_control_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_keyboard_control_request_t>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_keyboard_control_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_keyboard_control_request_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_keyboard_control_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_mask) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_keyboard_control_request_t),
            "::",
            stringify!(value_mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_control_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_keyboard_control_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_keyboard_control_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_keyboard_control_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_keyboard_control_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_keyboard_control_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_keyboard_control_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_keyboard_control_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_keyboard_control_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_keyboard_control_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_keyboard_control_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_keyboard_control_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_keyboard_control_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_keyboard_control_reply_t {
    pub response_type: u8,
    pub global_auto_repeat: u8,
    pub sequence: u16,
    pub length: u32,
    pub led_mask: u32,
    pub key_click_percent: u8,
    pub bell_percent: u8,
    pub bell_pitch: u16,
    pub bell_duration: u16,
    pub pad0: [u8; 2usize],
    pub auto_repeats: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_xcb_get_keyboard_control_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_keyboard_control_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_keyboard_control_reply_t>(),
        52usize,
        concat!("Size of: ", stringify!(xcb_get_keyboard_control_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_keyboard_control_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_keyboard_control_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).global_auto_repeat) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(global_auto_repeat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).led_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(led_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key_click_percent) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(key_click_percent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bell_percent) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(bell_percent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bell_pitch) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(bell_pitch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bell_duration) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(bell_duration)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).auto_repeats) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_keyboard_control_reply_t),
            "::",
            stringify!(auto_repeats)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_bell_request_t {
    pub major_opcode: u8,
    pub percent: i8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_bell_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_bell_request_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_bell_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_bell_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_bell_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_bell_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_bell_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).percent) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_bell_request_t),
            "::",
            stringify!(percent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_bell_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_pointer_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub acceleration_numerator: i16,
    pub acceleration_denominator: i16,
    pub threshold: i16,
    pub do_acceleration: u8,
    pub do_threshold: u8,
}
#[test]
fn bindgen_test_layout_xcb_change_pointer_control_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_pointer_control_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_pointer_control_request_t>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(xcb_change_pointer_control_request_t)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_pointer_control_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_change_pointer_control_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).acceleration_numerator) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(acceleration_numerator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).acceleration_denominator) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(acceleration_denominator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).threshold) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(threshold)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).do_acceleration) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(do_acceleration)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).do_threshold) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_pointer_control_request_t),
            "::",
            stringify!(do_threshold)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_control_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_pointer_control_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_pointer_control_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_pointer_control_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_pointer_control_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_pointer_control_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_pointer_control_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_pointer_control_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_pointer_control_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_pointer_control_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_pointer_control_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_pointer_control_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_pointer_control_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_control_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub acceleration_numerator: u16,
    pub acceleration_denominator: u16,
    pub threshold: u16,
    pub pad1: [u8; 18usize],
}
#[test]
fn bindgen_test_layout_xcb_get_pointer_control_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_pointer_control_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_pointer_control_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_pointer_control_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_pointer_control_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_pointer_control_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).acceleration_numerator) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(acceleration_numerator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).acceleration_denominator) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(acceleration_denominator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).threshold) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(threshold)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_control_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
pub const xcb_blanking_t_XCB_BLANKING_NOT_PREFERRED: xcb_blanking_t = 0;
pub const xcb_blanking_t_XCB_BLANKING_PREFERRED: xcb_blanking_t = 1;
pub const xcb_blanking_t_XCB_BLANKING_DEFAULT: xcb_blanking_t = 2;
pub type xcb_blanking_t = ::std::os::raw::c_uint;
pub const xcb_exposures_t_XCB_EXPOSURES_NOT_ALLOWED: xcb_exposures_t = 0;
pub const xcb_exposures_t_XCB_EXPOSURES_ALLOWED: xcb_exposures_t = 1;
pub const xcb_exposures_t_XCB_EXPOSURES_DEFAULT: xcb_exposures_t = 2;
pub type xcb_exposures_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_screen_saver_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub timeout: i16,
    pub interval: i16,
    pub prefer_blanking: u8,
    pub allow_exposures: u8,
}
#[test]
fn bindgen_test_layout_xcb_set_screen_saver_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_screen_saver_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_screen_saver_request_t>(),
        10usize,
        concat!("Size of: ", stringify!(xcb_set_screen_saver_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_screen_saver_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_set_screen_saver_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timeout) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interval) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(interval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prefer_blanking) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(prefer_blanking)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).allow_exposures) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_screen_saver_request_t),
            "::",
            stringify!(allow_exposures)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_screen_saver_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_screen_saver_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_screen_saver_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_screen_saver_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_screen_saver_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_screen_saver_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_screen_saver_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_screen_saver_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_screen_saver_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_screen_saver_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_screen_saver_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_screen_saver_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_screen_saver_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_get_screen_saver_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_screen_saver_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timeout: u16,
    pub interval: u16,
    pub prefer_blanking: u8,
    pub allow_exposures: u8,
    pub pad1: [u8; 18usize],
}
#[test]
fn bindgen_test_layout_xcb_get_screen_saver_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_screen_saver_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_screen_saver_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_screen_saver_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_screen_saver_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_screen_saver_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timeout) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interval) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(interval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prefer_blanking) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(prefer_blanking)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).allow_exposures) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(allow_exposures)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_screen_saver_reply_t),
            "::",
            stringify!(pad1)
        )
    );
}
pub const xcb_host_mode_t_XCB_HOST_MODE_INSERT: xcb_host_mode_t = 0;
pub const xcb_host_mode_t_XCB_HOST_MODE_DELETE: xcb_host_mode_t = 1;
pub type xcb_host_mode_t = ::std::os::raw::c_uint;
pub const xcb_family_t_XCB_FAMILY_INTERNET: xcb_family_t = 0;
pub const xcb_family_t_XCB_FAMILY_DECNET: xcb_family_t = 1;
pub const xcb_family_t_XCB_FAMILY_CHAOS: xcb_family_t = 2;
pub const xcb_family_t_XCB_FAMILY_SERVER_INTERPRETED: xcb_family_t = 5;
pub const xcb_family_t_XCB_FAMILY_INTERNET_6: xcb_family_t = 6;
pub type xcb_family_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_change_hosts_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub family: u8,
    pub pad0: u8,
    pub address_len: u16,
}
#[test]
fn bindgen_test_layout_xcb_change_hosts_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_change_hosts_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_change_hosts_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_change_hosts_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_change_hosts_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_change_hosts_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_hosts_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_hosts_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_hosts_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).family) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_hosts_request_t),
            "::",
            stringify!(family)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_hosts_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).address_len) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_change_hosts_request_t),
            "::",
            stringify!(address_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_host_t {
    pub family: u8,
    pub pad0: u8,
    pub address_len: u16,
}
#[test]
fn bindgen_test_layout_xcb_host_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_host_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_host_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_host_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_host_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_host_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).family) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_host_t),
            "::",
            stringify!(family)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_host_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).address_len) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_host_t),
            "::",
            stringify!(address_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_host_iterator_t {
    pub data: *mut xcb_host_t,
    pub rem: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_xcb_host_iterator_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_host_iterator_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_host_iterator_t>(),
        16usize,
        concat!("Size of: ", stringify!(xcb_host_iterator_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_host_iterator_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_host_iterator_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_host_iterator_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_host_iterator_t),
            "::",
            stringify!(rem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_host_iterator_t),
            "::",
            stringify!(index)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_hosts_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_list_hosts_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_hosts_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_hosts_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_hosts_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_hosts_cookie_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_hosts_cookie_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_hosts_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_list_hosts_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_hosts_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_hosts_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_list_hosts_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_hosts_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_list_hosts_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_list_hosts_reply_t {
    pub response_type: u8,
    pub mode: u8,
    pub sequence: u16,
    pub length: u32,
    pub hosts_len: u16,
    pub pad0: [u8; 22usize],
}
#[test]
fn bindgen_test_layout_xcb_list_hosts_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_list_hosts_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_list_hosts_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_list_hosts_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_list_hosts_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_list_hosts_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_reply_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hosts_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_reply_t),
            "::",
            stringify!(hosts_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_list_hosts_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub const xcb_access_control_t_XCB_ACCESS_CONTROL_DISABLE: xcb_access_control_t = 0;
pub const xcb_access_control_t_XCB_ACCESS_CONTROL_ENABLE: xcb_access_control_t = 1;
pub type xcb_access_control_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_access_control_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_set_access_control_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_access_control_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_access_control_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_set_access_control_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_access_control_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_access_control_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_access_control_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_access_control_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_access_control_request_t),
            "::",
            stringify!(length)
        )
    );
}
pub const xcb_close_down_t_XCB_CLOSE_DOWN_DESTROY_ALL: xcb_close_down_t = 0;
pub const xcb_close_down_t_XCB_CLOSE_DOWN_RETAIN_PERMANENT: xcb_close_down_t = 1;
pub const xcb_close_down_t_XCB_CLOSE_DOWN_RETAIN_TEMPORARY: xcb_close_down_t = 2;
pub type xcb_close_down_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_close_down_mode_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_set_close_down_mode_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_close_down_mode_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_close_down_mode_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_set_close_down_mode_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_close_down_mode_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_close_down_mode_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_close_down_mode_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_close_down_mode_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_close_down_mode_request_t),
            "::",
            stringify!(length)
        )
    );
}
pub const xcb_kill_t_XCB_KILL_ALL_TEMPORARY: xcb_kill_t = 0;
pub type xcb_kill_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_kill_client_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub resource: u32,
}
#[test]
fn bindgen_test_layout_xcb_kill_client_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_kill_client_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_kill_client_request_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_kill_client_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_kill_client_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_kill_client_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_kill_client_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_kill_client_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_kill_client_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resource) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_kill_client_request_t),
            "::",
            stringify!(resource)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_rotate_properties_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub atoms_len: u16,
    pub delta: i16,
}
#[test]
fn bindgen_test_layout_xcb_rotate_properties_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_rotate_properties_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_rotate_properties_request_t>(),
        12usize,
        concat!("Size of: ", stringify!(xcb_rotate_properties_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_rotate_properties_request_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_rotate_properties_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rotate_properties_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rotate_properties_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rotate_properties_request_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rotate_properties_request_t),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atoms_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rotate_properties_request_t),
            "::",
            stringify!(atoms_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).delta) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_rotate_properties_request_t),
            "::",
            stringify!(delta)
        )
    );
}
pub const xcb_screen_saver_t_XCB_SCREEN_SAVER_RESET: xcb_screen_saver_t = 0;
pub const xcb_screen_saver_t_XCB_SCREEN_SAVER_ACTIVE: xcb_screen_saver_t = 1;
pub type xcb_screen_saver_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_force_screen_saver_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_force_screen_saver_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_force_screen_saver_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_force_screen_saver_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_force_screen_saver_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_force_screen_saver_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_force_screen_saver_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_force_screen_saver_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_force_screen_saver_request_t),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_force_screen_saver_request_t),
            "::",
            stringify!(length)
        )
    );
}
pub const xcb_mapping_status_t_XCB_MAPPING_STATUS_SUCCESS: xcb_mapping_status_t = 0;
pub const xcb_mapping_status_t_XCB_MAPPING_STATUS_BUSY: xcb_mapping_status_t = 1;
pub const xcb_mapping_status_t_XCB_MAPPING_STATUS_FAILURE: xcb_mapping_status_t = 2;
pub type xcb_mapping_status_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_pointer_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_set_pointer_mapping_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_pointer_mapping_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_pointer_mapping_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_set_pointer_mapping_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_pointer_mapping_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_pointer_mapping_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_pointer_mapping_request_t {
    pub major_opcode: u8,
    pub map_len: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_set_pointer_mapping_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_pointer_mapping_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_pointer_mapping_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_set_pointer_mapping_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_pointer_mapping_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_pointer_mapping_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).map_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_request_t),
            "::",
            stringify!(map_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_pointer_mapping_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}
#[test]
fn bindgen_test_layout_xcb_set_pointer_mapping_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_pointer_mapping_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_pointer_mapping_reply_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_set_pointer_mapping_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_pointer_mapping_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_set_pointer_mapping_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_reply_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_pointer_mapping_reply_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_pointer_mapping_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_pointer_mapping_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_pointer_mapping_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_pointer_mapping_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_pointer_mapping_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_pointer_mapping_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_pointer_mapping_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_pointer_mapping_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_pointer_mapping_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_pointer_mapping_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_pointer_mapping_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_pointer_mapping_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_pointer_mapping_reply_t {
    pub response_type: u8,
    pub map_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24usize],
}
#[test]
fn bindgen_test_layout_xcb_get_pointer_mapping_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_pointer_mapping_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_pointer_mapping_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_pointer_mapping_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_pointer_mapping_reply_t>(),
        4usize,
        concat!("Alignment of ", stringify!(xcb_get_pointer_mapping_reply_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).map_len) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_reply_t),
            "::",
            stringify!(map_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_pointer_mapping_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
pub const xcb_map_index_t_XCB_MAP_INDEX_SHIFT: xcb_map_index_t = 0;
pub const xcb_map_index_t_XCB_MAP_INDEX_LOCK: xcb_map_index_t = 1;
pub const xcb_map_index_t_XCB_MAP_INDEX_CONTROL: xcb_map_index_t = 2;
pub const xcb_map_index_t_XCB_MAP_INDEX_1: xcb_map_index_t = 3;
pub const xcb_map_index_t_XCB_MAP_INDEX_2: xcb_map_index_t = 4;
pub const xcb_map_index_t_XCB_MAP_INDEX_3: xcb_map_index_t = 5;
pub const xcb_map_index_t_XCB_MAP_INDEX_4: xcb_map_index_t = 6;
pub const xcb_map_index_t_XCB_MAP_INDEX_5: xcb_map_index_t = 7;
pub type xcb_map_index_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_modifier_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_set_modifier_mapping_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_modifier_mapping_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_modifier_mapping_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_set_modifier_mapping_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_modifier_mapping_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_modifier_mapping_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub keycodes_per_modifier: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_set_modifier_mapping_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_modifier_mapping_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_modifier_mapping_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_set_modifier_mapping_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_modifier_mapping_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_modifier_mapping_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keycodes_per_modifier) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_request_t),
            "::",
            stringify!(keycodes_per_modifier)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_set_modifier_mapping_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}
#[test]
fn bindgen_test_layout_xcb_set_modifier_mapping_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_set_modifier_mapping_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_set_modifier_mapping_reply_t>(),
        8usize,
        concat!("Size of: ", stringify!(xcb_set_modifier_mapping_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_set_modifier_mapping_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_set_modifier_mapping_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_reply_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_set_modifier_mapping_reply_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_modifier_mapping_cookie_t {
    pub sequence: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_xcb_get_modifier_mapping_cookie_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_modifier_mapping_cookie_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_modifier_mapping_cookie_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_modifier_mapping_cookie_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_modifier_mapping_cookie_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_modifier_mapping_cookie_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_cookie_t),
            "::",
            stringify!(sequence)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_get_modifier_mapping_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_modifier_mapping_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_modifier_mapping_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_get_modifier_mapping_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_modifier_mapping_request_t>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_modifier_mapping_request_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_request_t),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type: u8,
    pub keycodes_per_modifier: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24usize],
}
#[test]
fn bindgen_test_layout_xcb_get_modifier_mapping_reply_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_get_modifier_mapping_reply_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_get_modifier_mapping_reply_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_get_modifier_mapping_reply_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_get_modifier_mapping_reply_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(xcb_get_modifier_mapping_reply_t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response_type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_reply_t),
            "::",
            stringify!(response_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keycodes_per_modifier) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_reply_t),
            "::",
            stringify!(keycodes_per_modifier)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sequence) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_reply_t),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_reply_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_get_modifier_mapping_reply_t),
            "::",
            stringify!(pad0)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_no_operation_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}
#[test]
fn bindgen_test_layout_xcb_no_operation_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_no_operation_request_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_no_operation_request_t>(),
        4usize,
        concat!("Size of: ", stringify!(xcb_no_operation_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_no_operation_request_t>(),
        2usize,
        concat!("Alignment of ", stringify!(xcb_no_operation_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).major_opcode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_operation_request_t),
            "::",
            stringify!(major_opcode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_operation_request_t),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_no_operation_request_t),
            "::",
            stringify!(length)
        )
    );
}
extern "C" {
    pub fn xcb_char2b_next(i: *mut xcb_char2b_iterator_t);
}
extern "C" {
    pub fn xcb_char2b_end(i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_window_next(i: *mut xcb_window_iterator_t);
}
extern "C" {
    pub fn xcb_window_end(i: xcb_window_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_pixmap_next(i: *mut xcb_pixmap_iterator_t);
}
extern "C" {
    pub fn xcb_pixmap_end(i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_cursor_next(i: *mut xcb_cursor_iterator_t);
}
extern "C" {
    pub fn xcb_cursor_end(i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_font_next(i: *mut xcb_font_iterator_t);
}
extern "C" {
    pub fn xcb_font_end(i: xcb_font_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_gcontext_next(i: *mut xcb_gcontext_iterator_t);
}
extern "C" {
    pub fn xcb_gcontext_end(i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_colormap_next(i: *mut xcb_colormap_iterator_t);
}
extern "C" {
    pub fn xcb_colormap_end(i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_atom_next(i: *mut xcb_atom_iterator_t);
}
extern "C" {
    pub fn xcb_atom_end(i: xcb_atom_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_drawable_next(i: *mut xcb_drawable_iterator_t);
}
extern "C" {
    pub fn xcb_drawable_end(i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_fontable_next(i: *mut xcb_fontable_iterator_t);
}
extern "C" {
    pub fn xcb_fontable_end(i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_bool32_next(i: *mut xcb_bool32_iterator_t);
}
extern "C" {
    pub fn xcb_bool32_end(i: xcb_bool32_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_visualid_next(i: *mut xcb_visualid_iterator_t);
}
extern "C" {
    pub fn xcb_visualid_end(i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_timestamp_next(i: *mut xcb_timestamp_iterator_t);
}
extern "C" {
    pub fn xcb_timestamp_end(i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_keysym_next(i: *mut xcb_keysym_iterator_t);
}
extern "C" {
    pub fn xcb_keysym_end(i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_keycode_next(i: *mut xcb_keycode_iterator_t);
}
extern "C" {
    pub fn xcb_keycode_end(i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_keycode32_next(i: *mut xcb_keycode32_iterator_t);
}
extern "C" {
    pub fn xcb_keycode32_end(i: xcb_keycode32_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_button_next(i: *mut xcb_button_iterator_t);
}
extern "C" {
    pub fn xcb_button_end(i: xcb_button_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_point_next(i: *mut xcb_point_iterator_t);
}
extern "C" {
    pub fn xcb_point_end(i: xcb_point_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_rectangle_next(i: *mut xcb_rectangle_iterator_t);
}
extern "C" {
    pub fn xcb_rectangle_end(i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_arc_next(i: *mut xcb_arc_iterator_t);
}
extern "C" {
    pub fn xcb_arc_end(i: xcb_arc_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_format_next(i: *mut xcb_format_iterator_t);
}
extern "C" {
    pub fn xcb_format_end(i: xcb_format_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);
}
extern "C" {
    pub fn xcb_visualtype_end(i: xcb_visualtype_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_depth_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_depth_visuals(R: *const xcb_depth_t) -> *mut xcb_visualtype_t;
}
extern "C" {
    pub fn xcb_depth_visuals_length(R: *const xcb_depth_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;
}
extern "C" {
    pub fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
}
extern "C" {
    pub fn xcb_depth_end(i: xcb_depth_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_screen_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_screen_allowed_depths_length(R: *const xcb_screen_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t) -> xcb_depth_iterator_t;
}
extern "C" {
    pub fn xcb_screen_next(i: *mut xcb_screen_iterator_t);
}
extern "C" {
    pub fn xcb_screen_end(i: xcb_screen_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_request_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_request_authorization_protocol_name(
        R: *const xcb_setup_request_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_setup_request_authorization_protocol_name_length(
        R: *const xcb_setup_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_request_authorization_protocol_name_end(
        R: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_request_authorization_protocol_data(
        R: *const xcb_setup_request_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_setup_request_authorization_protocol_data_length(
        R: *const xcb_setup_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_request_authorization_protocol_data_end(
        R: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_request_next(i: *mut xcb_setup_request_iterator_t);
}
extern "C" {
    pub fn xcb_setup_request_end(i: xcb_setup_request_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_failed_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_failed_reason(R: *const xcb_setup_failed_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_setup_failed_reason_length(R: *const xcb_setup_failed_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_failed_reason_end(R: *const xcb_setup_failed_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_failed_next(i: *mut xcb_setup_failed_iterator_t);
}
extern "C" {
    pub fn xcb_setup_failed_end(i: xcb_setup_failed_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_authenticate_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_authenticate_reason(
        R: *const xcb_setup_authenticate_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_setup_authenticate_reason_length(
        R: *const xcb_setup_authenticate_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_authenticate_reason_end(
        R: *const xcb_setup_authenticate_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_authenticate_next(i: *mut xcb_setup_authenticate_iterator_t);
}
extern "C" {
    pub fn xcb_setup_authenticate_end(
        i: xcb_setup_authenticate_iterator_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_vendor(R: *const xcb_setup_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_setup_vendor_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_vendor_end(R: *const xcb_setup_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_setup_pixmap_formats(R: *const xcb_setup_t) -> *mut xcb_format_t;
}
extern "C" {
    pub fn xcb_setup_pixmap_formats_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_pixmap_formats_iterator(R: *const xcb_setup_t) -> xcb_format_iterator_t;
}
extern "C" {
    pub fn xcb_setup_roots_length(R: *const xcb_setup_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
}
extern "C" {
    pub fn xcb_setup_next(i: *mut xcb_setup_iterator_t);
}
extern "C" {
    pub fn xcb_setup_end(i: xcb_setup_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_client_message_data_next(i: *mut xcb_client_message_data_iterator_t);
}
extern "C" {
    pub fn xcb_client_message_data_end(
        i: xcb_client_message_data_iterator_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_create_window_value_list_serialize(
        _buffer: *mut *mut ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *const xcb_create_window_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_window_value_list_unpack(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *mut xcb_create_window_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_window_value_list_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_window_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_window_checked(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_window_aux_checked(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_create_window_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_window_aux(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_create_window_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_window_value_list(
        R: *const xcb_create_window_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_change_window_attributes_value_list_serialize(
        _buffer: *mut *mut ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *const xcb_change_window_attributes_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_window_attributes_value_list_unpack(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *mut xcb_change_window_attributes_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_window_attributes_value_list_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_window_attributes_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_window_attributes_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_window_attributes_aux_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const xcb_change_window_attributes_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_window_attributes_aux(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const xcb_change_window_attributes_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_window_attributes_value_list(
        R: *const xcb_change_window_attributes_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_get_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
}
extern "C" {
    pub fn xcb_get_window_attributes_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
}
extern "C" {
    pub fn xcb_get_window_attributes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_window_attributes_reply_t;
}
extern "C" {
    pub fn xcb_destroy_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_destroy_subwindows_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_destroy_subwindows(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_save_set_checked(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_save_set(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_reparent_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_map_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_map_subwindows_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_map_subwindows(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_unmap_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_unmap_subwindows_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_unmap_subwindows(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_configure_window_value_list_serialize(
        _buffer: *mut *mut ::std::os::raw::c_void,
        value_mask: u16,
        _aux: *const xcb_configure_window_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_configure_window_value_list_unpack(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u16,
        _aux: *mut xcb_configure_window_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_configure_window_value_list_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u16,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_configure_window_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_configure_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_configure_window_aux_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const xcb_configure_window_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_configure_window_aux(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const xcb_configure_window_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_configure_window_value_list(
        R: *const xcb_configure_window_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_circulate_window_checked(
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_circulate_window(
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
}
extern "C" {
    pub fn xcb_get_geometry_unchecked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
}
extern "C" {
    pub fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
}
extern "C" {
    pub fn xcb_query_tree_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_tree(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
}
extern "C" {
    pub fn xcb_query_tree_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
}
extern "C" {
    pub fn xcb_query_tree_children(R: *const xcb_query_tree_reply_t) -> *mut xcb_window_t;
}
extern "C" {
    pub fn xcb_query_tree_children_length(
        R: *const xcb_query_tree_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_tree_children_end(R: *const xcb_query_tree_reply_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_query_tree_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_tree_reply_t;
}
extern "C" {
    pub fn xcb_intern_atom_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_intern_atom_cookie_t;
}
extern "C" {
    pub fn xcb_intern_atom_unchecked(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_intern_atom_cookie_t;
}
extern "C" {
    pub fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
}
extern "C" {
    pub fn xcb_get_atom_name_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_atom_name(
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t;
}
extern "C" {
    pub fn xcb_get_atom_name_unchecked(
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t;
}
extern "C" {
    pub fn xcb_get_atom_name_name(
        R: *const xcb_get_atom_name_reply_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_get_atom_name_name_length(
        R: *const xcb_get_atom_name_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_atom_name_name_end(
        R: *const xcb_get_atom_name_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_atom_name_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_atom_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_atom_name_reply_t;
}
extern "C" {
    pub fn xcb_change_property_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_property_checked(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_property_data(
        R: *const xcb_change_property_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_change_property_data_length(
        R: *const xcb_change_property_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_property_data_end(
        R: *const xcb_change_property_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_delete_property_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_delete_property(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_get_property_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t;
}
extern "C" {
    pub fn xcb_get_property_unchecked(
        c: *mut xcb_connection_t,
        _delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t;
}
extern "C" {
    pub fn xcb_get_property_value(
        R: *const xcb_get_property_reply_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_get_property_value_length(
        R: *const xcb_get_property_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_property_value_end(R: *const xcb_get_property_reply_t)
        -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
}
extern "C" {
    pub fn xcb_list_properties_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_properties(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t;
}
extern "C" {
    pub fn xcb_list_properties_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t;
}
extern "C" {
    pub fn xcb_list_properties_atoms(R: *const xcb_list_properties_reply_t) -> *mut xcb_atom_t;
}
extern "C" {
    pub fn xcb_list_properties_atoms_length(
        R: *const xcb_list_properties_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_properties_atoms_end(
        R: *const xcb_list_properties_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_list_properties_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_properties_reply_t;
}
extern "C" {
    pub fn xcb_set_selection_owner_checked(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_selection_owner(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_get_selection_owner(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
}
extern "C" {
    pub fn xcb_get_selection_owner_unchecked(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
}
extern "C" {
    pub fn xcb_get_selection_owner_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_selection_owner_reply_t;
}
extern "C" {
    pub fn xcb_convert_selection_checked(
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_convert_selection(
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_send_event_checked(
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_pointer(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
}
extern "C" {
    pub fn xcb_grab_pointer_unchecked(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
}
extern "C" {
    pub fn xcb_grab_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_pointer_reply_t;
}
extern "C" {
    pub fn xcb_ungrab_pointer_checked(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_pointer(c: *mut xcb_connection_t, time: xcb_timestamp_t)
        -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_button_checked(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_button(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_button_checked(
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_button(
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_active_pointer_grab_checked(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_active_pointer_grab(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_keyboard(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t;
}
extern "C" {
    pub fn xcb_grab_keyboard_unchecked(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t;
}
extern "C" {
    pub fn xcb_grab_keyboard_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_keyboard_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_keyboard_reply_t;
}
extern "C" {
    pub fn xcb_ungrab_keyboard_checked(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_keyboard(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_key_checked(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_key(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_key_checked(
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_key(
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_allow_events_checked(
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_allow_events(
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_server_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_server_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_query_pointer(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
}
extern "C" {
    pub fn xcb_query_pointer_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
}
extern "C" {
    pub fn xcb_query_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t;
}
extern "C" {
    pub fn xcb_timecoord_next(i: *mut xcb_timecoord_iterator_t);
}
extern "C" {
    pub fn xcb_timecoord_end(i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_motion_events_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_motion_events(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t;
}
extern "C" {
    pub fn xcb_get_motion_events_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t;
}
extern "C" {
    pub fn xcb_get_motion_events_events(
        R: *const xcb_get_motion_events_reply_t,
    ) -> *mut xcb_timecoord_t;
}
extern "C" {
    pub fn xcb_get_motion_events_events_length(
        R: *const xcb_get_motion_events_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_motion_events_events_iterator(
        R: *const xcb_get_motion_events_reply_t,
    ) -> xcb_timecoord_iterator_t;
}
extern "C" {
    pub fn xcb_get_motion_events_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_motion_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_motion_events_reply_t;
}
extern "C" {
    pub fn xcb_translate_coordinates(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t;
}
extern "C" {
    pub fn xcb_translate_coordinates_unchecked(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t;
}
extern "C" {
    pub fn xcb_translate_coordinates_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_translate_coordinates_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_translate_coordinates_reply_t;
}
extern "C" {
    pub fn xcb_warp_pointer_checked(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_warp_pointer(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_input_focus_checked(
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_input_focus(
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
}
extern "C" {
    pub fn xcb_get_input_focus_unchecked(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
}
extern "C" {
    pub fn xcb_get_input_focus_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_input_focus_reply_t;
}
extern "C" {
    pub fn xcb_query_keymap(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t;
}
extern "C" {
    pub fn xcb_query_keymap_unchecked(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t;
}
extern "C" {
    pub fn xcb_query_keymap_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_keymap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_keymap_reply_t;
}
extern "C" {
    pub fn xcb_open_font_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_open_font_checked(
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_open_font(
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_open_font_name(R: *const xcb_open_font_request_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_open_font_name_length(R: *const xcb_open_font_request_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_open_font_name_end(R: *const xcb_open_font_request_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_close_font_checked(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_close_font(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_fontprop_next(i: *mut xcb_fontprop_iterator_t);
}
extern "C" {
    pub fn xcb_fontprop_end(i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_charinfo_next(i: *mut xcb_charinfo_iterator_t);
}
extern "C" {
    pub fn xcb_charinfo_end(i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_query_font_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_font(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t;
}
extern "C" {
    pub fn xcb_query_font_unchecked(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t;
}
extern "C" {
    pub fn xcb_query_font_properties(R: *const xcb_query_font_reply_t) -> *mut xcb_fontprop_t;
}
extern "C" {
    pub fn xcb_query_font_properties_length(
        R: *const xcb_query_font_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_font_properties_iterator(
        R: *const xcb_query_font_reply_t,
    ) -> xcb_fontprop_iterator_t;
}
extern "C" {
    pub fn xcb_query_font_char_infos(R: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t;
}
extern "C" {
    pub fn xcb_query_font_char_infos_length(
        R: *const xcb_query_font_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_font_char_infos_iterator(
        R: *const xcb_query_font_reply_t,
    ) -> xcb_charinfo_iterator_t;
}
extern "C" {
    pub fn xcb_query_font_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_font_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_font_reply_t;
}
extern "C" {
    pub fn xcb_query_text_extents_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        string_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_text_extents(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t;
}
extern "C" {
    pub fn xcb_query_text_extents_unchecked(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t;
}
extern "C" {
    pub fn xcb_query_text_extents_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_text_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_text_extents_reply_t;
}
extern "C" {
    pub fn xcb_str_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_str_name(R: *const xcb_str_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_str_name_length(R: *const xcb_str_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_str_name_end(R: *const xcb_str_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_str_next(i: *mut xcb_str_iterator_t);
}
extern "C" {
    pub fn xcb_str_end(i: xcb_str_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_list_fonts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_fonts(
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_cookie_t;
}
extern "C" {
    pub fn xcb_list_fonts_unchecked(
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_cookie_t;
}
extern "C" {
    pub fn xcb_list_fonts_names_length(R: *const xcb_list_fonts_reply_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_fonts_names_iterator(R: *const xcb_list_fonts_reply_t) -> xcb_str_iterator_t;
}
extern "C" {
    pub fn xcb_list_fonts_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_fonts_reply_t;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_fonts_with_info(
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_with_info_cookie_t;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_unchecked(
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const ::std::os::raw::c_char,
    ) -> xcb_list_fonts_with_info_cookie_t;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_properties(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut xcb_fontprop_t;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_properties_length(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_properties_iterator(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_fontprop_iterator_t;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_name(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_name_length(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_name_end(
        R: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_list_fonts_with_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_with_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_fonts_with_info_reply_t;
}
extern "C" {
    pub fn xcb_set_font_path_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_font_path_checked(
        c: *mut xcb_connection_t,
        font_qty: u16,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_font_path(
        c: *mut xcb_connection_t,
        font_qty: u16,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_font_path_font_length(
        R: *const xcb_set_font_path_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_font_path_font_iterator(
        R: *const xcb_set_font_path_request_t,
    ) -> xcb_str_iterator_t;
}
extern "C" {
    pub fn xcb_get_font_path_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_font_path(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t;
}
extern "C" {
    pub fn xcb_get_font_path_unchecked(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t;
}
extern "C" {
    pub fn xcb_get_font_path_path_length(
        R: *const xcb_get_font_path_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_font_path_path_iterator(
        R: *const xcb_get_font_path_reply_t,
    ) -> xcb_str_iterator_t;
}
extern "C" {
    pub fn xcb_get_font_path_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_font_path_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_font_path_reply_t;
}
extern "C" {
    pub fn xcb_create_pixmap_checked(
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_pixmap(
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_pixmap_checked(
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_gc_value_list_serialize(
        _buffer: *mut *mut ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *const xcb_create_gc_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_gc_value_list_unpack(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *mut xcb_create_gc_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_gc_value_list_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_gc_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_create_gc_checked(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_gc(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_gc_aux_checked(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const xcb_create_gc_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_gc_aux(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const xcb_create_gc_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_gc_value_list(
        R: *const xcb_create_gc_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_change_gc_value_list_serialize(
        _buffer: *mut *mut ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *const xcb_change_gc_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_gc_value_list_unpack(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *mut xcb_change_gc_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_gc_value_list_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_gc_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_gc_checked(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_gc_aux_checked(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const xcb_change_gc_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_gc_aux(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const xcb_change_gc_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_gc_value_list(
        R: *const xcb_change_gc_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_copy_gc_checked(
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: u32,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_gc(
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: u32,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_dashes_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_dashes_checked(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: u16,
        dashes_len: u16,
        dashes: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_dashes(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: u16,
        dashes_len: u16,
        dashes: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_dashes_dashes(R: *const xcb_set_dashes_request_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_set_dashes_dashes_length(
        R: *const xcb_set_dashes_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_dashes_dashes_end(R: *const xcb_set_dashes_request_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_set_clip_rectangles_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        rectangles_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_clip_rectangles_checked(
        c: *mut xcb_connection_t,
        ordering: u8,
        gc: xcb_gcontext_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_clip_rectangles(
        c: *mut xcb_connection_t,
        ordering: u8,
        gc: xcb_gcontext_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_clip_rectangles_rectangles(
        R: *const xcb_set_clip_rectangles_request_t,
    ) -> *mut xcb_rectangle_t;
}
extern "C" {
    pub fn xcb_set_clip_rectangles_rectangles_length(
        R: *const xcb_set_clip_rectangles_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_clip_rectangles_rectangles_iterator(
        R: *const xcb_set_clip_rectangles_request_t,
    ) -> xcb_rectangle_iterator_t;
}
extern "C" {
    pub fn xcb_free_gc_checked(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_clear_area_checked(
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_clear_area(
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_area_checked(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_area(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_plane_checked(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_plane(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_point_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        points_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_point_checked(
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_point(
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_point_points(R: *const xcb_poly_point_request_t) -> *mut xcb_point_t;
}
extern "C" {
    pub fn xcb_poly_point_points_length(
        R: *const xcb_poly_point_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_point_points_iterator(
        R: *const xcb_poly_point_request_t,
    ) -> xcb_point_iterator_t;
}
extern "C" {
    pub fn xcb_poly_line_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        points_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_line_checked(
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_line(
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_line_points(R: *const xcb_poly_line_request_t) -> *mut xcb_point_t;
}
extern "C" {
    pub fn xcb_poly_line_points_length(R: *const xcb_poly_line_request_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_line_points_iterator(R: *const xcb_poly_line_request_t)
        -> xcb_point_iterator_t;
}
extern "C" {
    pub fn xcb_segment_next(i: *mut xcb_segment_iterator_t);
}
extern "C" {
    pub fn xcb_segment_end(i: xcb_segment_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_poly_segment_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        segments_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_segment_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: u32,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_segment(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: u32,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_segment_segments(R: *const xcb_poly_segment_request_t) -> *mut xcb_segment_t;
}
extern "C" {
    pub fn xcb_poly_segment_segments_length(
        R: *const xcb_poly_segment_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_segment_segments_iterator(
        R: *const xcb_poly_segment_request_t,
    ) -> xcb_segment_iterator_t;
}
extern "C" {
    pub fn xcb_poly_rectangle_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        rectangles_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_rectangle_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_rectangle_rectangles(
        R: *const xcb_poly_rectangle_request_t,
    ) -> *mut xcb_rectangle_t;
}
extern "C" {
    pub fn xcb_poly_rectangle_rectangles_length(
        R: *const xcb_poly_rectangle_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_rectangle_rectangles_iterator(
        R: *const xcb_poly_rectangle_request_t,
    ) -> xcb_rectangle_iterator_t;
}
extern "C" {
    pub fn xcb_poly_arc_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        arcs_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_arc_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_arc(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_arc_arcs(R: *const xcb_poly_arc_request_t) -> *mut xcb_arc_t;
}
extern "C" {
    pub fn xcb_poly_arc_arcs_length(R: *const xcb_poly_arc_request_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_arc_arcs_iterator(R: *const xcb_poly_arc_request_t) -> xcb_arc_iterator_t;
}
extern "C" {
    pub fn xcb_fill_poly_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        points_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_fill_poly_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: u8,
        coordinate_mode: u8,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_fill_poly(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: u8,
        coordinate_mode: u8,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_fill_poly_points(R: *const xcb_fill_poly_request_t) -> *mut xcb_point_t;
}
extern "C" {
    pub fn xcb_fill_poly_points_length(R: *const xcb_fill_poly_request_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_fill_poly_points_iterator(R: *const xcb_fill_poly_request_t)
        -> xcb_point_iterator_t;
}
extern "C" {
    pub fn xcb_poly_fill_rectangle_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        rectangles_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_fill_rectangle_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_fill_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_fill_rectangle_rectangles(
        R: *const xcb_poly_fill_rectangle_request_t,
    ) -> *mut xcb_rectangle_t;
}
extern "C" {
    pub fn xcb_poly_fill_rectangle_rectangles_length(
        R: *const xcb_poly_fill_rectangle_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_fill_rectangle_rectangles_iterator(
        R: *const xcb_poly_fill_rectangle_request_t,
    ) -> xcb_rectangle_iterator_t;
}
extern "C" {
    pub fn xcb_poly_fill_arc_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        arcs_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_fill_arc_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_fill_arc(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_fill_arc_arcs(R: *const xcb_poly_fill_arc_request_t) -> *mut xcb_arc_t;
}
extern "C" {
    pub fn xcb_poly_fill_arc_arcs_length(
        R: *const xcb_poly_fill_arc_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_fill_arc_arcs_iterator(
        R: *const xcb_poly_fill_arc_request_t,
    ) -> xcb_arc_iterator_t;
}
extern "C" {
    pub fn xcb_put_image_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        data_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_put_image_checked(
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_put_image(
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_put_image_data(R: *const xcb_put_image_request_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_put_image_data_length(R: *const xcb_put_image_request_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_put_image_data_end(R: *const xcb_put_image_request_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_image_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_image(
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t;
}
extern "C" {
    pub fn xcb_get_image_unchecked(
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t;
}
extern "C" {
    pub fn xcb_get_image_data(R: *const xcb_get_image_reply_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_get_image_data_length(R: *const xcb_get_image_reply_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_image_data_end(R: *const xcb_get_image_reply_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_image_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_image_reply_t;
}
extern "C" {
    pub fn xcb_poly_text_8_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        items_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_text_8_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_text_8(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_text_8_items(R: *const xcb_poly_text_8_request_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_poly_text_8_items_length(
        R: *const xcb_poly_text_8_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_text_8_items_end(R: *const xcb_poly_text_8_request_t)
        -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_poly_text_16_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        items_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_text_16_checked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_text_16(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_poly_text_16_items(R: *const xcb_poly_text_16_request_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_poly_text_16_items_length(
        R: *const xcb_poly_text_16_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_poly_text_16_items_end(
        R: *const xcb_poly_text_16_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_image_text_8_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_image_text_8_checked(
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_image_text_8(
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_image_text_8_string(
        R: *const xcb_image_text_8_request_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_image_text_8_string_length(
        R: *const xcb_image_text_8_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_image_text_8_string_end(
        R: *const xcb_image_text_8_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_image_text_16_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_image_text_16_checked(
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_image_text_16(
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_image_text_16_string(R: *const xcb_image_text_16_request_t) -> *mut xcb_char2b_t;
}
extern "C" {
    pub fn xcb_image_text_16_string_length(
        R: *const xcb_image_text_16_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_image_text_16_string_iterator(
        R: *const xcb_image_text_16_request_t,
    ) -> xcb_char2b_iterator_t;
}
extern "C" {
    pub fn xcb_create_colormap_checked(
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_colormap(
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_colormap_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_colormap(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_colormap_and_free_checked(
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_copy_colormap_and_free(
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_install_colormap_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_install_colormap(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_uninstall_colormap_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_uninstall_colormap(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_list_installed_colormaps_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_installed_colormaps(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t;
}
extern "C" {
    pub fn xcb_list_installed_colormaps_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t;
}
extern "C" {
    pub fn xcb_list_installed_colormaps_cmaps(
        R: *const xcb_list_installed_colormaps_reply_t,
    ) -> *mut xcb_colormap_t;
}
extern "C" {
    pub fn xcb_list_installed_colormaps_cmaps_length(
        R: *const xcb_list_installed_colormaps_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_installed_colormaps_cmaps_end(
        R: *const xcb_list_installed_colormaps_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_list_installed_colormaps_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_installed_colormaps_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_installed_colormaps_reply_t;
}
extern "C" {
    pub fn xcb_alloc_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_color_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_reply_t;
}
extern "C" {
    pub fn xcb_alloc_named_color_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_alloc_named_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_alloc_named_color_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_named_color_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_alloc_named_color_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_named_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_named_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_named_color_reply_t;
}
extern "C" {
    pub fn xcb_alloc_color_cells_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_alloc_color_cells(
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        planes: u16,
    ) -> xcb_alloc_color_cells_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_color_cells_unchecked(
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        planes: u16,
    ) -> xcb_alloc_color_cells_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_color_cells_pixels(R: *const xcb_alloc_color_cells_reply_t) -> *mut u32;
}
extern "C" {
    pub fn xcb_alloc_color_cells_pixels_length(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_alloc_color_cells_pixels_end(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_alloc_color_cells_masks(R: *const xcb_alloc_color_cells_reply_t) -> *mut u32;
}
extern "C" {
    pub fn xcb_alloc_color_cells_masks_length(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_alloc_color_cells_masks_end(
        R: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_alloc_color_cells_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cells_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_cells_reply_t;
}
extern "C" {
    pub fn xcb_alloc_color_planes_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_alloc_color_planes(
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
    ) -> xcb_alloc_color_planes_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_color_planes_unchecked(
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
    ) -> xcb_alloc_color_planes_cookie_t;
}
extern "C" {
    pub fn xcb_alloc_color_planes_pixels(R: *const xcb_alloc_color_planes_reply_t) -> *mut u32;
}
extern "C" {
    pub fn xcb_alloc_color_planes_pixels_length(
        R: *const xcb_alloc_color_planes_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_alloc_color_planes_pixels_end(
        R: *const xcb_alloc_color_planes_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_alloc_color_planes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_planes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_planes_reply_t;
}
extern "C" {
    pub fn xcb_free_colors_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        pixels_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_free_colors_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: u32,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_colors(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: u32,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_colors_pixels(R: *const xcb_free_colors_request_t) -> *mut u32;
}
extern "C" {
    pub fn xcb_free_colors_pixels_length(
        R: *const xcb_free_colors_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_free_colors_pixels_end(
        R: *const xcb_free_colors_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_coloritem_next(i: *mut xcb_coloritem_iterator_t);
}
extern "C" {
    pub fn xcb_coloritem_end(i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_store_colors_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        items_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_store_colors_checked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: u32,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_store_colors(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: u32,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_store_colors_items(R: *const xcb_store_colors_request_t) -> *mut xcb_coloritem_t;
}
extern "C" {
    pub fn xcb_store_colors_items_length(
        R: *const xcb_store_colors_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_store_colors_items_iterator(
        R: *const xcb_store_colors_request_t,
    ) -> xcb_coloritem_iterator_t;
}
extern "C" {
    pub fn xcb_store_named_color_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_store_named_color_checked(
        c: *mut xcb_connection_t,
        flags: u8,
        cmap: xcb_colormap_t,
        pixel: u32,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_store_named_color(
        c: *mut xcb_connection_t,
        flags: u8,
        cmap: xcb_colormap_t,
        pixel: u32,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_store_named_color_name(
        R: *const xcb_store_named_color_request_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xcb_store_named_color_name_length(
        R: *const xcb_store_named_color_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_store_named_color_name_end(
        R: *const xcb_store_named_color_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_rgb_next(i: *mut xcb_rgb_iterator_t);
}
extern "C" {
    pub fn xcb_rgb_end(i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_query_colors_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        pixels_len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_colors(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_query_colors_cookie_t;
}
extern "C" {
    pub fn xcb_query_colors_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_query_colors_cookie_t;
}
extern "C" {
    pub fn xcb_query_colors_colors(R: *const xcb_query_colors_reply_t) -> *mut xcb_rgb_t;
}
extern "C" {
    pub fn xcb_query_colors_colors_length(
        R: *const xcb_query_colors_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_colors_colors_iterator(
        R: *const xcb_query_colors_reply_t,
    ) -> xcb_rgb_iterator_t;
}
extern "C" {
    pub fn xcb_query_colors_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_colors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_colors_reply_t;
}
extern "C" {
    pub fn xcb_lookup_color_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_lookup_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_lookup_color_cookie_t;
}
extern "C" {
    pub fn xcb_lookup_color_unchecked(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_lookup_color_cookie_t;
}
extern "C" {
    pub fn xcb_lookup_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_lookup_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_lookup_color_reply_t;
}
extern "C" {
    pub fn xcb_create_cursor_checked(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_cursor(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_glyph_cursor_checked(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_create_glyph_cursor(
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_cursor_checked(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_free_cursor(c: *mut xcb_connection_t, cursor: xcb_cursor_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_recolor_cursor_checked(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_recolor_cursor(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_query_best_size(
        c: *mut xcb_connection_t,
        _class: u8,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_query_best_size_cookie_t;
}
extern "C" {
    pub fn xcb_query_best_size_unchecked(
        c: *mut xcb_connection_t,
        _class: u8,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_query_best_size_cookie_t;
}
extern "C" {
    pub fn xcb_query_best_size_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_best_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_best_size_reply_t;
}
extern "C" {
    pub fn xcb_query_extension_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_query_extension(
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_query_extension_cookie_t;
}
extern "C" {
    pub fn xcb_query_extension_unchecked(
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const ::std::os::raw::c_char,
    ) -> xcb_query_extension_cookie_t;
}
extern "C" {
    pub fn xcb_query_extension_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_extension_reply_t;
}
extern "C" {
    pub fn xcb_list_extensions_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_extensions(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t;
}
extern "C" {
    pub fn xcb_list_extensions_unchecked(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t;
}
extern "C" {
    pub fn xcb_list_extensions_names_length(
        R: *const xcb_list_extensions_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_extensions_names_iterator(
        R: *const xcb_list_extensions_reply_t,
    ) -> xcb_str_iterator_t;
}
extern "C" {
    pub fn xcb_list_extensions_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_extensions_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_extensions_reply_t;
}
extern "C" {
    pub fn xcb_change_keyboard_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_keyboard_mapping_checked(
        c: *mut xcb_connection_t,
        keycode_count: u8,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_keyboard_mapping(
        c: *mut xcb_connection_t,
        keycode_count: u8,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_keyboard_mapping_keysyms(
        R: *const xcb_change_keyboard_mapping_request_t,
    ) -> *mut xcb_keysym_t;
}
extern "C" {
    pub fn xcb_change_keyboard_mapping_keysyms_length(
        R: *const xcb_change_keyboard_mapping_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_keyboard_mapping_keysyms_end(
        R: *const xcb_change_keyboard_mapping_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping(
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping_unchecked(
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping_keysyms(
        R: *const xcb_get_keyboard_mapping_reply_t,
    ) -> *mut xcb_keysym_t;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping_keysyms_length(
        R: *const xcb_get_keyboard_mapping_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping_keysyms_end(
        R: *const xcb_get_keyboard_mapping_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_keyboard_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_keyboard_mapping_reply_t;
}
extern "C" {
    pub fn xcb_change_keyboard_control_value_list_serialize(
        _buffer: *mut *mut ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *const xcb_change_keyboard_control_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_keyboard_control_value_list_unpack(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
        _aux: *mut xcb_change_keyboard_control_value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_keyboard_control_value_list_sizeof(
        _buffer: *const ::std::os::raw::c_void,
        value_mask: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_keyboard_control_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_keyboard_control_checked(
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_keyboard_control(
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const ::std::os::raw::c_void,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_keyboard_control_aux_checked(
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const xcb_change_keyboard_control_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_keyboard_control_aux(
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const xcb_change_keyboard_control_value_list_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_keyboard_control_value_list(
        R: *const xcb_change_keyboard_control_request_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xcb_get_keyboard_control(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t;
}
extern "C" {
    pub fn xcb_get_keyboard_control_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_keyboard_control_cookie_t;
}
extern "C" {
    pub fn xcb_get_keyboard_control_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_keyboard_control_reply_t;
}
extern "C" {
    pub fn xcb_bell_checked(c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_bell(c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_pointer_control_checked(
        c: *mut xcb_connection_t,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_pointer_control(
        c: *mut xcb_connection_t,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_get_pointer_control(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t;
}
extern "C" {
    pub fn xcb_get_pointer_control_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_control_cookie_t;
}
extern "C" {
    pub fn xcb_get_pointer_control_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_pointer_control_reply_t;
}
extern "C" {
    pub fn xcb_set_screen_saver_checked(
        c: *mut xcb_connection_t,
        timeout: i16,
        interval: i16,
        prefer_blanking: u8,
        allow_exposures: u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_screen_saver(
        c: *mut xcb_connection_t,
        timeout: i16,
        interval: i16,
        prefer_blanking: u8,
        allow_exposures: u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_get_screen_saver(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t;
}
extern "C" {
    pub fn xcb_get_screen_saver_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_screen_saver_cookie_t;
}
extern "C" {
    pub fn xcb_get_screen_saver_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_screen_saver_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_screen_saver_reply_t;
}
extern "C" {
    pub fn xcb_change_hosts_sizeof(_buffer: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_hosts_checked(
        c: *mut xcb_connection_t,
        mode: u8,
        family: u8,
        address_len: u16,
        address: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_hosts(
        c: *mut xcb_connection_t,
        mode: u8,
        family: u8,
        address_len: u16,
        address: *const u8,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_change_hosts_address(R: *const xcb_change_hosts_request_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_change_hosts_address_length(
        R: *const xcb_change_hosts_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_change_hosts_address_end(
        R: *const xcb_change_hosts_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_host_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_host_address(R: *const xcb_host_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_host_address_length(R: *const xcb_host_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_host_address_end(R: *const xcb_host_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_host_next(i: *mut xcb_host_iterator_t);
}
extern "C" {
    pub fn xcb_host_end(i: xcb_host_iterator_t) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_list_hosts_sizeof(_buffer: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_hosts(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t;
}
extern "C" {
    pub fn xcb_list_hosts_unchecked(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t;
}
extern "C" {
    pub fn xcb_list_hosts_hosts_length(R: *const xcb_list_hosts_reply_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_list_hosts_hosts_iterator(R: *const xcb_list_hosts_reply_t) -> xcb_host_iterator_t;
}
extern "C" {
    pub fn xcb_list_hosts_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_list_hosts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_hosts_reply_t;
}
extern "C" {
    pub fn xcb_set_access_control_checked(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_access_control(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_close_down_mode_checked(c: *mut xcb_connection_t, mode: u8)
        -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_close_down_mode(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_kill_client_checked(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_kill_client(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_rotate_properties_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_rotate_properties_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: u16,
        delta: i16,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_rotate_properties(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: u16,
        delta: i16,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_rotate_properties_atoms(
        R: *const xcb_rotate_properties_request_t,
    ) -> *mut xcb_atom_t;
}
extern "C" {
    pub fn xcb_rotate_properties_atoms_length(
        R: *const xcb_rotate_properties_request_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_rotate_properties_atoms_end(
        R: *const xcb_rotate_properties_request_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_force_screen_saver_checked(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_force_screen_saver(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_set_pointer_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_pointer_mapping(
        c: *mut xcb_connection_t,
        map_len: u8,
        map: *const u8,
    ) -> xcb_set_pointer_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_set_pointer_mapping_unchecked(
        c: *mut xcb_connection_t,
        map_len: u8,
        map: *const u8,
    ) -> xcb_set_pointer_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_set_pointer_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_set_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_set_pointer_mapping_reply_t;
}
extern "C" {
    pub fn xcb_get_pointer_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_pointer_mapping(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_get_pointer_mapping_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_get_pointer_mapping_map(R: *const xcb_get_pointer_mapping_reply_t) -> *mut u8;
}
extern "C" {
    pub fn xcb_get_pointer_mapping_map_length(
        R: *const xcb_get_pointer_mapping_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_pointer_mapping_map_end(
        R: *const xcb_get_pointer_mapping_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_pointer_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_pointer_mapping_reply_t;
}
extern "C" {
    pub fn xcb_set_modifier_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_set_modifier_mapping(
        c: *mut xcb_connection_t,
        keycodes_per_modifier: u8,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_set_modifier_mapping_unchecked(
        c: *mut xcb_connection_t,
        keycodes_per_modifier: u8,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_set_modifier_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_set_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_set_modifier_mapping_reply_t;
}
extern "C" {
    pub fn xcb_get_modifier_mapping_sizeof(
        _buffer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_modifier_mapping(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_get_modifier_mapping_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_get_modifier_mapping_cookie_t;
}
extern "C" {
    pub fn xcb_get_modifier_mapping_keycodes(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> *mut xcb_keycode_t;
}
extern "C" {
    pub fn xcb_get_modifier_mapping_keycodes_length(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_modifier_mapping_keycodes_end(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> xcb_generic_iterator_t;
}
extern "C" {
    pub fn xcb_get_modifier_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_modifier_mapping_reply_t;
}
extern "C" {
    pub fn xcb_no_operation_checked(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
extern "C" {
    pub fn xcb_no_operation(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_auth_info_t {
    pub namelen: ::std::os::raw::c_int,
    pub name: *mut ::std::os::raw::c_char,
    pub datalen: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_xcb_auth_info_t() {
    const UNINIT: ::std::mem::MaybeUninit<xcb_auth_info_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<xcb_auth_info_t>(),
        32usize,
        concat!("Size of: ", stringify!(xcb_auth_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<xcb_auth_info_t>(),
        8usize,
        concat!("Alignment of ", stringify!(xcb_auth_info_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).namelen) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_auth_info_t),
            "::",
            stringify!(namelen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_auth_info_t),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).datalen) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_auth_info_t),
            "::",
            stringify!(datalen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(xcb_auth_info_t),
            "::",
            stringify!(data)
        )
    );
}
extern "C" {
    pub fn xcb_flush(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_get_maximum_request_length(c: *mut xcb_connection_t) -> u32;
}
extern "C" {
    pub fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t);
}
extern "C" {
    pub fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
}
extern "C" {
    pub fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
}
extern "C" {
    pub fn xcb_poll_for_queued_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_special_event {
    _unused: [u8; 0],
}
pub type xcb_special_event_t = xcb_special_event;
extern "C" {
    pub fn xcb_poll_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t;
}
extern "C" {
    pub fn xcb_wait_for_special_event(
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xcb_extension_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn xcb_register_for_special_xge(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
        eid: u32,
        stamp: *mut u32,
    ) -> *mut xcb_special_event_t;
}
extern "C" {
    pub fn xcb_unregister_for_special_event(c: *mut xcb_connection_t, se: *mut xcb_special_event_t);
}
extern "C" {
    pub fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
}
extern "C" {
    pub fn xcb_discard_reply(c: *mut xcb_connection_t, sequence: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn xcb_discard_reply64(c: *mut xcb_connection_t, sequence: u64);
}
extern "C" {
    pub fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
}
extern "C" {
    pub fn xcb_prefetch_extension_data(c: *mut xcb_connection_t, ext: *mut xcb_extension_t);
}
extern "C" {
    pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
}
extern "C" {
    pub fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_connect_to_fd(
        fd: ::std::os::raw::c_int,
        auth_info: *mut xcb_auth_info_t,
    ) -> *mut xcb_connection_t;
}
extern "C" {
    pub fn xcb_disconnect(c: *mut xcb_connection_t);
}
extern "C" {
    pub fn xcb_parse_display(
        name: *const ::std::os::raw::c_char,
        host: *mut *mut ::std::os::raw::c_char,
        display: *mut ::std::os::raw::c_int,
        screen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xcb_connect(
        displayname: *const ::std::os::raw::c_char,
        screenp: *mut ::std::os::raw::c_int,
    ) -> *mut xcb_connection_t;
}
extern "C" {
    pub fn xcb_connect_to_display_with_auth_info(
        display: *const ::std::os::raw::c_char,
        auth: *mut xcb_auth_info_t,
        screen: *mut ::std::os::raw::c_int,
    ) -> *mut xcb_connection_t;
}
extern "C" {
    pub fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;
}
extern "C" {
    pub fn xcb_total_read(c: *mut xcb_connection_t) -> u64;
}
extern "C" {
    pub fn xcb_total_written(c: *mut xcb_connection_t) -> u64;
}
pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}
#[test]
fn bindgen_test_layout_VkXcbSurfaceCreateInfoKHR() {
    const UNINIT: ::std::mem::MaybeUninit<VkXcbSurfaceCreateInfoKHR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<VkXcbSurfaceCreateInfoKHR>(),
        40usize,
        concat!("Size of: ", stringify!(VkXcbSurfaceCreateInfoKHR))
    );
    assert_eq!(
        ::std::mem::align_of::<VkXcbSurfaceCreateInfoKHR>(),
        8usize,
        concat!("Alignment of ", stringify!(VkXcbSurfaceCreateInfoKHR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sType) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VkXcbSurfaceCreateInfoKHR),
            "::",
            stringify!(sType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pNext) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VkXcbSurfaceCreateInfoKHR),
            "::",
            stringify!(pNext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VkXcbSurfaceCreateInfoKHR),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connection) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VkXcbSurfaceCreateInfoKHR),
            "::",
            stringify!(connection)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).window) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VkXcbSurfaceCreateInfoKHR),
            "::",
            stringify!(window)
        )
    );
}
pub type PFN_vkCreateXcbSurfaceKHR = ::std::option::Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
>;
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = ::std::option::Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> VkBool32,
>;
extern "C" {
    pub fn vkCreateXcbSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
extern "C" {
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> VkBool32;
}