#[derive(Debug)]
pub enum WindowEvent
{
	KeyPress(KeyValues),
	MousePress(MouseValues),
	WindowAction(WindowActions)
}

#[derive(Debug)]
pub enum WindowActions
{
	EXPOSE,
	LEAVE,
	CLOSE,
	MINIMIZE,
	MAXIMIZE,
	RESIZE
}

#[derive(Debug)]
pub enum KeyValues
{
	ESC,
	UNKNOWN,
}

#[derive(Debug)]
pub enum MouseValues
{
	LEFT,
	RIGHT,
	MIDDLE,
}