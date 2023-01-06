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
    pub fn count_even(num: i32) -> i32 {
        let mut res = 0;
        for i in 1..=num {
            let mut now = i;
            let mut sum = 0;
            while now != 0 {
                sum += now % 10;
                now /= 10;
            }
            if sum % 2 == 0 {
                res += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
