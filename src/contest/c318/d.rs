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
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {

    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}


