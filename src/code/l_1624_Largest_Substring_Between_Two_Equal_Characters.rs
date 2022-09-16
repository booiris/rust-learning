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
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut key = [-1; 30];
        let mut res = -1;
        for (i, c) in s.chars().enumerate() {
            let now = key.get_mut((c as u8 - 'a' as u8) as usize).unwrap();
            if *now != -1 {
                res = max(res, i as i32 - *now - 1);
            } else {
                *now = i as i32;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
