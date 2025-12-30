
#[cfg(test)]
mod tests 
{
    use component_derive::Component;
    use decs2::decs::{manager, typedef::Component};

    #[derive(Component)]
    struct NameComponent(pub String);
    // impl Component for NameComponent {}

    #[derive(Component)]
    struct HealthComponent(pub f32);
    // impl Component for HealthComponent {}

    #[derive(Component)]
    struct IsPlayerComponent {}
    // impl Component for IsPlayerComponent {}

    #[test]
    fn manager_initialization_defaults() 
    {
        let decs_manager = manager::dECSManager::new();

        assert_eq!(decs_manager.archetypes.len(), 1);
        assert_eq!(decs_manager.sparse.len(), 0);
        assert_eq!(decs_manager.component_type_registry.counter, 0);
    }

    #[test]
    fn add_multiple_entities() 
    {
        let mut decs_manager = manager::dECSManager::new();

        let _ = decs_manager.add_entity();

        assert_eq!(decs_manager.sparse.len(), 1);
        assert_eq!(decs_manager.sparse[0], 0);

        let _ = decs_manager.add_entity();

        assert_eq!(decs_manager.sparse.len(), 2);
        assert_eq!(decs_manager.sparse[0], 0);
        assert_eq!(decs_manager.sparse[1], 0);
    }

    /// Check if adding a component messes with unintented parts of the structure
    /// Component addition should involve :
    /// 1. Archetypical relocation of our entity
    /// 2. The change in the manager sparse set, pointing to the new archetype
    /// 3. The old Archetype should erase all relations to the entity
    /// 4. The new Archetype must set its sparse and dense sets to the entity
    #[test]
    fn add_multiple_components_same_entity() 
    {
        let mut decs_manager = manager::dECSManager::new();

        let entity = decs_manager.add_entity();

        // -------------------------------------------------------------------------------

        let component = 
            NameComponent{
                0: "Pookie".to_owned()
            };
        decs_manager.add_component(&entity, component);

        // Manager
        assert_eq!(decs_manager.archetypes.len(), 2);
        assert_eq!(decs_manager.sparse[0], 1);
        assert_eq!(decs_manager.component_type_registry.counter, 1);
        
        // Type Registry
        assert_eq!(decs_manager.component_type_registry.get_uuid_of(&std::any::TypeId::of::<NameComponent>()), Some(0));

        // Archetype 1
        assert_eq!(decs_manager.archetypes[1].dense.len(), 1);
        assert_eq!(decs_manager.archetypes[1].dense[0], 0);
        assert_eq!(decs_manager.archetypes[1].sparse[0], Some(0));
        assert_eq!(decs_manager.archetypes[1].type_flags, vec![true]);

        // -------------------------------------------------------------------------------

        let component = 
            IsPlayerComponent{};
        decs_manager.add_component(&entity, component);

        // Manager
        assert_eq!(decs_manager.archetypes.len(), 3);
        assert_eq!(decs_manager.sparse[0], 2);
        assert_eq!(decs_manager.component_type_registry.counter, 2);
        
        // Type Registry
        assert_eq!(decs_manager.component_type_registry.get_uuid_of(&std::any::TypeId::of::<NameComponent>()), Some(0));
        assert_eq!(decs_manager.component_type_registry.get_uuid_of(&std::any::TypeId::of::<IsPlayerComponent>()), Some(1));

        // Archetype 0
        assert_eq!(decs_manager.archetypes[1].dense.len(), 0);
        assert_eq!(decs_manager.archetypes[1].sparse[0], None);
        assert_eq!(decs_manager.archetypes[1].type_flags, vec![true, false]);

        // Archetype 1
        assert_eq!(decs_manager.archetypes[2].dense.len(), 1);
        assert_eq!(decs_manager.archetypes[2].dense[0], 0);
        assert_eq!(decs_manager.archetypes[2].sparse[0], Some(0));
        assert_eq!(decs_manager.archetypes[2].type_flags, vec![true, true]);

    }

    #[test]
    fn add_components_multiple_entities() 
    {
        let mut decs_manager = manager::dECSManager::new();

        let entity = decs_manager.add_entity();

        let component = 
            NameComponent{
                0: "Pookie".to_owned()
            };
        decs_manager.add_component(&entity, component);

        assert_eq!(decs_manager.archetypes.len(), 2);
        assert_eq!(decs_manager.sparse[0], 1);
        assert_eq!(decs_manager.component_type_registry.counter, 1);
        
        assert_eq!(decs_manager.component_type_registry.get_uuid_of(&std::any::TypeId::of::<NameComponent>()), Some(0));

        let component = 
            IsPlayerComponent{};
        decs_manager.add_component(&entity, component);

        assert_eq!(decs_manager.archetypes.len(), 3);
        assert_eq!(decs_manager.sparse[0], 2);
        assert_eq!(decs_manager.component_type_registry.counter, 2);
        
        assert_eq!(decs_manager.component_type_registry.get_uuid_of(&std::any::TypeId::of::<NameComponent>()), Some(0));
        assert_eq!(decs_manager.component_type_registry.get_uuid_of(&std::any::TypeId::of::<IsPlayerComponent>()), Some(1));
    }
}