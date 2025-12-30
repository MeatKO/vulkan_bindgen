use std::{any::{self, Any, TypeId}, collections::HashMap};

use component_derive::Component;

type System = dyn Fn(&mut dECSManager) -> () + Send + Sync + 'static;

use crate::decs::{
    archetype::Archetype, component_registry::ComponentTypeRegistry, component_storage::{ComponentStorage, ComponentVec}, typedef::{
        Component, 
        Entity,
    }, util::vec_two_mut
};

#[derive(Default, Component)]
struct _PlaceHolderComponent;

#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct dECSManager
{
    // we can't have a dense set here because its a one-to-many relation so a single usize won't suffice
    // not to mention that every Archetype keeps store of a dense entity_id vector
    // so we don't need one here if we want a dense iteration
    pub sparse: Vec<usize>, // archetype id
    pub archetypes: Vec<Archetype>,
    pub component_type_registry: ComponentTypeRegistry,

    global_unique_storage: HashMap<TypeId, Box<dyn Any>>,

    init_systems: Vec<Box<System>>,
	systems: Vec<Box<System>>,

    types_to_archetype_index: HashMap<Vec<bool>, usize>,
}

impl dECSManager
{
    pub fn new() -> Self
    {
        let archetypes = 
            vec![
                Archetype::default()
            ];

        Self {
            archetypes,
            ..Default::default()
        }
    }

    pub fn add_global_storage<C: Component>(&mut self, component: C)
    {
        println!("Adding storage : {:?} - {}", std::any::TypeId::of::<C>(), std::any::type_name::<C>());
        self.global_unique_storage.insert(std::any::TypeId::of::<C>(), Box::new(component));
    }

    pub fn modify_global_storage<C: Component>(&mut self, mut modifier_fn:  impl FnMut(&mut C) -> Result<(), String>)
    -> Result<(), String>
    {
        let any_ref = 
            match self.global_unique_storage.get_mut(&std::any::TypeId::of::<C>())
            {
                Some(reference) => { reference }
                None => { return Err(format!("modify_global_storage<C>(fn) - reference of type '{}' not found", std::any::type_name::<C>()).to_owned()) }
            };

        let c_ref = 
            any_ref.downcast_mut::<C>()
            .expect(&format!("modify_global_storage - any_ref.downcast_mut::<C>() failed for type '{}'", std::any::type_name::<C>()));

        modifier_fn(c_ref)
    }

