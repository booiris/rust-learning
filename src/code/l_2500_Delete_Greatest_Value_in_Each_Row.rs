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
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|x| x.sort_unstable());
        let mut res = 0;
        for i in 0..grid[0].len() {
            let mut max = 0;
            for j in 0..grid.len() {
                max = max.max(grid[j][i]);
            }
            res += max;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
