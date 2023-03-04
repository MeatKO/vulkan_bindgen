use crate::loseit::{
	window_events::KeyValues,
};

pub fn convert_key_code(key_code: u8) -> KeyValues
{
	match key_code
	{
		9 => KeyValues::ESC,
		25 => KeyValues::W,
		38 => KeyValues::A,
		39 => KeyValues::S,
		40 => KeyValues::D,
		_ => KeyValues::UNKNOWN
	}
}