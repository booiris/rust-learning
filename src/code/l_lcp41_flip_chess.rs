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

fn check(
    x: usize,
    y: usize,
    sx: i32,
    sy: i32,
    n: usize,
    m: usize,
    chessboard: &Vec<Vec<char>>,
    key: &mut Vec<Vec<Vec<(usize, usize)>>>,
) {
    let n = n as i32;
    let m = m as i32;
    let mut flag = 0;
    let mut pos = (-1, -1);
    let mut pos1 = vec![];
    let mut pos2 = vec![];

    let mut nowx = x as i32 + sx;
    let mut nowy = y as i32 + sy;
    while nowx < n && nowy < m && nowx >= 0 && nowy >= 0 {
        let temp = chessboard[nowx as usize][nowy as usize];
        if temp == '.' {
            pos = (nowx, nowy);
            break;
        } else if temp == 'X' {
            flag += 1;
            break;
        } else {
            pos1.push((nowx as usize, nowy as usize));
        }
        nowx += sx;
        nowy += sy;
    }

    let mut nowx = x as i32 - sx;
    let mut nowy = y as i32 - sy;
    while nowx < n && nowy < m && nowx >= 0 && nowy >= 0 {
        let temp = chessboard[nowx as usize][nowy as usize];
        if temp == '.' {
            pos = (nowx, nowy);
            break;
        } else if temp == 'X' {
            flag += 2;
            break;
        } else {
            pos2.push((nowx as usize, nowy as usize));
        }
        nowx -= sx;
        nowy -= sy;
    }

    if (flag == 1 || flag == 2) && pos.0 >= 0 {
        key[pos.0 as usize][pos.1 as usize].push((x, y));
    }
    if flag == 1 {
        for d in pos2 {
            key[d.0][d.1].push((x, y));
        }
    }
    if flag == 2 {
        for d in pos1 {
            key[d.0][d.1].push((x, y));
        }
    }
}

fn begin(x: usize, y: usize, n: usize, m: usize, key: &Vec<Vec<Vec<(usize, usize)>>>) -> i32 {
    let mut res = 0;
    let mut vis = vec![vec![false; m]; n];
    let mut q = VecDeque::new();
    vis[x][y] = true;
    for x in &key[x][y] {
        if !vis[x.0][x.1] {
            q.push_back(x);
            vis[x.0][x.1] = true;
        }
    }
    while let Some(now) = q.pop_front() {
        res += 1;
        for x in &key[now.0][now.1] {
            if !vis[x.0][x.1] {
                q.push_back(x);
                vis[x.0][x.1] = true;
            }
        }
    }

    return res;
}

impl Solution {
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        let (n, m) = (chessboard.len(), chessboard[0].len());
        let chessboard: Vec<Vec<char>> = chessboard
            .into_iter()
            .map(|x| x.chars().map(|x| x).collect())
            .collect();
        let mut key: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; m]; n];
        for i in 0..n {
            for j in 0..m {
                if chessboard[i][j] == 'O' {
                    check(i, j, 1, 0, n, m, &chessboard, &mut key);
                    check(i, j, 1, 1, n, m, &chessboard, &mut key);
                    check(i, j, 0, 1, n, m, &chessboard, &mut key);
                    check(i, j, 1, -1, n, m, &chessboard, &mut key);
                }
            }
        }

        // println!("{:?}", key);

        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if chessboard[i][j] == '.' {
                    res = res.max(begin(i, j, n, m, &key));
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let chessboard = [
        ".O.....", ".O.....", "XOO.OOX", ".OO.OO.", ".XO.OX.", "..X.X..",
    ];
    let chessboard: Vec<String> = chessboard.iter().map(|x| x.to_string()).collect();

    println!("res:{}", Solution::flip_chess(chessboard));
}
