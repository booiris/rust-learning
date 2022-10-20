use super::utils;
use js_sys::Array;
use log::*;
use screeps::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::model::memory::CreepMemory;

pub trait Action {
    fn custom_move<T>(&self, target: &T, mem: &mut CreepMemory)
    where
        T: HasPosition;
}

// fn find_func(room_name: local::RoomName) -> pathfinder::MultiRoomCostResult {
//     let mut cost = objects::CostMatrix::new();
//     let room = game::rooms().get(room_name);
//     if room.is_none() {
//         return pathfinder::MultiRoomCostResult::Default;
//     }
//     info!("{:?}", cost.get_bits());
//     let room = room.unwrap();
//     let structs = room.find(constants::find::RoomObject::Structures);
//     // for s_struct in structs{
//     //     if s_struct.ge(other)
//     // }

//     pathfinder::MultiRoomCostResult::CostMatrix(cost)
// }

impl Action for Creep {
    fn custom_move<T>(&self, target: &T, mem: &mut CreepMemory)
    where
        T: HasPosition,
    {
        let path;
        if let Some(mem_path) = &mem.path {
            path = JsValue::from(mem_path);
        } else {
            path = JsValue::from(find_path(self, target.pos(), mem));
        }

        let res = self.move_by_path(&path);
        debug!("{:?}", res);
        if res != ReturnCode::Ok {
            let path = find_path(self, target.pos(), mem);
            self.move_by_path(&path);
        }
    }
}
#[derive(Serialize, Deserialize)]
struct FindOptions {
    range: i32,
}

fn find_path(creep: &Creep, to: Position, mem: &mut CreepMemory) -> Array {
    let path = creep.room().unwrap().find_path(
        &creep.pos().into(),
        &to.into(),
        Some(&utils::change_to_object(FindOptions { range: 1 })),
    );
    mem.path = Some(Room::serialize_path(&path));
    path
}
