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
    pub fn construct_array(n: i32, mut k: i32) -> Vec<i32> {
        let mut vis = vec![false; n as usize + 1];
        let mut index = 0;
        let mut now = 1;
        let mut res = vec![0; n as usize];
        while now <= n {
            res[index] = now;
            vis[now as usize] = true;
            index += 1;
            if index >= n as usize {
                break;
            }
            res[index] = now + k;
            if !vis[(now + k) as usize] {
                vis[(now + k) as usize] = true;
                k -= 2;
                index += 1;
            }
            while now <= n && vis[now as usize] {
                now += 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:{:?}", Solution::construct_array(5, 2));
}
