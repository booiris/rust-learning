use screeps_arena::Creep;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[inline]
pub fn split_creep(creeps: &Vec<Creep>) -> (Vec<&Creep>, Vec<&Creep>) {
    let mut my_creeps = vec![];
    let mut other_creeps = vec![];
    for creep in creeps {
        if creep.my() {
            my_creeps.push(creep);
        } else {
            other_creeps.push(creep);
        }
    }
    (my_creeps, other_creeps)
}
