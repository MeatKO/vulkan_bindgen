#[derive(Debug)]
pub enum WindowEvent
{
	// KeyPress(KeyValues),
	// KeyRelease(KeyValues),
	KeyPress(u8),
	KeyRelease(u8),
	MousePress(u8),
	MouseRelease(u8),
	WindowAction(WindowActions)
}

#[derive(Debug)]
pub enum WindowActions
{
	Expose,
	Leave,
	Close,
	Minimize,
	Maximize,
	Resize,
	FocusOut,
	FocusIn,
	Motion(i32, i32),
	Configure(i32, i32),
}

#[derive(Debug)]
#[repr(u8)]
pub enum KeyValues
{
	ESC = 9,
	W = 25,
	A = 38,
	S = 39,
	D = 40,
	Unknown = 255, // to record the unmapped key value
}

#[derive(Debug)]
pub enum MouseValues
{
	Left,
	Right,
	Middle,
}