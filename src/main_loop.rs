use crate::model::long_memory::CreepLongMemory;
use crate::model::memory::*;
use crate::role::role_action::RoleAction;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::role::{self, *};
use crate::utils::utils::*;
use log::*;
use screeps::*;
use wasm_bindgen::prelude::*;

use crate::logging;

#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Info);
}

// this is one way to persist data between ticks within Rust's memory, as opposed to
// keeping state in memory on game objects - but will be lost on global resets!
thread_local! {
    static MEMORY: RefCell<Memory>= RefCell::new(Memory {
        creep_memory:  HashMap::new() ,
        room_memory: HashMap::new() ,
    });
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    debug!("loop starting! CPU: {}", game::cpu::get_used());
    MEMORY.with(|m| {
        let mut m = m.borrow_mut();
        run(&mut m);
    });

    info!("done! cpu: {}", game::cpu::get_used())
}

fn run(memory: &mut Memory) {
    for creep in game::creeps().values() {
        run_creep(&creep, &mut memory.creep_memory);
    }

    let mut additional = 0;

    for spawn in game::spawns().values() {
        debug!("running spawn {}", String::from(spawn.name()));

        let body = [Part::Move, Part::Move, Part::Carry, Part::Work];
        if spawn.room().unwrap().energy_available() >= body.iter().map(|p| p.cost()).sum() {
            let time = game::time();
            let name = format!("{}-{}", time, additional);
            let res = spawn.spawn_creep(&body, &name);
            if res != ReturnCode::Ok {
                warn!("couldn't spawn: {:?}", res);
            } else {
                let role = Role::Harvester;
                game::creeps()
                    .get(name)
                    .unwrap()
                    .set_memory(&struct_to_js_value(CreepLongMemory::new(role)));
                additional += 1;
            }
        }
    }
}

fn run_creep(creep: &Creep, memorys: &mut HashMap<RawObjectId, CreepMemory>) {
    if creep.spawning() {
        return;
    }
    let id = creep.try_raw_id().unwrap();

    let mut mem = memorys.entry(id).or_insert(CreepMemory::new());
    let mut long_mem: CreepLongMemory = js_value_to_struct(creep.memory());
    if let Some(role) = &long_mem.role {
        match role {
            Role::Harvester => {
                role::harvester::Harvester::new(creep, &mut mem, &mut long_mem).run()
            }
            Role::Builder => role::builder::Builder::new(creep, &mut mem, &mut long_mem).run(),
            Role::Upgrader => role::upgrader::Upgrader::new(creep, &mut mem, &mut long_mem).run(),
        }
    } else {
        long_mem.role = Some(Role::Builder);
    }
    creep.set_memory(&struct_to_js_value(long_mem));

    // let room = creep.room().expect("couldn't resolve creep room");
    // if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
    //     for structure in room.find(find::STRUCTURES).iter() {
    //         if let StructureObject::StructureController(controller) = structure {
    //             creep.custom_move(&controller, controller.raw_id(), &mut mem);
    //         }
    //     }
    // } else if let Some(source) = room.find(find::SOURCES_ACTIVE).get(0) {
    //     creep.custom_move(source, source.raw_id(), &mut mem);
    // }

    // let target = creep_targets.remove(&id);
    // match target {
    //     Some(creep_target) => {
    //         let keep_target = match creep_target.target {
    //             CreepTarget::Upgrade(controller_id) => {
    //                 if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
    //                     match controller_id.resolve() {
    //                         Some(controller) => {
    //                             let r = creep.upgrade_controller(&controller);
    //                             if r == ReturnCode::NotInRange {
    //                                 creep.move_to(&controller);
    //                                 true
    //                             } else if r != ReturnCode::Ok {
    //                                 warn!("couldn't upgrade: {:?}", r);
    //                                 false
    //                             } else {
    //                                 true
    //                             }
    //                         }
    //                         None => false,
    //                     }
    //                 } else {
    //                     false
    //                 }
    //             }
    //             CreepTarget::Harvest(source_id) => {
    //                 if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
    //                     match source_id.resolve() {
    //                         Some(source) => {
    //                             if creep.pos().is_near_to(source.pos()) {
    //                                 let r = creep.harvest(&source);
    //                                 if r != ReturnCode::Ok {
    //                                     warn!("couldn't harvest: {:?}", r);
    //                                     false
    //                                 } else {
    //                                     true
    //                                 }
    //                             } else {
    //                                 creep.move_to(&source);
    //                                 true
    //                             }
    //                         }
    //                         None => false,
    //                     }
    //                 } else {
    //                     false
    //                 }
    //             }
    //         };

    //         if keep_target {
    //             creep_targets.insert(id, creep_target);
    //         }
    //     }
    //     None => {
    //         // no target, let's find one depending on if we have energy
    //         let room = creep.room().expect("couldn't resolve creep room");
    //         if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
    //             for structure in room.find(find::STRUCTURES).iter() {
    //                 if let StructureObject::StructureController(controller) = structure {
    //                     creep_targets.insert(
    //                         id,
    //                         CreepMemory {
    //                             target: CreepTarget::Upgrade(controller.id()),
    //                         },
    //                     );
    //                     break;
    //                 }
    //             }
    //         } else if let Some(source) = room.find(find::SOURCES_ACTIVE).get(0) {
    //             creep_targets.insert(
    //                 id,
    //                 CreepMemory {
    //                     target: CreepTarget::Harvest(source.id()),
    //                 },
    //             );
    //         }
    //     }
    // }
}
