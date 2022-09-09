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
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut index = 0;
        for x in logs {
            if x == "./" {
                continue;
            } else if x == "../" {
                index -= 1;
            } else {
                index += 1;
            }
        }
        max(0, index)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
