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

pub struct DisjointSet {
    father: Vec<usize>,
}

impl DisjointSet {
    #[inline]
    pub fn new(len: usize) -> Self {
        DisjointSet {
            father: (0..len).into_iter().map(|x| x).collect::<Vec<_>>(),
        }
    }

    #[inline]
    pub fn find(&mut self, x: usize) -> usize {
        if self.father[x] != x {
            self.father[x] = self.find(self.father[x]);
        }
        self.father[x]
    }

    #[inline]
    /// 将 y 合并到 x
    pub fn union(&mut self, x: usize, y: usize) {
        let x_father = self.find(x);
        self.father[x_father] = self.find(y);
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut p = vec![vec![]; vals.len()];
        for x in edges {
            p[x[0] as usize].push(x[1]);
            p[x[1] as usize].push(x[0]);
        }
        let mut res = 0;
        let mut order = vals
            .iter()
            .enumerate()
            .map(|(index, &x)| (index, x))
            .collect::<Vec<_>>();
        order.sort_unstable_by_key(|x| x.1);
        let mut key = DisjointSet::new(vals.len());
        let mut cnt = vec![1; vals.len()];
        for x in order {
            let x = x.0;
            let fa = key.find(x);
            for &nxt in &p[x] {
                let nxt_f = key.find(nxt as usize);
                if nxt_f == fa || vals[nxt_f] > vals[x] {
                    continue;
                }
                if vals[nxt_f] == vals[x] {
                    res += (cnt[fa] * cnt[nxt_f]) as i32;
                    cnt[fa] += cnt[nxt_f];
                }
                key.union(nxt as usize, x);
            }
        }
        res + vals.len() as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let _a = vec![2, 4, 1, 2, 2, 5, 3, 4, 4];
    println!("res:");
}

// [2,4,1,2,2,5,3,4,4]
// [[0,1],[2,1],[0,3],[4,1],[4,5],[3,6],[7,5],[2,8]]
