use screeps::*;

use crate::model::{long_memory::CreepLongMemory, memory::CreepMemory};
pub trait RoleAction<T: HasPosition + MaybeHasId> {
    fn find_target(&mut self) -> T;
    fn work(&self, target: T) -> ReturnCode;
    fn memory(&mut self) -> &mut CreepMemory;
    fn long_memory(&mut self) -> &mut CreepLongMemory;
    fn creep(&self) -> &Creep;
    fn action_move(&mut self, target: &T, id: RawObjectId);

    fn do_target(&mut self, target: T) {
        if let Some(id) = target.try_raw_id() {
            if let Some(need_move) = self.long_memory().need_move {
                if need_move {
                    self.action_move(&target, id);
                }
            } else {
                self.long_memory().need_move = Some(false);
            }
            let res = self.work(target);
            if res == ReturnCode::NotInRange {
                self.long_memory().need_move = Some(true);
            }
        }
    }
    fn run(&mut self) {
        let x: T = self.find_target();
        self.do_target(x);
    }
}
