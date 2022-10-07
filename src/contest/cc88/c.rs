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
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut key1 = 0;
        nums1.iter().for_each(|x| key1 ^= x);
        let mut key2 = 0;
        nums2.iter().for_each(|x| key2 ^= x);
        if nums2.len() % 2 == 1 {
            if nums1.len() % 2 == 0 {
                return key1;
            } else {
                return key1 ^ key2;
            }
        } else {
            if nums1.len() % 2 == 0 {
                return 0;
            } else {
                return key2;
            }
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
