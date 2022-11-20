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
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut key = vec![poured as f64];
        for i in 1..=query_row {
            let mut temp = vec![0.0; (i as usize) + 1];
            for i in 0..key.len() {
                if key[i] > 1.0 {
                    temp[i] += (key[i] - 1.0) / 2.0;
                    temp[i + 1] += (key[i] - 1.0) / 2.0;
                }
            }
            key = temp;
        }
        if key[query_glass as usize] > 1.0 {
            1.0
        } else {
            key[query_glass as usize]
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
