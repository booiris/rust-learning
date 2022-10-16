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

fn dfs(now: usize, color: &mut Vec<i32>, now_c: i32, p: &Vec<Vec<usize>>) -> bool {
    color[now] = now_c;
    for &x in &p[now] {
        if color[x] == now_c {
            return false;
        }
        if color[x] == 0 && !dfs(x, color, -now_c, p) {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut p = vec![vec![]; n as usize];
        let mut color = vec![0; n as usize];
        for x in dislikes {
            p[x[0] as usize - 1].push(x[1] as usize - 1);
            p[x[1] as usize - 1].push(x[0] as usize - 1);
        }
        for x in 0..n as usize {
            if color[x] == 0 && !dfs(x, &mut color, 1, &p) {
                return false;
            }
        }
        true
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let b = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5]];
    println!("res:{}", Solution::possible_bipartition(5, b));
}
