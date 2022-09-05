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

fn solve(now: char, garbage: &Vec<String>, travel: &Vec<i32>) -> i32 {
    let mut res = travel.iter().sum();
    let mut index = travel.len();
    for (i, x) in garbage.iter().enumerate() {
        let mut t = false;
        for y in x.chars() {
            if y == now {
                t = true;
                res += 1;
            }
        }
        if t {
            index = travel.len();
        } else {
            index = min(index, i - 1);
        }
    }
    index = max(index, 0);
    for i in index..travel.len() {
        res -= travel[i];
    }
    res
}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        solve('M', &garbage, &travel)
            + solve('P', &garbage, &travel)
            + solve('G', &garbage, &travel)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
