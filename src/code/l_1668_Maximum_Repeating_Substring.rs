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
//     pub fn max_repeating(sequence: String, word: String) -> i32 {
//         let mut res = 0;
//         for i in 0..sequence.len() {
//             let mut now = i;
//             let mut cnt = 0;
//             while now + word.len() <= sequence.len() {
//                 if &sequence[now..now + word.len()] == word {
//                     cnt += 1;
//                     now += word.len();
//                 } else {
//                     break;
//                 }
//             }
//             res = res.max(cnt);
//         }
//         res
//     }
// }

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut res = 0;
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
