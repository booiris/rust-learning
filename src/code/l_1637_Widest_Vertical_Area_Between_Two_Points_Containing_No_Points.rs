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
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|x| x[0]);
        points
            .windows(2)
            .into_iter()
            .map(|x| x[1][0] - x[0][0])
            .max()
            .unwrap()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
