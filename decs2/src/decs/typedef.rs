use std::any::Any;

pub struct Entity(pub usize);

pub trait Component: 'static
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}