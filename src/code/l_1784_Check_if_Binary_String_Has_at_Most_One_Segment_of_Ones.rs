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
    pub fn check_ones_segment(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let mut cnt = 0;
        let mut pre = '#';
        for i in 0..s.len() {
            if s[i] == '1' && pre != '1' {
                cnt += 1;
            }
            pre = s[i];
        }
        cnt <= 1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
