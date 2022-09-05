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
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut key = vec![0; edges.len()];
        let mut res = 0;
        let mut maxn = 0;
        for i in 0..edges.len() {
            key[edges[i] as usize] += i;
        }
        for i in 0..key.len() {
            if maxn < key[i] {
                maxn = key[i];
                res = i;
            }
        }
        return res as i32;
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
