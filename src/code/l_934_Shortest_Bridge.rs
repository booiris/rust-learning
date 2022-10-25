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

static DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs(
    color: &mut Vec<Vec<i32>>,
    nowx: usize,
    nowy: usize,
    grid: &Vec<Vec<i32>>,
    key: &mut VecDeque<(usize, usize)>,
) {
    color[nowx][nowy] = 1;
    let n = color.len() as i32;
    for x in DIR {
        let nx = nowx as i32 + x.0;
        let ny = nowy as i32 + x.1;
        if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if color[nx][ny] != 0 {
            continue;
        }
        if grid[nx][ny] == 0 && color[nx][ny] == 0 {
            color[nx][ny] = 2;
            key.push_back((nx, ny));
            continue;
        }
        color[nx][ny] = 1;
        dfs(color, nx, ny, grid, key);
    }
}

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut color = vec![vec![0; grid.len()]; grid.len()];
        let mut key = VecDeque::<(usize, usize)>::new();
        let mut flag = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    dfs(&mut color, i, j, &grid, &mut key);
                    flag = true;
                    break;
                }
            }
            if flag {
                break;
            }
        }

        let n = color.len() as i32;
        while !key.is_empty() {
            let len = key.len();
            res += 1;
            for _ in 0..len {
                let (x, y) = key.pop_front().unwrap();
                color[x][y] = 2;
                for dir in DIR {
                    let nx = x as i32 + dir.0;
                    let ny = y as i32 + dir.1;
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 1 && color[nx][ny] != 1 {
                        return res;
                    }
                    if color[nx][ny] == 2 || grid[nx][ny] == 1 {
                        continue;
                    }
                    color[nx][ny] = 2;
                    key.push_back((nx, ny));
                }
            }
        }

        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
    println!("res:{}", Solution::shortest_bridge(a));
}
