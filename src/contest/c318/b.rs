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
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut key = HashSet::new();
        let mut res = 0;
        let (mut i, mut j) = (0, 0);
        let mut now = 0;
        while i <= j && j < nums.len() {
            let mut bad = -1;
            while j - i < k && j < nums.len() {
                if key.get(&nums[j]).is_some() {
                    bad = nums[j];
                    break;
                }
                key.insert(nums[j]);
                now += nums[j] as i64;
                j += 1;
            }
            if j - i == k {
                res = now.max(res);
            }
            if bad > 0 {
                while i <= j {
                    if nums[i] == bad {
                        break;
                    }
                    key.remove(&nums[i]);
                    now -= nums[i] as i64;
                    i += 1;
                }
            }
            key.remove(&nums[i]);
            now -= nums[i] as i64;
            i += 1;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![4, 4, 4];
    println!("res:{}", Solution::maximum_subarray_sum(a, 3));
}
