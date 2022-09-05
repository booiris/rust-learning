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
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut key = vec![];
        let mut res = vec![0; prices.len()];
        for i in (0..prices.len()).rev() {
            let mut temp = 0;
            while !key.is_empty() {
                if prices[i] >= *key.last().unwrap() {
                    temp = *key.last().unwrap();
                    if prices[i] == *key.last().unwrap() {
                        key.pop();
                    }
                    break;
                }
                key.pop();
            }
            res[i] = prices[i] - temp;
            key.push(prices[i]);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
