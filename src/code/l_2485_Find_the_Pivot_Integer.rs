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
    pub fn pivot_integer(n: i32) -> i32 {
        for x in 1..=n {
            if (1 + x) * x == (n + x) * (n - x + 1) {
                return x;
            }
        }
        return -1;
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
