#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
struct Solution;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn dfs(
    now: i32,
    nowcolor: &i32,
    key: &Vec<Vec<i32>>,
    vis: &mut Vec<bool>,
    colorcnt: &mut Vec<i32>,
) {
    for nxt in key[now as usize].iter() {
        if !vis[*nxt as usize] {
            vis[*nxt as usize] = true;
            colorcnt[*nowcolor as usize] += 1;
            dfs(*nxt, nowcolor, key, vis, colorcnt);
        }
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut vis = vec![false; n as usize];
        let mut nowcolor = 1;
        let mut key: Vec<Vec<i32>> = vec![vec![]; n as usize];
        let mut colorcnt = vec![0; (n + 1) as usize];
        for edge in edges.iter() {
            key[edge[0] as usize].push(edge[1]);
            key[edge[1] as usize].push(edge[0]);
        }
        for now in 0..n {
            if vis[now as usize] {
                continue;
            }
            vis[now as usize] = true;
            colorcnt[nowcolor as usize] += 1;
            dfs(now, &nowcolor, &key, &mut vis, &mut colorcnt);
            nowcolor += 1;
        }
        let mut res: i64 = 0;
        for i in 1..nowcolor {
            res += (n - colorcnt[i as usize]) as i64 * colorcnt[i as usize] as i64;
        }
        res / 2
    }
}

#[cfg(test)]
pub fn main() {
    let n = 7;
    let edges: Vec<Vec<i32>> = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
    println!("res:{:?}", Solution::count_pairs(n, edges));
}
