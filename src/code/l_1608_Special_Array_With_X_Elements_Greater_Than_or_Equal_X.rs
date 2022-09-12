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
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for (i, x) in nums.iter().enumerate() {
            let key = (nums.len() - i) as i32;
            if key <= *x && ((i > 0 && key > nums[i - 1]) || i == 0) {
                return key;
            }
        }
        -1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 2, 3, 4, 5, 6, 7];
    println!("res:{}", Solution::special_array(a));
}
