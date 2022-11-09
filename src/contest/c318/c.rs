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
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut key = BinaryHeap::new();
        let k = k as usize;
        let candidates = candidates as usize;
        let len = costs.len();
        let (mut i, mut j) = (0, len - 1);

        while i < candidates {
            key.push(Reverse((costs[i], i)));
            i += 1;
        }
        while i <= j && len - j - 1 < candidates {
            key.push(Reverse((costs[j], j)));
            j -= 1;
        }
        let mut res = 0;
        for _ in 0..k {
            let top = key.pop().unwrap();
            // println!("{:?} {} {} {}", top, res, i, j);
            // println!("{:?}", key);
            res += top.0 .0 as i64;
            let index = top.0 .1;
            if index < i && i <= j {
                key.push(Reverse((costs[i], i)));
                i += 1;
            } else if index > j && i <= j {
                key.push(Reverse((costs[j], j)));
                j -= 1;
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![1, 2, 4, 1];
    println!("res:{}", Solution::total_cost(a, 3, 3));
}
