use std::f32::consts::PI;

pub fn radians(degrees: f32) -> f32
{
	return ( PI / 180.0f32 ) * degrees;
}