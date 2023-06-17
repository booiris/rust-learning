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
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if n % 2 == 0 {
            n / 2
        } else {
            n
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
