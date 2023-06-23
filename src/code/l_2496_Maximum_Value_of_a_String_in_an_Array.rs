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
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|x| match x.parse::<i32>() {
                Ok(x) => x,
                Err(_) => x.len() as i32,
            })
            .max()
            .unwrap()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
