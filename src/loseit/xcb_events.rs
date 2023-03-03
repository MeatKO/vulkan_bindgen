use crate::loseit::{
	window_events::KeyValues,
};

pub fn convert_key_code(key_code: u8) -> KeyValues
{
	match key_code
	{
		9 => KeyValues::ESC,
		_ => KeyValues::UNKNOWN
	}
}