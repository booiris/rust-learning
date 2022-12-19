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
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_by(|x, y| (x.to_string() + y).cmp(&(y.to_string() + x)));
        nums.into_iter().fold("".to_string(), |mut now, x| {
            now += &x;
            now
        })
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
