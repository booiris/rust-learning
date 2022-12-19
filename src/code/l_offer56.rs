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
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let k1 = nums.iter().fold(0, |mut now, &x| {
            now ^= x;
            now
        });
        let mut kk = 1;
        while (kk & k1) == 0{
            kk <<= 1;
        }
        nums.into_iter().fold(vec![0_i32,0_i32], |mut now,x|{
            if x & kk == 0{
                now[1] ^= x;
            }else{
                now[0] ^=x;
            }
            return now
        })
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
