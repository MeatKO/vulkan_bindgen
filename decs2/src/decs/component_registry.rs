use std::{any::TypeId, collections::HashMap};

use crate::decs::{component_storage::{ComponentStorage, ComponentVec}, typedef::Component};


#[derive(Default)]
pub struct ComponentTypeRegistry
{
    pub counter: usize,
    typeid_to_uuid: HashMap<TypeId, usize>,
    index_to_name: Vec<String>,
    vector_factories: Vec<fn() -> Box<dyn ComponentStorage>>,
}

impl ComponentTypeRegistry
{
    pub fn new() -> Self
    {
        Self { 
            counter: 0usize, 
            typeid_to_uuid: HashMap::new(), 
            index_to_name: vec![],
            vector_factories: vec![]
        }
    }

    pub fn get_uuid_of(&self, type_id: &TypeId)
    -> Option<usize>
    {
        self.typeid_to_uuid.get(type_id).copied()
    }

    pub fn index_to_typename(&self, type_index: usize)
    -> Option<String>
    {
        Some(self.index_to_name[type_index].clone())
    }

    /// true = type was just registered
    /// false = type already exists in the registry
    pub fn register_type<C: Component>(&mut self)
    -> bool
    {
        let component_typeid = std::any::TypeId::of::<C>();

        if self.typeid_to_uuid.contains_key(&component_typeid)
        {
            return false
        }

        self.typeid_to_uuid.insert(component_typeid, self.counter);
        self.index_to_name.push(std::any::type_name::<C>().to_owned());
        self.vector_factories.push(
            || { Box::new(ComponentVec::<C>::new_default()) }
        );

        self.counter += 1;

        true
    }

    /// Shit will panic if you use invalid index
    pub fn index_to_factory(&self, type_index: usize)
    -> fn() -> Box<dyn ComponentStorage>
    {
        self.vector_factories[type_index]
    }
}

#[cfg(test)]
mod tests 
{
    use component_derive::Component;

    use crate::decs::{component_registry::ComponentTypeRegistry, component_storage::ComponentVec, typedef::Component};

    #[derive(Component)]
    struct TestComponent{}

    #[test]
    fn get_existing_key() 
    {
        let mut component_registry = ComponentTypeRegistry::new();

        component_registry.register_type::<TestComponent>();

        let test_component_type_id = std::any::TypeId::of::<TestComponent>();
        let result = component_registry.get_uuid_of(&test_component_type_id).unwrap();

        assert_eq!(result, 0);

        let vec_factory = component_registry.index_to_factory(result);
        let component_vec = vec_factory();

        let box_typeid = component_vec.as_any().type_id();
        let desired_typeid = std::any::TypeId::of::<ComponentVec<TestComponent>>();

        assert_eq!(box_typeid, desired_typeid);
    }

    #[test]
    fn get_non_existing_key() 
    {
        let component_registry = ComponentTypeRegistry::new();

        let test_component_type_id = std::any::TypeId::of::<TestComponent>();
        let result = component_registry.get_uuid_of(&test_component_type_id);

        assert_eq!(result, None)
    }
}