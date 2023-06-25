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

const MAX_STATE: usize = 253;

impl Solution {
    pub fn get_max_grid_happiness(
        n: i32,
        m: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let mut dp: [[[[[i32; 1]; 7]; 7]; MAX_STATE]; 5] = [[[[[0]; 7]; 7]; MAX_STATE]; 5 as usize];
        let mut state: [(i32, i32, i32); MAX_STATE] = [(0, 0, 0); MAX_STATE];
        for i in 0..MAX_STATE {
            
        }

        todo!()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
