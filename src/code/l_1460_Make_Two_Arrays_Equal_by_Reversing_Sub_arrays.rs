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
//     pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
//         let mut key = [0; 1005];
//         arr.iter().for_each(|x| key[*x as usize] += 1);
//         for x in target {
//             if key[x as usize] == 0 {
//                 return false;
//             }
//             key[x as usize] -= 1;
//         }
//         true
//     }
// }

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        return target == arr;
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
