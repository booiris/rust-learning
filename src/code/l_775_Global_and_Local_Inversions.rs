#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
#[cfg(feature = "local")]
struct Solution;

pub struct Bitree {
    tree: Vec<i32>,
    len: usize,
}
impl Bitree {
    #[inline]
    pub fn new(ini: &Vec<i32>) -> Self {
        let mut res = Bitree {
            tree: vec![0; ini.len() + 1],
            len: ini.len(),
        };
        for (index, v) in ini.iter().enumerate() {
            res.update(index + 1, *v);
        }
        res
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

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut cnt1 = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                cnt1 += 1;
            }
        }
        let mut cnt2 = 0;
        let mut key = Bitree::new(&vec![0; nums.len() + 1]);
        for i in (0..nums.len()).rev() {
            key.update((nums[i] as usize) + 1, 1);
            cnt2 += key.query(1, nums[i] as usize);
        }
        // println!("{} {}", cnt1, cnt2);
        cnt1 == cnt2
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 2, 0];
    println!("res:{}", Solution::is_ideal_permutation(a));
}
