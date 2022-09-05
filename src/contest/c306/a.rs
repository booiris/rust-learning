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
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; grid[0].len() - 2]; grid.len() - 2];
        let key = [
            [0, 1],
            [0, -1],
            [0, 0],
            [1, 0],
            [-1, 0],
            [1, 1],
            [1, -1],
            [-1, 1],
            [-1, -1],
        ];
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                let mut maxn = 0;
                for k in key {
                    let (nx, ny) = (i as i32 + k[0], j as i32 + k[1]);
                    maxn = max(maxn, grid[nx as usize][ny as usize]);
                }
                res[i - 1][j - 1] = maxn;
            }
        }
        return res;
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
