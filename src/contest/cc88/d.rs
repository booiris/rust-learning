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

pub struct Bitree {
    tree: Vec<i32>,
    len: usize,
}
impl Bitree {
    #[inline]
    pub fn new(len: usize) -> Self {
        Bitree {
            tree: vec![0; len + 1],
            len,
        }
    }

    #[inline]
    fn lowbit(x: usize) -> usize {
        return (x as i32 & -(x as i32)) as usize;
    }

    fn update(&mut self, index: usize, v: i32) {
        let mut pos = index;
        while pos <= self.len {
            self.tree[pos] += v;
            pos += Self::lowbit(pos);
        }
    }

    fn get(&self, n: usize) -> i32 {
        let mut res = 0;
        let mut pos = n;
        while pos > 0 {
            res += self.tree[pos];
            pos -= Self::lowbit(pos);
        }
        res
    }

    /// `query` 下标从`1`开始 查询范围 [l,r]
    pub fn query(&self, l: usize, r: usize) -> i32 {
        self.get(r) - self.get(l - 1)
    }
}

impl Solution {
    pub fn number_of_pairs(mut nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let maxn = 4e4 as usize + 10;

        let mut key = Bitree::new(1e5 as usize);
        nums1
            .iter_mut()
            .zip(nums2.iter())
            .for_each(|(x, y)| *x -= y);
        let mut res = 0;
        for x in nums1 {
            res += key.query(1, (x + maxn as i32 + diff) as usize) as i64;
            key.update(x as usize + maxn, 1);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
