#![allow(unused_imports)]
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

fn dfs(now: i32, nowcolor: i32, key: Vec<Vec<i32>>, mut color: Vec<i32>) {
    for nxt in key[now as usize].iter() {
        if color[*nxt as usize] == 0 {
            color[*nxt as usize] = nowcolor;
            dfs(*nxt, nowcolor, key, color);
        }
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut color = vec![0; n as usize];
        let mut nowcolor = 1;
        let mut key: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for edge in edges.iter() {
            key[edge[0] as usize].push(edge[1]);
            key[edge[1] as usize].push(edge[0]);
        }
        for now in 0..n {
            if color[now as usize] != 0 {
                continue;
            }
            color[now as usize] = nowcolor;
            dfs(now, nowcolor, key, color);
            nowcolor += 1;
        }
        let mut res: i64 = 0;
        for i in 1..nowcolor {
            res += (n - color[i as usize]) as i64 * color[i as usize] as i64;
        }
        res
    }
}

#[cfg(test)]
pub fn main() {
    let n = 7;
    let edges: Vec<Vec<i32>> = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
    println!("res:{:?}", Solution::count_pairs(n, edges));
}
