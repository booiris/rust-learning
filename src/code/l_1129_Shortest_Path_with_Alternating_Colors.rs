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

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut p = HashMap::new();
        for x in red_edges {
            p.entry(x[0] as usize)
                .or_insert(vec![])
                .push((x[1] as usize, -1));
        }
        for x in blue_edges {
            p.entry(x[0] as usize)
                .or_insert(vec![])
                .push((x[1] as usize, 1));
        }
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        let mut res = vec![-1; n as usize];
        let mut vis = vec![(false, false); n as usize];
        res[0] = 0;
        while let Some((now, want, d)) = q.pop_front() {
            if res[now] == -1 {
                res[now] = d;
            }
            res[now] = d.min(res[now]);
            if let Some(iter) = p.get(&now) {
                for &(nxt, flag) in iter {
                    if want == flag || want == 0 {
                        if flag == -1 {
                            if vis[nxt].0 {
                                continue;
                            } else {
                                vis[nxt].0 = true;
                            }
                        }
                        if flag == 1 {
                            if vis[nxt].1 {
                                continue;
                            } else {
                                vis[nxt].1 = true;
                            }
                        }
                        q.push_back((nxt, -flag, d + 1));
                    }
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
