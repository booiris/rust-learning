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
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut key = HashMap::<i32, i32>::new();
        nums.iter().for_each(|x| {
            key.entry(*x).and_modify(|c| *c += 1).or_insert(1);
        });
        nums.sort_unstable_by_key(|&x| (key[&x], Reverse(x)));
        nums
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
