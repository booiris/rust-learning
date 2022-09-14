#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::mem::swap;
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
    pub fn maximum_swap(num: i32) -> i32 {
        let mut res = num;
        let mut n = vec![];
        let mut temp = num;
        while temp != 0 {
            n.push(temp % 10);
            temp /= 10;
        }
        n.reverse();
        let mut key = [1; 10];
        for i in 1..10 {
            key[i] = key[i - 1] * 10;
        }
        let len = n.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }
                n.swap(i, j);
                let mut temp = 0;
                for x in &n {
                    temp *= 10;
                    temp += x;
                }
                res = max(res, temp);
                n.swap(i, j);
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
