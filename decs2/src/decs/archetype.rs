use std::any::Any;

use crate::decs::component_storage::ComponentStorage;

#[derive(Default)]
pub struct Archetype
{
    pub component_type_count: usize,
    pub component_count: usize,
    pub type_flags: Vec<bool>,
    pub sparse: Vec<Option<usize>>, // which component
    pub dense: Vec<usize>,
    pub components: Vec<Box<dyn ComponentStorage>>,
}

impl Archetype
{
    pub fn has_type_flag(&self, type_flag: usize) 
    -> bool
    {
        self.type_flags[type_flag]
    }

    /// Shit panics if types don't match or if the input parameters are out of bounds !
    pub fn get_component<C: 'static>(&self, type_uuid: usize, component_index: usize) -> &C
    {
        self.components[type_uuid].
        get_component_unchecked(component_index)
        .as_any().downcast_ref::<C>()
        .unwrap()
    }

    /// Shit panics if types don't match or if the input parameters are out of bounds !
    pub fn get_component_mut<C: 'static>(&self, type_uuid: usize, component_index: usize) -> &mut C
    {
        unsafe
        {
            (
                ( 
                    self.components[type_uuid]
                    .get_component_unchecked(component_index)
                    .as_any().downcast_ref::<C>()
                    .unwrap() 
                    as *const C 
                ) as *mut C
            ).as_mut()
            .unwrap()
        }
    }

    /// Shit panics if types don't match or if the input parameters are out of bounds !
    pub fn try_get_component<C: 'static>(&self, type_uuid: usize, component_index: usize) -> Option<&C>
    {
        self.components[type_uuid]
        .try_get_component_unchecked(component_index)
        .map(
            |x| 
            x.as_any().downcast_ref::<C>()
            .unwrap()
        )
    }

    /// Shit panics if types don't match or if the input parameters are out of bounds !
    pub fn try_get_component_mut<C: 'static>(&self, type_uuid: usize, component_index: usize) -> Option<&mut C>
    {
        unsafe
        {
            self.components[type_uuid]
            .try_get_component_unchecked(component_index)
            .map(
                |x| 
                {
                    ( 
                        (
                            x.as_any().downcast_ref::<C>()
                            .unwrap()
                            as *const C
                        ) as *mut C
                    ).as_mut()
                    .unwrap()
                }
            )
        }
    }

    /// Extracts the Boxed components and removes them from the archetype.
    /// Then Sets the sparse entity id to None.
    pub fn take_components(&mut self, entity_sparse_id: usize)
    -> Vec<(usize, Box<dyn Any>)>
    {
        let mut out_vec = Vec::with_capacity(self.component_type_count);

        if self.component_type_count == 0
        {
            return out_vec
        }

        let dense_index = self.sparse[entity_sparse_id].unwrap();

        for (type_index, type_flag) in self.type_flags.iter().cloned().enumerate()
        {
            if type_flag
            {
                out_vec.push((type_index, self.components[type_index].swap_remove(dense_index)));
            }
        }

        let last_sparse_index = self.dense.last().cloned().unwrap();
        self.sparse[last_sparse_index] = Some(dense_index);
        self.sparse[entity_sparse_id] = None;

        self.dense.swap_remove(dense_index);

        self.component_count -= 1;

        out_vec
    }

    /// Deletes the components from the archetype.
    /// Then Sets the sparse entity id to None.
    pub fn remove_components(&mut self, entity_sparse_id: usize)
    {
        if self.component_type_count == 0
        {
            return
        }

        let dense_index = self.sparse[entity_sparse_id].unwrap();

        for (type_index, type_flag) in self.type_flags.iter().cloned().enumerate()
        {
            if type_flag
            {
                self.components[type_index].swap_remove(dense_index);
            }
        }

        let last_sparse_index = self.dense.last().cloned().unwrap();
        self.sparse[last_sparse_index] = Some(dense_index);
        self.sparse[entity_sparse_id] = None;

        self.dense.swap_remove(dense_index);

        self.component_count -= 1;
    }

    pub fn insert_components(&mut self, entity_sparse: usize, components: Vec<(usize, Box<dyn Any>)>)
    {
        for (type_index, component_box) in components.into_iter()
        {
            self.components[type_index].insert_box(component_box);
        }

        self.component_count += 1;
        self.dense.push(entity_sparse);
        self.sparse[entity_sparse] = Some(self.component_count - 1);
    }
}
