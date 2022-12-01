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
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .into_iter()
            .enumerate()
            .fold((i32::MAX, -1), |now, i| {
                if x == i.1[0] && now.0 > i32::abs(y - i.1[1]) {
                    (i32::abs(y - i.1[1]), i.0 as i32)
                } else if y == i.1[1] && now.0 > i32::abs(x - i.1[0]) {
                    (i32::abs(x - i.1[0]), i.0 as i32)
                } else {
                    now
                }
            })
            .1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
