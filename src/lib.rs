pub mod collect;
pub mod ctf;
mod logging;
pub mod swamp;
mod utils;

use js_sys::Array;
use screeps_arena::{
    constants::{prototypes, Part, ResourceType, ReturnCode},
    game,
    prelude::*,
    Creep,
};
use utils::*;
use wasm_bindgen::prelude::*;

fn setup() {
    logging::setup_logging(logging::Info);
}

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup();
    }

    log!("now tick: {}", tick);

    // let info = game::arena_info();
    // info!("arena_info: {:?}", info);

    #[cfg(feature = "tutorial")]
    {
        // tutorial4
        // let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
        // let (my_creeps, other_creeps) = split_creep(&creeps);
        // let enemy_creep = other_creeps[0];

        // for creep in &my_creeps {
        //     let parts = creep.body();
        //     if parts.iter().find(|x| x.part() == Part::Attack).is_some() {
        //         creep.move_to(enemy_creep, None);
        //         creep.attack(enemy_creep);
        //     }
        //     if parts
        //         .iter()
        //         .find(|x| x.part() == Part::RangedAttack)
        //         .is_some()
        //     {
        //         creep.move_to(enemy_creep, None);
        //         creep.ranged_attack(enemy_creep);
        //     }
        //     if parts.iter().find(|x| x.part() == Part::Heal).is_some() {
        //         if let Some(heart_creep) = my_creeps.iter().find(|x| x.hits() < x.hits_max()) {
        //             creep.move_to(heart_creep, None);
        //             creep.heal(heart_creep);
        //         }
        //     }
        // }

        // tutorial5
        // let towers = game::utils::get_objects_by_prototype(prototypes::STRUCTURE_TOWER);
        // let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
        // let (my_creeps, enemys) = split_creep(&creeps);
        // let tower = towers.first().unwrap();
        // if tower.store().get(ResourceType::Energy).unwrap() < 10u32 {
        //     let my_creep = my_creeps[0];
        //     if my_creep.store().get(ResourceType::Energy).unwrap() == 0 {
        //         let containers =
        //             game::utils::get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
        //         let container = containers.first().unwrap();
        //         let res = my_creep.withdraw(container, ResourceType::Energy, None);
        //         log!("{:?}", res);
        //     } else {
        //         my_creep.move_to(tower, None);
        //         let res = my_creep.transfer(tower, ResourceType::Energy, None);
        //         log!("{:?}", res);
        //     }
        // } else {
        //     let target = enemys[0];
        //     tower.attack(target);
        // }

        // tutorial6
        // let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
        // let flags = game::utils::get_objects_by_prototype(prototypes::FLAG)
        //     .iter()
        //     .collect::<Array>();
        // for creep in creeps {
        //     let flag = creep.find_closest_by_path(&flags, None).unwrap();
        //     creep.move_to(&JsValue::from(flag), None);
        // }

        // tutorial7
        
    }
}
