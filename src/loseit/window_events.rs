pub enum WindowEvent
{
	KeyPress(KeyValues),
	MousePress(MouseValues),
	WindowAction(WindowActions)
}

pub enum WindowActions
{
	EXPOSE,
	LEAVE,
	CLOSE,
	MINIMIZE,
	MAXIMIZE,
	RESIZE
}

pub enum KeyValues
{
	ESC,
	UNKNOWN,
}

pub enum MouseValues
{
	LEFT,
	RIGHT,
	MIDDLE,
}