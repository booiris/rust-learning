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
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        match n {
            0 => 1, // trival
            1 => if presses >= 1 { 2 } else { 1 },
            2 => match presses {
                0 => 1,
                1 => 3,
                _ => 4,
            },
            n if n >= 3 => match presses {
                0 => 1,
                1 => 4,
                2 => 7,
                _ => 8,
            }
            _ => panic!(),
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}


