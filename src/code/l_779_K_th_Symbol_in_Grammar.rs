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
    pub fn kth_grammar(n: i32, mut k: i32) -> i32 {
        k -= 1;
        let mut now = 0;
        for _ in 0..n - 1 {
            if k & 1 == 1 {
                now = !now;
            }
            k >>= 1;
        }
        if now != 0 {
            return 1;
        } else {
            return 0;
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
