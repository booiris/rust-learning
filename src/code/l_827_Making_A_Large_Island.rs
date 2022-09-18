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

const DIR: [[i32; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

fn dfs(
    color: &mut Vec<Vec<i32>>,
    vis: &mut Vec<Vec<bool>>,
    cnt: &mut HashMap<i32, i32>,
    grid: &Vec<Vec<i32>>,
    p: (usize, usize),
    nowc: i32,
) {
    color[p.0][p.1] = nowc;
    vis[p.0][p.1] = true;
    cnt.entry(nowc).and_modify(|c| *c += 1).or_insert(1);
    let n = grid.len() as i32;
    for x in DIR {
        let nx = p.0 as i32 + x[0];
        let ny = p.1 as i32 + x[1];
        if nx >= 0
            && nx < n
            && ny >= 0
            && ny < n
            && !vis[nx as usize][ny as usize]
            && grid[nx as usize][ny as usize] == 1
        {
            dfs(color, vis, cnt, grid, (nx as usize, ny as usize), nowc);
        }
    }
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = HashMap::<i32, i32>::new();
        cnt.insert(0, 0);
        let mut res = 0;
        let mut color = vec![vec![0; grid[0].len()]; grid.len()];
        let mut vis = vec![vec![false; grid[0].len()]; grid.len()];
        let mut now = 1;
        let n = grid.len() as i32;
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == 1 && !vis[i][j] {
                    dfs(&mut color, &mut vis, &mut cnt, &grid, (i, j), now);
                    now += 1;
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                let mut temp = HashSet::new();
                for d in DIR {
                    let nx = i as i32 + d[0];
                    let ny = j as i32 + d[1];
                    if nx >= 0 && nx < n && ny >= 0 && ny < n {
                        temp.insert(color[nx as usize][ny as usize]);
                    }
                }
                let mut sum = 0;
                if grid[i][j] == 0 {
                    sum = 1;
                }
                for x in &temp {
                    sum += cnt.get(x).unwrap();
                }
                res = max(res, sum);
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![vec![0, 0], vec![1, 0]];
    println!("res:{}", Solution::largest_island(a));
}
