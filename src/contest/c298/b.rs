#![allow(unused_imports)]
#![warn(unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let mut key = [false; 10];
        let mut flag = false;
        let mut now = k;
        let mut res = 1;
        while now <= num {
            if now % 10 == num % 10 {
                flag = true;
                break;
            }
            if key[(now % 10) as usize] {
                break;
            }
            key[(now % 10) as usize] = true;
            now += k;
            res += 1;
        }
        if flag {
            return res;
        } else {
            return -1;
        }
    }
}
#[cfg(test)]
pub fn main() {
    println!("res:{}",Solution::minimum_numbers(37, 2));
}
