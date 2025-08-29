use std::{any::Any, fmt::Debug};

pub trait AsAny
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Component: Any + Debug + AsAny {}

impl<T: Sized + Component> AsAny for T
{
    fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
}