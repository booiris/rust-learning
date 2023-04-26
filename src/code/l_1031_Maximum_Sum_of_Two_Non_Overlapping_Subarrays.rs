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

fn get(nums: &[i32], len: usize) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let mut now = 0;
    for i in 0..len {
        now += nums[i];
    }
    let mut maxn = now;
    res[len - 1] = maxn;
    for i in len..nums.len() {
        now -= nums[i - len];
        now += nums[i];
        maxn = maxn.max(now);
        res[i] = maxn;
    }
    res
}

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let rev = nums.iter().map(|x| *x).rev().collect::<Vec<_>>();
        let mut res = 0;
        let n1 = get(&nums, first_len as usize);
        let n2 = get(&rev, second_len as usize);
        // println!("{:?}\n{:?}", n1, n2);
        for i in first_len as usize..=nums.len() - second_len as usize {
            res = res.max(n1[i - 1] + n2[nums.len() - i - 1]);
        }
        let n1 = get(&nums, second_len as usize);
        let n2 = get(&rev, first_len as usize);
        // println!("{:?}\n{:?}", n1, n2);
        for i in second_len as usize..=nums.len() - first_len as usize {
            res = res.max(n1[i - 1] + n2[nums.len() - i - 1]);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = [2, 1, 5, 6, 0, 9, 5, 0, 3, 8];
    println!("res:{}", Solution::max_sum_two_no_overlap(a.into(), 4, 3));
}
