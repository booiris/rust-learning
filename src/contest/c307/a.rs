#![allow(dead_code, unused_imports, unused_macros)]
use std::borrow::Borrow;
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
#[cfg(feature = "local")]
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut aim_e = 1;
        for x in energy {
            aim_e += x;
        }
        let mut res = 0;
        if initial_energy < aim_e {
            res += aim_e - initial_energy;
        }
        for s in (initial_experience..10001).rev() {
            let mut now = s;
            for x in experience.iter() {
                if x >= &now {
                    if s >= initial_experience {
                        res += s - initial_experience + 1;
                    }
                    return res;
                }
                now += x;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
