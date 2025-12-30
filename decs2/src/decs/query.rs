use std::{any::TypeId, marker::PhantomData};

use crate::{decs::{archetype::Archetype, manager::dECSManager, typedef::Component}};

pub struct Read<T>(PhantomData<T>);
pub struct Write<T>(PhantomData<T>);
pub struct MaybeRead<T>(PhantomData<T>);
pub struct MaybeWrite<T>(PhantomData<T>);

pub trait Fetch<'a>
{
    type Item;
    const LEN: usize;

    unsafe fn fetch(
        archetype: &'a Archetype,
        type_uuids: &'a [usize],
        current_type_uuid_index: usize,
        dense_index: usize,
    ) -> Self::Item;

    fn get_type_ids() -> Vec<TypeId>;
    fn get_non_optonal_type_ids() -> Vec<TypeId>;
}

impl <'a, T> Fetch<'a> for Read<T>
where
    T: Component
{
    type Item = &'a T;
    const LEN: usize = 1;

    unsafe fn fetch(
        archetype: &'a Archetype,
        type_uuids: &'a [usize],
        current_type_uuid_index: usize,
        dense_index: usize,
    ) -> Self::Item
    {
        archetype.get_component(type_uuids[current_type_uuid_index], dense_index)
    }

    fn get_type_ids() -> Vec<TypeId> 
    {
        vec![std::any::TypeId::of::<T>()]
    }

    fn get_non_optonal_type_ids() -> Vec<TypeId> 
    {
        vec![std::any::TypeId::of::<T>()]
    }
}

impl <'a, T> Fetch<'a> for Write<T>
where
    T: Component
{
    type Item = &'a mut T;
    const LEN: usize = 1;

    unsafe fn fetch(
        archetype: &'a Archetype,
        type_uuids: &'a [usize],
        current_type_uuid_index: usize,
        dense_index: usize,
    ) -> Self::Item
    {
        archetype.get_component_mut(type_uuids[current_type_uuid_index], dense_index)
    }

    fn get_type_ids() -> Vec<TypeId> 
    {
        vec![std::any::TypeId::of::<T>()]
    }

    fn get_non_optonal_type_ids() -> Vec<TypeId> 
    {
        vec![std::any::TypeId::of::<T>()]
    }
}

impl <'a, T> Fetch<'a> for MaybeRead<T>
where
    T: Component
{
    type Item = Option<&'a T>;
    const LEN: usize = 1;

    unsafe fn fetch(
        archetype: &'a Archetype,
        type_uuids: &[usize],
        current_type_uuid_index: usize,
        dense_index: usize,
    ) -> Self::Item
    {
        archetype.try_get_component(type_uuids[current_type_uuid_index], dense_index)
    }

    fn get_type_ids() -> Vec<TypeId> 
    {
        // vec![std::any::TypeId::of::<T>()]
        vec![]
    }

    fn get_non_optonal_type_ids() -> Vec<TypeId> 
    {
        vec![std::any::TypeId::of::<T>()]
    }
}

impl <'a, T> Fetch<'a> for MaybeWrite<T>
where
    T: Component
{
    type Item = Option<&'a mut T>;
    const LEN: usize = 1;

    unsafe fn fetch(
        archetype: &'a Archetype,
        type_uuids: &'a [usize],
        current_type_uuid_index: usize,
        dense_index: usize,
    ) -> Self::Item
    {
        archetype.try_get_component_mut(type_uuids[current_type_uuid_index], dense_index)
    }

    fn get_type_ids() -> Vec<TypeId> 
    {
        // vec![std::any::TypeId::of::<T>()]
        vec![]
    }

    fn get_non_optonal_type_ids() -> Vec<TypeId> 
    {
        vec![std::any::TypeId::of::<T>()]
    }
}

// impl <'a, C1, C2> Fetch<'a> for (C1, C2)
// where
//     C1: Fetch<'a>,
//     C2: Fetch<'a>,
// {
//     type Item = (C1::Item, C2::Item);
//     const LEN: usize = C1::LEN + C2::LEN;
    
//     unsafe fn fetch(
//         archetype: &'a Archetype,
//         type_uuids: &'a [usize],
//         current_type_uuid_index: usize,
//         dense_index: usize,
//     ) -> Self::Item 
//     {
//         unsafe
//         {
//             (
//                 C1::fetch(archetype, type_uuids, current_type_uuid_index, dense_index),
//                 C2::fetch(archetype, type_uuids, current_type_uuid_index + C1::LEN, dense_index),
//             )
//         }
//     }

//     fn get_type_ids() -> Vec<TypeId> 
//     {
//         let mut out = C1::get_type_ids();
//         out.append(&mut C2::get_type_ids());
        
//         out
//     }

//     fn get_non_optonal_type_ids() -> Vec<TypeId> 
//     {
//         let mut out = C1::get_non_optonal_type_ids();
//         out.append(&mut C2::get_non_optonal_type_ids());
        
//         out
//     }
// }


