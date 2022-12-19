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
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut key = VecDeque::new();
        let mut p = vec![vec![]; n as usize];
        for y in edges {
            p[y[0] as usize].push(y[1] as usize);
            p[y[1] as usize].push(y[0] as usize);
        }
        let mut visited = vec![false; n as usize];
        key.push_front(source as usize);
        visited[source as usize] = true;
        while let Some(front) = key.pop_front() {
            if front == destination as usize {
                return true;
            }
            for &x in &p[front] {
                if visited[x] == false {
                    visited[x] = true;
                    key.push_back(x);
                }
            }
        }
        false
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
