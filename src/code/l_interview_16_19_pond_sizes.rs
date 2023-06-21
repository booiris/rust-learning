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

const DIR: [[i32; 2]; 8] = [
    [0, 1],
    [1, 0],
    [-1, 0],
    [0, -1],
    [1, 1],
    [1, -1],
    [-1, -1],
    [-1, 1],
];

fn bfs(
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    land: &Vec<Vec<i32>>,
    vis: &mut Vec<Vec<bool>>,
) -> i32 {
    let n = n as i32;
    let m = m as i32;
    let mut q = VecDeque::new();
    let mut res = 0;
    q.push_back((x as i32, y as i32));
    vis[x][y] = true;
    while let Some(now) = q.pop_front() {
        res += 1;
        for d in DIR {
            let nx = now.0 + d[0];
            let ny = now.1 + d[1];
            if nx < n
                && nx >= 0
                && ny < m
                && ny >= 0
                && !vis[nx as usize][ny as usize]
                && land[nx as usize][ny as usize] == 0
            {
                vis[nx as usize][ny as usize] = true;
                q.push_back((nx, ny));
            }
        }
    }
    res
}

impl Solution {
    pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (land.len(), land[0].len());
        let mut vis = vec![vec![false; m]; n];
        let mut res = vec![];
        for i in 0..n {
            for j in 0..m {
                if land[i][j] == 0 && !vis[i][j] {
                    res.push(bfs(i, j, n, m, &land, &mut vis));
                }
            }
        }
        res.sort_unstable();
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let ini = [[0, 2, 1, 0], [0, 1, 0, 1], [1, 1, 0, 1], [0, 1, 0, 1]];
    let ini: Vec<Vec<i32>> = ini
        .iter()
        .map(|x| x.into_iter().map(|&x| x).collect())
        .collect();
    println!("res:{:?}", Solution::pond_sizes(ini));
}
