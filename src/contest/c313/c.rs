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
    pub fn minimize_xor(mut num1: i32, mut num2: i32) -> i32 {
        let mut key = 0;
        while num2 != 0 {
            if num2 & 1 == 1 {
                key += 1;
            }
            num2 >>= 1;
        }
        let mut k = vec![];
        while num1 != 0 {
            k.push(num1 & 1);
            num1 >>= 1;
        }
        k.reverse();
        let mut res = 0;
        for x in k.iter_mut() {
            res <<= 1;
            if *x == 1 && key > 0 {
                key -= 1;
            } else {
                *x = 0;
            }
        }
        for i in (0..k.len()).rev() {
            if key > 0 && k[i] == 0 {
                key -= 1;
                k[i] = 1;
            }
        }
        for _ in 0..key {
            k.push(1);
        }
        for x in k {
            res <<= 1;
            res += x;
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{}", Solution::minimize_xor(1, 12));
}
