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

// impl Solution {
//     pub fn missing_two(mut nums: Vec<i32>) -> Vec<i32> {
//         nums.push(-1);
//         nums.push(-1);
//         let mut res = vec![];
//         for i in 0..nums.len() {
//             while nums[i] != -1 && nums[i] - 1 != i as i32 {
//                 let temp = nums[i] as usize - 1;
//                 nums.swap(i, temp);
//             }
//         }
//         for i in 0..nums.len() {
//             if nums[i] == -1 {
//                 res.push(i as i32 + 1);
//             }
//         }
//         res
//     }
// }

// impl Solution {
//     pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
//         let sum = nums.iter().sum::<i32>();
//         let n = nums.len() as i32 + 2;
//         let sum_two = n * (n + 1) / 2 - sum;
//         let key = sum_two / 2;
//         let sum_low = nums.iter().filter(|&&x| x <= key).sum::<i32>();
//         let one = key * (key + 1) / 2 - sum_low;
//         // println!("{} {} {}", sum_two, sum_low, sum);
//         vec![one, sum_two - one]
//     }
// }

impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let nums = &nums;
        let n = nums.len() as i32 + 2;
        let mut key = 0;
        (1..=n).into_iter().for_each(|x| key ^= x);
        nums.into_iter().for_each(|x| key ^= x);
        let diff = key & -key;
        let mut one = 0;
        (1..=n).into_iter().for_each(|x| {
            if x & diff != 0 {
                one ^= x
            }
        });
        nums.into_iter().for_each(|x| {
            if x & diff != 0 {
                one ^= x
            }
        });
        vec![one, key ^ one]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{:?}", Solution::missing_two(vec![1]));
}
