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
    pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut key = students.into_iter().collect::<VecDeque<_>>();
        sandwiches.reverse();
        let mut cnt = 0;
        while !key.is_empty() {
            if key.front() == sandwiches.last() {
                sandwiches.pop();
                key.pop_front();
                cnt = 0;
            } else {
                let x = key.pop_front().unwrap();
                key.push_back(x);
                cnt += 1;
            }
            if cnt == key.len() {
                return cnt as i32;
            }
        }
        0
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
