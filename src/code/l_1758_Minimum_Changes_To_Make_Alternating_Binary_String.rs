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
    pub fn min_operations(s: String) -> i32 {
        let mut cnt1 = 0;
        let mut now = false;
        for x in s.chars() {
            if now && x != '0' {
                cnt1 += 1;
            } else if !now && x != '1' {
                cnt1 += 1;
            }
            now = !now;
        }
        let mut cnt2 = 0;
        now = true;
        for x in s.chars() {
            if now && x != '0' {
                cnt2 += 1;
            } else if !now && x != '1' {
                cnt2 += 1;
            }
            now = !now;
        }
        min(cnt1, cnt2)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
