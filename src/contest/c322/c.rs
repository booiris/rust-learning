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

fn dfs(now: usize, p: &Vec<Vec<(usize, i32)>>, res: &mut i32, vis: &mut Vec<bool>) {
    vis[now] = true;
    for x in &p[now] {
        *res = x.1.min(*res);
        if vis[x.0] {
            continue;
        }
        dfs(x.0, p, res, vis);
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut p = vec![vec![]; n + 1];
        let mut vis = vec![false; n + 1];
        for x in roads {
            p[x[0] as usize].push((x[1] as usize, x[2]));
            p[x[1] as usize].push((x[0] as usize, x[2]));
        }
        let mut res = i32::MAX;
        dfs(1, &p, &mut res, &mut vis);
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
