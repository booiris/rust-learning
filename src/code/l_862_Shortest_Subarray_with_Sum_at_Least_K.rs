#![allow(dead_code, unused_imports, unused_macros)]
use ron::from_str;
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
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = usize::MAX;

        let mut key = VecDeque::<(i64, usize)>::new();
        let mut sum = vec![0; nums.len() + 1];
        for (i, x) in nums.iter().enumerate() {
            sum[i + 1] = sum[i] + *x as i64;
        }

        for (i, &x) in sum.iter().enumerate() {
            while let Some(back) = key.back() {
                if back.0 < x {
                    break;
                }
                key.pop_back();
            }
            key.push_back((x, i));
            while let Some(front) = key.front() {
                if x - front.0 < k as i64 {
                    break;
                }
                res = res.min(i - front.1);
                key.pop_front();
            }
        }
        if res == usize::MAX {
            -1
        } else {
            res as i32
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    //     [17,85,93,-45,-21]
    // 150
    let a = vec![17, 85, 93, -45, -21];
    println!("res:{}", Solution::shortest_subarray(a, 150));
}
