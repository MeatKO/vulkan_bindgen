#![feature(duration_millis_float)]
#![feature(trim_prefix_suffix)]

use core::panic;
use std::{marker::PhantomData, thread, time::{self, Duration, Instant}};
use rand::{Rng, rng};

use component_derive::{Component, system};
use decs2::{decs::{self, manager::dECSManager, query::QueryIterator, typedef::{Component, Entity}}, query::{Fetch, MaybeRead, MaybeWrite, Query, Read}};

#[derive(Component, Debug)]
struct NameComponent(pub String);

#[derive(Component, Debug)]
struct HealthComponent(pub f32);

#[derive(Component)]
struct IsPlayerComponent {}

#[derive(Component)]
struct IsNpcComponent {}

#[derive(Component)]
struct PositionComponent(pub f32, pub f32);

#[derive(Component)]
struct VelocityComponent(pub f32, pub f32);

#[derive(Component)]
struct ManaComponent(pub f32);

#[derive(Component)]
struct StaminaComponent(pub f32);

#[derive(Component)]
struct StrengthComponent(pub u32);

#[derive(Component)]
struct InventoryComponent(pub Vec<String>);

#[derive(Component)]
struct GoldComponent(pub u32);

#[derive(Component)]
struct ExperienceComponent(pub u64);

#[derive(Component)]
struct LevelComponent(pub u32);

#[derive(Component)]
struct RenderMeshComponent(pub String);

#[derive(Component)]
struct TransformComponent {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

#[derive(Component)]
struct InputComponent {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Component)]
struct AIStateComponent(pub u8);

#[derive(Component)]
struct QuestFlagsComponent(pub u64);

#[derive(Component)]
struct AttackCooldownComponent(pub f32);

#[derive(Component)]
struct MovementSpeedComponent(pub f32);

#[derive(Component)]
struct BuffsComponent(pub Vec<String>);

#[derive(Component)]
struct DebuffsComponent(pub Vec<String>);

