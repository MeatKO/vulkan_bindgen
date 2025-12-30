use std::any::Any;

use crate::decs::typedef::Component;

pub trait ComponentStorage
{
    fn swap_remove(&mut self, remove_index: usize) -> Box<dyn Any>;
    // fn push_default(&mut self);
    fn insert_box(&mut self, new_element: Box<dyn Any>);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_component_unchecked(&self, index: usize) -> &dyn Component;
    fn try_get_component_unchecked(&self, index: usize) -> Option<&dyn Component>;
}

pub trait ComponentStorageView
{
    fn as_vec_mut<T: Component>(&mut self) -> Option<&mut Vec<T>>;
    fn as_vec<T: Component>(&self) -> Option<&Vec<T>>;
}

// #[derive(Default)]
pub struct ComponentVec<T: Component>(pub Vec<T>);

impl<S: ComponentStorage + ?Sized> ComponentStorageView for S 
{
    fn as_vec_mut<T: Component>(&mut self) -> Option<&mut Vec<T>> 
    {
        // Some(&mut self.as_any_mut().downcast_mut::<ComponentVec<T>>()?.0) 
        unsafe { Some(&mut self.as_any_mut().downcast_unchecked_mut::<ComponentVec<T>>().0) } // fuck
    }

    fn as_vec<T: Component>(&self) -> Option<&Vec<T>> 
    {
        // Some(&self.as_any().downcast_ref::<ComponentVec<T>>()?.0)
        unsafe { Some(&self.as_any().downcast_unchecked_ref::<ComponentVec<T>>().0) } // fuck
    }
}

impl<T: Component> ComponentVec<T>
{
    pub fn new_default() 
    -> Self
    {
        return Self(vec![])
    }
}

impl<T: Component> ComponentStorage for ComponentVec<T>
{
    fn swap_remove(&mut self, remove_index: usize) 
    -> Box<dyn Any> 
    {
        Box::new(self.0.swap_remove(remove_index))
    }

    // fn push_default(&mut self) 
    // {
    //     self.0.push(T::default());
    // }
    
    fn insert_box(&mut self, new_element: Box<dyn Any>) 
    {
        // self.0.push(*new_element.downcast().unwrap()); 
        unsafe { self.0.push(*new_element.downcast_unchecked()); } // fuck
    }
    
    fn as_any(&self) 
    -> &dyn Any 
    {
        self
    }
    
    fn as_any_mut(&mut self) 
    -> &mut dyn Any 
    {
        self
    }
    
    fn get_component_unchecked(&self, index: usize) -> &dyn Component 
    {
        &self.0[index]
    }
    
    fn try_get_component_unchecked(&self, index: usize) -> Option<&dyn Component>
    {
        self.0.get(index).map(|x| x as &dyn Component)
    }
}