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
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let a = nums1.into_iter().collect::<HashSet<_>>();
        let b = nums2.into_iter().collect::<HashSet<_>>();
        let c = nums3.into_iter().collect::<HashSet<_>>();
        let a1 = a.intersection(&b).collect::<HashSet<_>>();
        let a2 = b.intersection(&c).collect::<HashSet<_>>();
        let a3 = a.intersection(&c).collect::<HashSet<_>>();
        a1.union(&a2)
            .map(|x| *x)
            .collect::<HashSet<_>>()
            .union(&a3)
            .map(|x| **x)
            .collect()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
