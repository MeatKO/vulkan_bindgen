#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::CStr;

pub fn from_c_string(char_array: &[i8]) -> Result<&str, std::str::Utf8Error>
{
	unsafe
	{
		CStr::from_ptr(char_array.as_ptr()).to_str()
	}
}

pub fn from_c_string_ptr(char_array: *const i8) -> Result<&'static str, std::str::Utf8Error>
{
	unsafe
	{
		CStr::from_ptr(char_array).to_str()
	}
}

// LEAKS !
// results in a null-terminated essentially 'static output str
pub fn to_c_string<S: ToString>(input_str: S)-> *const i8
{
	let null_terminated_string = String::from(input_str.to_string() + "\0");

	let out_ptr = null_terminated_string.as_ptr() as *const i8;
	std::mem::forget(null_terminated_string);

	out_ptr
}