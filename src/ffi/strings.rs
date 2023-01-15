#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::CStr;

pub unsafe fn from_c_string(char_array: &[i8]) -> &str
{
	CStr::from_ptr(char_array.as_ptr()).to_str().unwrap()
}

// only takes static string slices because I have trust issues 
// (I don't know how the C code will later use this memory)
pub unsafe fn to_c_string(input_str: &'static str) -> *const i8
{
	input_str.as_ptr() as *const i8
}