macro_rules! impl_fetch_for_tuples {
    // Base case: single element tuple
    ($($C:ident),+) => {
        impl_fetch_for_tuples!(@inner [$($C),+]);
    };
    
    // // Recursive case: add one more element
    // ($($C:ident),+ , $Rest:ident) => {
    //     impl_fetch_for_tuples!($($C),+);
    //     impl_fetch_for_tuples!(@inner [$($C),+ , $Rest]);
    // };
    
    // Inner implementation
    (@inner [$($C:ident),+]) => {
        impl<'a, $($C),+> Fetch<'a> for ($($C),+)
        where
            $($C: Fetch<'a>),+
        {
            type Item = ($($C::Item),+);
            const LEN: usize = 0 $(+ $C::LEN)+;
            
            unsafe fn fetch(
                archetype: &'a Archetype,
                type_uuids: &'a [usize],
                mut current_type_uuid_index: usize,
                dense_index: usize,
            ) -> Self::Item 
            {
                // Generate tuple with appropriate offsets
                (
                    $(
                        {
                            let item = $C::fetch(archetype, type_uuids, current_type_uuid_index, dense_index);
                            current_type_uuid_index += $C::LEN;
                            item
                        }
                    ),+
                )
            }

            fn get_type_ids() -> Vec<TypeId> 
            {
                let mut out = Vec::new();
                $(
                    out.append(&mut $C::get_type_ids());
                )+
                out
            }

            fn get_non_optonal_type_ids() -> Vec<TypeId> 
            {
                let mut out = Vec::new();
                $(
                    out.append(&mut $C::get_non_optonal_type_ids());
                )+
                out
            }
        }
    };
}

// impl_fetch_for_tuples!(C1);
impl_fetch_for_tuples!(C1, C2);
impl_fetch_for_tuples!(C1, C2, C3);
impl_fetch_for_tuples!(C1, C2, C3, C4);
impl_fetch_for_tuples!(C1, C2, C3, C4, C5);
impl_fetch_for_tuples!(C1, C2, C3, C4, C5, C6);
impl_fetch_for_tuples!(C1, C2, C3, C4, C5, C6, C7);
impl_fetch_for_tuples!(C1, C2, C3, C4, C5, C6, C7, C8);

pub struct Query<'a, F>
{
    pub archetypes: Vec<&'a Archetype>,
    pub type_uuids: Vec<usize>, 
    phantom: PhantomData<F>,
}

pub struct QueryIterator<'a, F> 
{
    pub archetypes: &'a [&'a Archetype], // only archetypes that match all type IDs
    pub type_uuids: &'a [usize],         // component indices in each archetype
    pub archetype_index: usize,          // current archetype
    pub dense_index: usize,              // index inside the current archetype
    phantom: PhantomData<F>,
}

impl<'a, F> Query<'a, F>
where
    F: Fetch<'a>,
{
    pub fn new(decs: &'a dECSManager) -> Self
    {
        let decs: &dECSManager = decs;

        let optional_type_uuids = 
            F::get_type_ids().into_iter()
            .map(
                |type_id|
                decs.component_type_registry.get_uuid_of(&type_id).unwrap()
            )
            .collect::<Vec<usize>>();

        let type_uuids = 
            F::get_non_optonal_type_ids().into_iter()
            .map(
                |type_id|
                decs.component_type_registry.get_uuid_of(&type_id).unwrap()
            )
            .collect::<Vec<usize>>();

        // panic!("{:?} and {:?}", optional_type_uuids, type_uuids);

        let archetypes = 
            decs
            .archetypes
            .iter()
            .filter(
                |arch| 
                {
                    optional_type_uuids.iter().all(
                        |&uuid| 
                        arch.type_flags[uuid]
                    )
                    // &&
                    // arch.component_count > 0
                }
            )
            .collect::<Vec<_>>();

        Self { 
            archetypes: archetypes.clone(), 
            type_uuids: type_uuids.clone(), 
            phantom: PhantomData
        }
    }
}

impl<'a, F> Query<'a, F>
where
    F: Fetch<'a>,
{
    pub fn iter(&'a self) -> QueryIterator<'a, F>
    {
        QueryIterator::new(self)
    }
}

impl<'a, F> QueryIterator<'a, F>
where
    F: Fetch<'a>,
{
    pub fn new(query: &'a Query<'a, F>) -> Self
    {
        Self { 
            archetypes: &query.archetypes, 
            type_uuids: &query.type_uuids, 
            archetype_index: 0, 
            dense_index: 0, 
            phantom: PhantomData,
        }
    }
}

impl<'a, F> Iterator for QueryIterator<'a, F>
where 
    F: Fetch<'a>
{
    type Item = F::Item;

    fn next(&mut self) -> Option<Self::Item> 
    {
        loop
        {
            if self.archetype_index >= self.archetypes.len()
            {
                return None
            }

            let current_archetype = self.archetypes[self.archetype_index];

            // if self.component_index >= current_archetype.component_count
            if self.dense_index >= current_archetype.dense.len()
            {
                self.archetype_index += 1;
                self.dense_index = 0;
                continue;
            }

            unsafe
            {
                let item = Some(F::fetch(self.archetypes[self.archetype_index], self.type_uuids, 0, self.dense_index)); 
                self.dense_index += 1;
                return item;
            }
        }
    }
}