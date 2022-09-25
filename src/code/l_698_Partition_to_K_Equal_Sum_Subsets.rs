// Solution
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
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        let aim = sum / k;
        let mut key = HashMap::new();
        let mut key1 = HashMap::new();
        nums.iter().for_each(|&x| {
            key.entry(x).and_modify(|c| *c += 1).or_insert(1);
        });
        for x in nums.iter() {
            if let Some(x) = key1.get_mut(x) {
                if *x > 0 {
                    *x -= 1;
                    continue;
                }
            }
            key.entry(*x).and_modify(|c| *c -= 1);
            if aim == *x {
                continue;
            }
            if key.get(&(aim - x)).is_none() || key[&(aim - x)] == 0 {
                return false;
            } else {
                key.entry(*x).and_modify(|c| *c -= 1);
                key1.entry(aim - x).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    //[1,1,1,1,2,2,2,2] 2
    let a = vec![1, 1, 1, 1, 2, 2, 2, 2];
    println!("res:{}", Solution::can_partition_k_subsets(a, 2));
}