fn main() 
{
    let mut decs_manager = dECSManager::new();
    let n = 1000;

    // Spawn empty entities
    let start = Instant::now();
    let entity_ids: Vec<_> = (0..n).map(|_| decs_manager.add_entity()).collect();
    let duration_spawn = start.elapsed();
    println!("Spawned {} entities in {:?}", n, duration_spawn);

    let start = Instant::now();

    let mut rng = rand::rng();

    let mut adds = 0usize;
    let mut removes = 0usize;

    for hui_guz in 0..10
    {
        for i in 0..n-1
        {
            let entity = Entity{0: i};

            for sex in 0..5
            {
                adds += 1;
                let choice = rng.random_range(0..24);

                match choice 
                {
                    0  => decs_manager.add_component(&entity, NameComponent(format!("Entity_{i}"))),
                    1  => decs_manager.add_component(&entity, HealthComponent(rng.random_range(0.0..100.0))),
                    2  => decs_manager.add_component(&entity, IsPlayerComponent {}),
                    3  => decs_manager.add_component(&entity, IsNpcComponent {}),
                    4  => decs_manager.add_component(&entity, PositionComponent(rng.random(), rng.random())),
                    5  => decs_manager.add_component(&entity, VelocityComponent(rng.random(), rng.random())),
                    6  => decs_manager.add_component(&entity, ManaComponent(rng.random_range(0.0..200.0))),
                    7  => decs_manager.add_component(&entity, StaminaComponent(rng.random_range(0.0..200.0))),
                    8  => decs_manager.add_component(&entity, StrengthComponent(rng.random_range(1..100))),
                    9  => decs_manager.add_component(&entity, InventoryComponent(vec!["Sword".into(), "Apple".into()])),
                    10 => decs_manager.add_component(&entity, GoldComponent(rng.random_range(0..99999))),
                    11 => decs_manager.add_component(&entity, ExperienceComponent(rng.random_range(0..1_000_000))),
                    12 => decs_manager.add_component(&entity, LevelComponent(rng.random_range(1..50))),
                    13 => decs_manager.add_component(&entity, RenderMeshComponent("MeshName".into())),
                    14 => decs_manager.add_component(&entity, TransformComponent {
                        x: rng.random(),
                        y: rng.random(),
                        rotation: rng.random(),
                    }),
                    15 => decs_manager.add_component(&entity, InputComponent {
                        up: rng.random_bool(0.5),
                        down: rng.random_bool(0.5),
                        left: rng.random_bool(0.5),
                        right: rng.random_bool(0.5),
                    }),
                    16 => decs_manager.add_component(&entity, AIStateComponent(rng.random_range(0..10))),
                    17 => decs_manager.add_component(&entity, QuestFlagsComponent(rng.random_range(0..u64::MAX))),
                    18 => decs_manager.add_component(&entity, AttackCooldownComponent(rng.random_range(0.0..10.0))),
                    19 => decs_manager.add_component(&entity, MovementSpeedComponent(rng.random_range(0.0..10.0))),
                    20 => decs_manager.add_component(&entity, BuffsComponent(vec!["Regeneration".into()])),
                    21 => decs_manager.add_component(&entity, DebuffsComponent(vec!["Poison".into()])),
                    22 => {
                        // optional: add more dynamic content for testing
                        let choice_vec = vec!["Extra1".into(), "Extra2".into()];
                        decs_manager.add_component(&entity, InventoryComponent(choice_vec.clone()));
                    }
                    23 => {
                        decs_manager.add_component(&entity, GoldComponent(rng.random_range(0..50000)));
                    }
                    _ => unreachable!(),
                }
            }

            // randomly remove components to force archetype churn
            if rng.random_bool(0.5) {
                removes += 1;
                decs_manager.remove_component::<PositionComponent>(&entity);
            }
            if rng.random_bool(0.5) {
                removes += 1;
                decs_manager.remove_component::<VelocityComponent>(&entity);
            }
            if rng.random_bool(0.3) {
                removes += 1;
                decs_manager.remove_component::<InventoryComponent>(&entity);
            }
            if rng.random_bool(0.3) {
                removes += 1;
                decs_manager.remove_component::<NameComponent>(&entity);
            }

            // println!("Entity : {}", entity.0);
            // println!("Time passed : {}", time::Instant::now().duration_since(start).as_secs_f32());
            // println!("Archetype count : [{}]\nEntity count : [{}]\n", decs_manager.archetypes.len(), decs_manager.sparse.len());
            // println!("Total frames, if 16.6ms : {}", time::Instant::now().duration_since(start).as_millis_f32() / 16.0f32);
            // println!("Total entities per frame , if 16.6ms : {}", entity.0 as f32 / (time::Instant::now().duration_since(start).as_millis_f32() / 16.0f32));

            // system(&mut decs_manager);
        }
    }

    let duration_add = start.elapsed();
    // println!("Added 1 random component to {} entities in {:?}", n, duration_add);

    // decs_manager.print_typenames();
    // decs_manager.print_archetypes();

    println!("Spawned {} entities in {:?}", n, duration_spawn);
    // println!("Added 1 component to {} entities in {:?}", n, duration_add);
    println!("Additions {:?}", adds);
    println!("Removals {:?}", removes);
    // println!("Components per ms : {}", n as f32 / duration_add.as_millis_f32());
    println!("Component OPs total time : {:?}", duration_add);
    println!("Component OPs per ms : {}", (adds + removes) as f32 / duration_add.as_millis_f32());

    // for x in 0..
    // {
    //     system(&mut decs_manager);

    //     let iter_entity = decs_manager.add_entity();
    //     let name_component =  NameComponent(format!("Iter_{}", x).to_owned());
    //     decs_manager.add_component(&iter_entity, name_component);

    //     // decs_manager.print_archetypes();

    //     // thread::sleep(Duration::from_millis(1));

    //     stress_test(&mut decs_manager, 10);

    //     println!("Time passed : {}", time::Instant::now().duration_since(start).as_secs_f32());
    //     println!("Archetype count : [{}]\nEntity count : [{}]\n", decs_manager.archetypes.len(), decs_manager.sparse.len());
    // }
}

