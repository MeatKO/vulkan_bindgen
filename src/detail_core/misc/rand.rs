use std::collections::hash_map::*;
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::ops::Range;

// pub fn rand(in_range: Range<u64>) -> u64 
// {
// 	let random_number = RandomState::new().build_hasher().finish();

// 	return (random_number % in_range.start.abs_diff(in_range.end)) + in_range.start;
// }

pub fn rand(in_range: Range<i64>) -> i64 
{
	let random_number = RandomState::new().build_hasher().finish();

	return (random_number % (in_range.start - in_range.end).unsigned_abs() ) as i64 + in_range.start;
}
