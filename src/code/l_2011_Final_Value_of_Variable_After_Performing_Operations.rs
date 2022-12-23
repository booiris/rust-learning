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
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0;
        for x in operations {
            if x.contains('+') {
                res += 1
            } else {
                res -= 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
