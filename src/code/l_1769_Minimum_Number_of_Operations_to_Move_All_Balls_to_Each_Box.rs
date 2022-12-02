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
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut k1 = vec![0; boxes.len()];
        let mut k2 = vec![0; boxes.len()];
        let mut res = vec![0; boxes.len()];
        let mut now = 0;
        let mut cnt = 0;
        let boxes = boxes.chars().into_iter().collect::<Vec<_>>();
        for (i, c) in boxes.iter().enumerate() {
            now += cnt;
            k1[i] = now;
            if *c == '1' {
                cnt += 1;
            }
        }
        now = 0;
        cnt = 0;
        for i in (0..boxes.len()).rev() {
            now += cnt;
            k2[i] = now;
            if boxes[i] == '1' {
                cnt += 1;
            }
        }
        for i in 0..boxes.len() {
            res[i] = k1[i] + k2[i];
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = "001011";
    let a = a.to_owned();
    println!("res:{:?}", Solution::min_operations(a));
}
