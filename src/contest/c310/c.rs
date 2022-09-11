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
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        let mut key = BinaryHeap::<Reverse<i32>>::new();
        for x in intervals {
            if let Some(y) = key.peek() {
                if y.0 < x[0] {
                    key.pop();
                }
            }
            key.push(Reverse(x[1]));
        }
        key.len() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
    println!("res:{}", Solution::min_groups(a));
}
