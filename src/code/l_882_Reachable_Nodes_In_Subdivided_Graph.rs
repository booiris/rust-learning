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

fn dfs(
    res: &mut i32,
    p: &Vec<Vec<usize>>,
    e: &mut Vec<(usize, usize, i32, i32, i32)>,
    vis: &mut Vec<bool>,
    now: usize,
    max_moves: i32,
) {
    *res += 1;
    for x in &p[now] {
        let edge = e.get_mut(*x).unwrap();
        let to;
        if now == edge.0 {
            to = edge.1;
        } else {
            to = edge.0;
        }
        if now == edge.0 {
            (*edge).3 = max(edge.3, min(edge.2, max_moves));
        } else {
            (*edge).4 = max(edge.4, min(edge.2, max_moves));
        }
        if vis[to] || max_moves < edge.2 {
            continue;
        }
        vis[to] = true;
        let temp = edge.2;
        dfs(res, p, e, vis, now, max_moves - temp);
    }
}

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let mut p: Vec<Vec<usize>> = vec![vec![]; n as usize];
        let mut e: Vec<(usize, usize, i32, i32, i32)> = vec![];
        for x in edges {
            e.push((x[0] as usize, x[1] as usize, x[2], 0, 0));
            p[x[0] as usize].push(e.len() - 1);
            p[x[1] as usize].push(e.len() - 1);
        }
        let mut vis = vec![false; n as usize];
        let mut res = 0;
        vis[0] = true;
        dfs(&mut res, &p, &mut e, &mut vis, 0, max_moves);
        println!("{}", res);
        println!("{:?}", e);
        for x in e {
            res += min(x.2, x.3 + x.4);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = [[0, 1, 10], [0, 2, 1], [1, 2, 2]];
    let a = a.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
    println!("res:{}", Solution::reachable_nodes(a, 6, 3));
}