// #[system]
// fn system(&mut self)
// {

// }
// #[system]
// pub unsafe fn system(npcs: Query::<(&NameComponent, &mut HealthComponent)>, players: Query::<(Option<&NameComponent>, Option<&mut IsPlayerComponent>)>)
// {
//     for npc in npcs
//     {

//     }
// }

#[system]
fn system(players: Query::<(Option<&NameComponent>,)>)
{
    println!("Printing deez nuts");

    for (name) in players.iter()
    {
        println!("Player '{:?}'", name);
    }
}
// #[system]
// fn system(players: Query::<(Option<&NameComponent>, Option<&mut HealthComponent>)>)
// {
//     println!("Printing deez nuts");

//     for (name, health) in players.iter()
//     {
//         println!("Player '{:?}', has {:?}HP", name, health);

//         if let Some(health) = health
//         {
//             health.0 = 10.0f32;
//         }
//     }
// }
// #[system]
// fn system(players: Query::<(&self::NameComponent, &mut HealthComponent)>)
// {
//     println!("Printing deez nuts");

//     for (name, health) in players.iter()
//     {
//         println!("Player '{}', has {}HP", name.0, health.0);
//     }
// }

// fn system(decs: &mut dECSManager)
// {
//     decs.register_type::<NameComponent>();
//     decs.register_type::<HealthComponent>();
//     decs.register_type::<IsPlayerComponent>();

//     let npcs = Query::<(Read<NameComponent>, Read<HealthComponent>)>::new(decs);
//     let players= Query::<(MaybeRead<NameComponent>, MaybeWrite<IsPlayerComponent>)>::new(decs);

// }
//     let mut hugh_jazz: u128 = 0u128;
    
//     println!("Name + Health : ");
//     for (name, health) in npcs.iter()
//     {
//         println!("{} - {}", name.0, health.0);
//     }

//     println!("Name");
//     for (name) in player.iter()
//     {
//         println!("{}", name.0);

//         hugh_jazz += name.0.trim_prefix("Entity_").parse::<u32>().unwrap() as u128;
//     }
//     println!("Hugh jazz: {}", hugh_jazz);
// }

fn create_entities(decs: &mut dECSManager, count: usize) 
{
    let mut hui = 0;
    for x in 0..count
    {
        decs.add_entity();
        hui += 1;
    }

    println!("done. {}", hui)
}

fn stress_test(decs: &mut dECSManager, steps: usize) 
{
    let entity = decs.add_entity();

    // list of component constructors so we can randomly pick them
    let mut rng = rand::rng();

    for i in 0..steps {
        let choice = rng.random_range(0..10);

        match choice {
            0 => decs.add_component(&entity, NameComponent(format!("Entity_{i}"))),
            1 => decs.add_component(&entity, HealthComponent(rng.random_range(0.0..100.0))),
            2 => decs.add_component(&entity, ManaComponent(rng.random_range(0.0..200.0))),
            3 => decs.add_component(&entity, PositionComponent(rng.random(), rng.random())),
            4 => decs.add_component(&entity, VelocityComponent(rng.random(), rng.random())),
            5 => decs.add_component(&entity, InventoryComponent(vec!["Sword".into(), "Apple".into()])),
            6 => decs.add_component(&entity, BuffsComponent(vec!["Regeneration".into()])),
            7 => decs.add_component(&entity, DebuffsComponent(vec!["Poison".into()])),
            8 => decs.add_component(&entity, GoldComponent(rng.random_range(0..99999))),
            9 => decs.add_component(&entity, LevelComponent(rng.random_range(1..50))),
            _ => unreachable!(),
        }

        // randomly remove components to force archetype churn
        if rng.random_bool(0.5) {
            decs.remove_component::<PositionComponent>(&entity);
        }
        if rng.random_bool(0.5) {
            decs.remove_component::<VelocityComponent>(&entity);
        }
        if rng.random_bool(0.3) {
            decs.remove_component::<InventoryComponent>(&entity);
        }
    }
}