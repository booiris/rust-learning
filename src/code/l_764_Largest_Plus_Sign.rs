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
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut key = vec![vec![1; n]; n];
        let mut res = 0;
        for x in mines {
            key[x[0] as usize][x[1] as usize] = 0;
        }

        let n = n as i32;
        for i in 0..n {
            for j in 0..n {
                let mut k1 = 0;
                while k1 + j < n
                    && j - k1 >= 0
                    && key[i as usize][(k1 + j) as usize] == 1
                    && key[i as usize][(j - k1) as usize] == 1
                {
                    k1 += 1;
                }
                let mut k2 = 0;
                while k2 + i < n
                    && i - k2 >= 0
                    && key[(k2 + i) as usize][j as usize] == 1
                    && key[(i - k2) as usize][j as usize] == 1
                {
                    k2 += 1;
                }
                res = max(min(k1, k2), res);
            }
        }
        res as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
