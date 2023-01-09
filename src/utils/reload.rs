use js_sys::JsString;
use screeps::Path::Vectorized;
use screeps::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::model::memory::CreepMemory;

pub trait Action {
    fn custom_move<T>(&self, target: &T, id: RawObjectId, mem: &mut CreepMemory)
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
    fn custom_move<T>(&self, target: &T, id: RawObjectId, mem: &mut CreepMemory)
    where
        T: HasPosition,
    {
        let path;
        if let Some(mem_target) = mem.target {
            if mem_target == id {
                if let Some(mem_path) = &mem.path {
                    path = JsValue::from(mem_path);
                } else {
                    path = JsValue::from(find_path(self, target.pos(), mem));
                }
            } else {
                mem.target = Some(id);
                path = JsValue::from(find_path(self, target.pos(), mem));
            }
        } else {
            mem.target = Some(id);
            path = JsValue::from(find_path(self, target.pos(), mem));
        }

        self.move_by_path(&path);

        if let Some(pre_pos) = mem.pre_pos {
            if pre_pos == self.pos() {
                mem.stay_times += 1;
            } else {
                mem.stay_times = 0;
            }
        }
        if mem.stay_times > mem.max_stay_times {
            let path = find_path(self, target.pos(), mem);
            self.move_by_path(&path);
        }
        mem.pre_pos = Some(self.pos());
    }
}
#[derive(Serialize, Deserialize)]
struct FindOptions {
    range: i32,
}

fn find_path(creep: &Creep, to: Position, mem: &mut CreepMemory) -> JsString {
    let option = FindPathOptions::<_, SingleRoomCostResult>::new()
        .range(1)
        .serialize(true);
    let path = creep
        .room()
        .unwrap()
        .find_path(&creep.pos().into(), &to.into(), Some(option));
    match path {
        Vectorized(path) => {
            let path = Room::serialize_path(&path);
            mem.path = Some(path.clone().into());
            path.into()
        }
        _ => {
            panic!()
        }
    }
}
