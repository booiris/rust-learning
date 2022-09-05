#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;

struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums.reverse();
        let mut sum = nums.iter().sum();
        let mut res = vec![];
        let mut now = 0;
        for num in nums.iter() {
            now += *num;
            sum -= *num;
            res.push(*num);
            if now > sum {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
pub fn main() {
    println!("res:{:?}", Solution::min_subsequence(vec![4, 3, 10, 9, 8]));
}
