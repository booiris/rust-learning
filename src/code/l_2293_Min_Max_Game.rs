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
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        while n != 1 {
            let mut index = 0;
            for i in (0..n).step_by(2) {
                let now = if (i / 2) % 2 == 0 {
                    min(nums[i], nums[i + 1])
                } else {
                    max(nums[i], nums[i + 1])
                };
                nums[index] = now;
                index += 1;
            }
            n /= 2;
        }
        nums[0]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
