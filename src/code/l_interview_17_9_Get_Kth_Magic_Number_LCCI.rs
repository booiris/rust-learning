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
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let mut key = BinaryHeap::new();
        let mut s = HashSet::new();
        s.insert(1 as u64);
        key.push(Reverse(1 as u64));
        for _ in 1..k {
            let now = key.pop().unwrap().0;
            if s.get(&(now * 3)).is_none() {
                key.push(Reverse(now * 3));
                s.insert(now * 3);
            }
            if s.get(&(now * 5)).is_none() {
                key.push(Reverse(now * 5));
                s.insert(now * 5);
            }
            if s.get(&(now * 7)).is_none() {
                key.push(Reverse(now * 7));
                s.insert(now * 7);
            }
        }
        (*key.peek().unwrap()).0 as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
