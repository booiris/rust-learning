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
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();
        let key = arr.len() / 20;
        let sum: i32 = arr[key..arr.len() - key].iter().sum();
        sum as f64 / (arr.len() - key * 2) as f64
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
