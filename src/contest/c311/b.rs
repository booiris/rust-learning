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
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut now = 0;
        let mut pre = '#';
        let mut res = 0;
        for (i, c) in s.chars().enumerate() {
            if pre == '#' {
                now += 1;
            } else {
                if pre as u8 + 1 == c as u8 {
                    now = now + 1;
                } else {
                    now = 1;
                }
            }
            res = max(res, now);
            pre = c;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
