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

fn bfs(now_c: i32, color: &mut Vec<i32>, p: &Vec<Vec<usize>>, head: usize, n: usize) -> i32 {
    let mut vis = vec![false; n + 1];
    let mut q = VecDeque::new();
    q.push_front((head, 1));
    vis[head] = true;
    let mut res = 0;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            let t = q.pop_front().unwrap();
            color[t.0] = now_c;
            res = res.max(t.1);
            for &x in &p[t.0] {
                if vis[x] {
                    continue;
                }
                vis[x] = true;
                q.push_back((x, t.1 + 1));
            }
        }
    }
    res
}

fn check(now_c: i32, color: &mut Vec<i32>, p: &Vec<Vec<usize>>, now: usize, father: usize) -> bool {
    for &x in &p[now] {
        if x == father {
            continue;
        }
        if color[x] == 0 {
            color[x] = -now_c;
            if !check(-now_c, color, p, x, now) {
                return false;
            }
        } else if color[x] == now_c {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut p = vec![vec![]; n + 1];
        for x in edges {
            p[x[0] as usize].push(x[1] as usize);
            p[x[1] as usize].push(x[0] as usize);
        }

        let mut color = vec![0; n + 1];
        for x in 1..=n {
            if color[x] != 0 {
                continue;
            }
            color[x] = 1;
            if !check(1, &mut color, &p, x, 0) {
                return -1;
            }
        }

        let mut key = HashMap::<i32, i32>::new();
        let mut color = vec![0; n + 1];
        let mut now_c = 1;
        for x in 1..=n {
            let c;
            if color[x] != 0 {
                c = color[x];
            } else {
                c = now_c;
                now_c += 1;
            }
            let v = key.entry(c).or_default();
            *v = bfs(c, &mut color, &p, x, n).max(*v);
        }
        key.into_iter().map(|x| x.1).sum()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let e = [
        [67, 29],
        [13, 29],
        [77, 29],
        [36, 29],
        [82, 29],
        [54, 29],
        [57, 29],
        [53, 29],
        [68, 29],
        [26, 29],
        [21, 29],
        [46, 29],
        [41, 29],
        [45, 29],
        [56, 29],
        [88, 29],
        [2, 29],
        [7, 29],
        [5, 29],
        [16, 29],
        [37, 29],
        [50, 29],
        [79, 29],
        [91, 29],
        [48, 29],
        [87, 29],
        [25, 29],
        [80, 29],
        [71, 29],
        [9, 29],
        [78, 29],
        [33, 29],
        [4, 29],
        [44, 29],
        [72, 29],
        [65, 29],
        [61, 29],
    ];
    let e = e.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
    println!("res:{}", Solution::magnificent_sets(92, e));
}
