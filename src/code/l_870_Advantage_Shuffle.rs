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
    pub fn advantage_count(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r, mut cache, mut ret) = (
            0,
            nums1.len() - 1,
            (0..nums1.len()).collect::<Vec<_>>(),
            vec![0; nums1.len()],
        );
        nums1.sort();
        cache.sort_by(|&a, &b| nums2[b].cmp(&nums2[a]));

        for i in 0..nums1.len() {
            ret[cache[i]] = if nums1[r] > nums2[cache[i]] {
                nums1[r]
            } else {
                nums1[l]
            };
            if nums1[r] > nums2[cache[i]] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        ret
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![12, 24, 8, 32];
    let b = vec![13, 25, 32, 11];
    println!("res:{:?}", Solution::advantage_count(a, b));
}
