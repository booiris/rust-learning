#![allow(dead_code, unused_imports, unused_macros)]
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
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut key = vec![];
        for x in box_types {
            for _ in 0..x[0] {
                key.push(x[1]);
            }
        }
        key.sort_unstable_by_key(|x| Reverse(*x));
        (&key[0..(truck_size as usize).min(key.len())]).iter().sum()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
