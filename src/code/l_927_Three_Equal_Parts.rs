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
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let cnt = arr.iter().filter(|&&x| x == 1).count();
        if cnt == 0 {
            return vec![0, 2];
        }
        if cnt % 3 != 0 {
            return vec![-1, -1];
        }
        let key = cnt / 3;
        let (mut i, mut j, mut k): (usize, usize, usize) = (0, 0, 0);
        let mut now = 0;
        for (index, x) in arr.iter().enumerate() {
            if *x == 1 {
                if now % key == 0 {
                    match now / key {
                        0 => i = index,
                        1 => j = index,
                        2 => k = index,
                        _ => {}
                    }
                }
                now += 1;
            }
        }
        while k < arr.len() {
            if arr[i] != arr[j] || arr[j] != arr[k] || arr[i] != arr[k] {
                return vec![-1, -1];
            }
            i += 1;
            j += 1;
            k += 1;
        }
        vec![(i - 1) as i32, j as i32]
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 1, 0, 0, 1];
    println!("res:{:?}", Solution::three_equal_parts(a));
}
