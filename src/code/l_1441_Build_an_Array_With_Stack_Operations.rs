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
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut now = 1;
        let mut res = vec![];
        for x in target {
            while now != x {
                res.push("Push".to_owned());
                res.push("Pop".to_owned());
                now += 1;
            }
            now += 1;
            res.push("Push".to_owned());
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
