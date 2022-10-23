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
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut ans1, mut ans2) = (0, 0);

        let mut pre = false;
        for i in 0..nums1.len() - 1 {
            if max(nums1[i], nums2[i]) < min(nums1[i + 1], nums2[i + 1]) {
                pre = false;
                continue;
            }
            if !pre && min(nums1[i], nums2[i]) != nums1[i] {
                println!("{} {}", nums1[i], i);
                ans1 += 1;
            }
            if !pre && min(nums1[i + 1], nums2[i + 1]) != nums1[i + 1] {
                ans1 += 1;
                pre = true;
            }
        }

        pre = false;
        for i in 0..nums1.len() - 1 {
            if max(nums1[i], nums2[i]) < min(nums1[i + 1], nums2[i + 1]) {
                pre = false;
                continue;
            }
            if !pre && max(nums1[i], nums2[i]) != nums1[i] {
                ans2 += 1;
            }
            if max(nums1[i + 1], nums2[i + 1]) != nums1[i + 1] {
                ans2 += 1;
                pre = true;
            }
        }

        println!("{} {} ", ans1, ans2);

        ans1.min(ans2)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    // [1,2,3,7,11,13,14,17,18,20]
    // [1,5,6,7,8,9,13,15,18,19]
    let a = vec![1, 2, 3, 7, 11, 13, 14, 17, 18, 20];
    let b = vec![1, 5, 6, 7, 8, 9, 13, 15, 18, 19];
    println!("res:{}", Solution::min_swap(a, b));
}
