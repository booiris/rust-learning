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

fn cmp(a: &String, b: &String) -> bool {
    let a = a.clone();
    let b = b.clone();
    let a = a.split(':').collect::<Vec<_>>();
    let b = b.split(':').collect::<Vec<_>>();
    if a[0].parse::<i32>().unwrap() > b[0].parse::<i32>().unwrap() {
        return true;
    } else if a[0].parse::<i32>().unwrap() == b[0].parse::<i32>().unwrap()
        && a[1].parse::<i32>().unwrap() >= b[1].parse::<i32>().unwrap()
    {
        return true;
    } else {
        return false;
    }
}

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        if cmp(&event1[1], &event2[0]) && cmp(&event2[1], &event1[0]) {
            return true;
        }
        if cmp(&event2[1], &event1[0]) && cmp(&event1[1], &event2[0]) {
            return true;
        }
        false
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
