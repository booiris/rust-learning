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
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut key = [(-1, -1), (-1, 0), (-1, 1), (0, 0), (1, -1), (1, 0), (1, 1)];
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut sum = 0;
                let mut flag = true;
                for k in key {
                    let (nx, ny) = (i as i32 + k.0, j as i32 + k.1);
                    if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                        flag = false;
                        break;
                    }
                    sum += grid[nx as usize][ny as usize];
                }
                if flag {
                    res = res.max(sum);
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