    pub fn get_global_storage_mut_unchecked<'a, 'b, C: Component>(&'a self)
    -> Option<&'b mut C>
    {
        
        // println!("Getting storage : {:?} - {}", std::any::TypeId::of::<C>(), std::any::type_name::<C>());
        let any_ref = self.global_unique_storage.get(&std::any::TypeId::of::<C>())?;

        unsafe
        {
            let raw_ptr: *const C = any_ref.downcast_ref::<C>()?;

            let unsafe_ref: &mut C = (raw_ptr as *mut C).as_mut().unwrap();

            Some(unsafe_ref)
        }
    }

    pub fn add_system<S: Fn(&mut dECSManager) + Send + Sync + 'static>(&mut self, system: S)
	{
		self.systems.push(Box::new(system));
	}

	pub fn add_init_system<S: Fn(&mut dECSManager) + Send + Sync + 'static>(&mut self, system: S)
	{
        self.init_systems.push(Box::new(system));
	}

    pub fn init_systems(&mut self)
	{
        let systems = std::mem::take(&mut self.init_systems);

		for system in systems.iter()
        {
            system(self);
        }

        self.init_systems = systems;
	}

	pub fn update_systems(&mut self)
	{
		let systems = std::mem::take(&mut self.systems);

        for system in systems.iter()
        {
            system(self);
        }

        self.systems = systems;
	}

    pub fn register_type<C: Component>(&mut self)
    {
        if self.component_type_registry.register_type::<C>()
        {
            let type_index = self.component_type_registry.get_uuid_of(&std::any::TypeId::of::<C>()).unwrap();
            let type_factory = self.component_type_registry.index_to_factory(type_index);

            for archetype in self.archetypes.iter_mut()
            {
                archetype.type_flags.push(false);
                archetype.components.push(type_factory());
            }
        }
    }

    pub fn add_entity(&mut self) -> Entity
    {
        // put in the default Archetype
        self.sparse.push(0);

        for archetype in self.archetypes.iter_mut()
        {
            archetype.sparse.push(None);
        }

        let out_sparse = self.sparse.len() - 1;

        // Assuming we have already initialized the default Archetype:
        self.archetypes[0].dense.push(out_sparse);
        self.archetypes[0].sparse[out_sparse] = Some(self.archetypes[0].dense.len() - 1);

        Entity(out_sparse)
    }

    pub fn remove_component<C: Component>(&mut self, entity: &Entity)
    {
        let entity_sparse = entity.0;

        // OOB check
        if entity_sparse >= self.sparse.len()
        {
            panic!("entity id [{}] out-of-bounds for sparse vector of len [{}]", entity_sparse, self.sparse.len());
        }

        self.register_type::<C>();

        let component_type_index =
            self.component_type_registry.get_uuid_of(&any::TypeId::of::<C>())
            .unwrap();

        let archetype_index = self.sparse[entity_sparse];

        // nothing to remove here...
        if archetype_index == 0
        {
            return;
        }

        // if the current has only one type
        // then we transfer the entity to the 0th archetype which is empty
        if self.archetypes[archetype_index].component_type_count == 1
        {
            // self.archetypes[archetype_index].take_components(entity_sparse);
            self.archetypes[archetype_index].remove_components(entity_sparse);
            self.sparse[entity_sparse] = 0;

            return
        }

        let source_archetype_index = archetype_index;

        // if the source archetype already doesn't hold components of that type we simply return
        // edge case was introduced with the addition of the default archetype
        // now the manager sparse set is not optional and I forgot to add this check after I removed the None check from before
        if !self.archetypes[source_archetype_index].type_flags[component_type_index]
        {
            return
        }

        let mut required_type_flags = self.archetypes[source_archetype_index].type_flags.clone();
        required_type_flags[component_type_index] = false; // Remove the <C> flag

        let destination_archetype_index = 
            match self.get_archetype_with_exact_type_flags(&required_type_flags)
            {
                Some(archetype) => { archetype }
                None => { self.add_archetype(required_type_flags) }
            };

        let (destination_archetype, source_archetype) = 
            vec_two_mut(self.archetypes.as_mut_slice(), destination_archetype_index, source_archetype_index)
            .expect(&format!("Attempted to transfer components from source a[{}] to destination a[{}]", source_archetype_index, destination_archetype_index));


        // Could be made way faster if we just make a take_insert(self, source_index, destination_index) 
        // and just push the taken value direcly
        // with no middlemen structures and no inbetween transfers
        let source_components = 
            source_archetype.take_components(entity_sparse)
            .into_iter()
            .filter(
                |(type_index, _)|
                {
                   *type_index != component_type_index
                }
            )
            .collect();

        destination_archetype.insert_components(entity_sparse, source_components);

        self.sparse[entity_sparse] = destination_archetype_index;
    }

    pub fn add_component<C: Component>(&mut self, entity: &Entity, component: C)
    {
        let entity_sparse = entity.0;

        // OOB check
        if entity_sparse >= self.sparse.len()
        {
            panic!("entity id [{}] out-of-bounds for sparse vector of len [{}]", entity_sparse, self.sparse.len());
        }

        self.register_type::<C>();

        let component_type_index =
            self.component_type_registry.get_uuid_of(&any::TypeId::of::<C>())
            .unwrap();

        let archetype_index = self.sparse[entity_sparse];

        if self.archetypes[archetype_index].has_type_flag(component_type_index)
        {
            let archetype = &mut self.archetypes[archetype_index];

            let component_vec =
                archetype.components[component_type_index]
                .as_any_mut()
                .downcast_mut::<ComponentVec<C>>() 
                .unwrap();
            
            match archetype.sparse[entity_sparse]
            {
                Some(dense_index) => 
                {
                    component_vec.0[dense_index] = component; 
                }
                None => 
                {
                    component_vec.0.push(component);
                    archetype.sparse[entity_sparse] = Some(component_vec.0.len() - 1);
                    archetype.dense.push(entity_sparse);
                    archetype.component_count += 1;
                }
            }
        }
        else
        {
            let source_archetype_index = archetype_index;

            let source_type_flags = self.archetypes[source_archetype_index].type_flags.clone();
            let mut required_type_flags = source_type_flags;
            required_type_flags[component_type_index] = true;

            let destination_archetype_index = 
                match self.get_archetype_with_exact_type_flags(&required_type_flags)
                {
                    Some(archetype) => { archetype }
                    None => { self.add_archetype(required_type_flags) }
                };

            let (destination_archetype, source_archetype) = 
                vec_two_mut(self.archetypes.as_mut_slice(), destination_archetype_index, source_archetype_index)
                .unwrap();

            let mut source_components = source_archetype.take_components(entity_sparse);
            source_components.push((component_type_index, Box::new(component)));

            destination_archetype.insert_components(entity_sparse, source_components);

            self.sparse[entity_sparse] = destination_archetype_index;
        }
    }

    fn get_archetype_with_exact_type_flags(&self, required_type_flags: &Vec<bool>)
    -> Option<usize>
    {
        self.types_to_archetype_index.get(required_type_flags).copied()

        // for (index, archetype) in self.archetypes.iter().enumerate()
        // {
        //     if archetype.type_flags == *required_type_flags
        //     {
        //         return Some(index)
        //     }
        // }

        // None
    }

    fn add_archetype(&mut self, type_flags: Vec<bool>) 
    -> usize
    {
        let placeholder_component_generator: fn () -> Box<dyn ComponentStorage> = || { Box::new(ComponentVec::<_PlaceHolderComponent>::new_default()) };

        let mut new_components_vec: Vec<Box<dyn ComponentStorage>> =
            std::iter::repeat_with(|| placeholder_component_generator())
            .take(self.component_type_registry.counter)
            .collect();

        let mut component_type_count = 0usize;

        for (type_index, is_required) in type_flags.iter().enumerate()
        {
            if *is_required
            {
                new_components_vec[type_index] = self.component_type_registry.index_to_factory(type_index)();
                component_type_count += 1;
            }
        }

        if component_type_count == 0
        {
            panic!("Can't add empty-typed archetype, input type_flags were : {:?}", type_flags)
        }

        let new_archetype = 
            Archetype{
                component_type_count: component_type_count,
                type_flags: type_flags.clone(),
                sparse: vec![None; self.sparse.len()],
                dense: vec![],
                component_count: 0,
                components: new_components_vec,
            };
        
        self.archetypes.push(new_archetype);

        let new_archetype_index = self.archetypes.len() - 1;

        self.types_to_archetype_index.insert(type_flags, new_archetype_index);

        new_archetype_index
    }
}
