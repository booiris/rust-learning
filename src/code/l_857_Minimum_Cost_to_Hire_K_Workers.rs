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
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut key = quality
            .iter()
            .zip(wage.iter())
            .map(|(&x, &y)| vec![x, y])
            .collect::<Vec<_>>();
        key.sort_unstable_by(|x, y| (x[1] * y[0]).cmp(&(x[0] * y[1])));
        let mut q = BinaryHeap::new();
        let mut sum = 0;
        for i in 0..(k - 1) as usize {
            q.push(key[i][0]);
            sum += key[i][0];
        }
        let mut res: f64 = 1000000000.0;
        for i in (k - 1) as usize..key.len() {
            let temp = sum + key[i][0];
            res = res.min(temp as f64 / key[i][0] as f64 * key[i][1] as f64);
            if let Some(x) = q.peek() {
                if key[i][0] < *x {
                    sum -= *x;
                    q.pop();
                    q.push(key[i][0]);
                    sum += key[i][0];
                }
            }
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![3, 1, 10, 10, 1];
    let b = vec![4, 8, 2, 2, 7];
    println!("res:{}", Solution::mincost_to_hire_workers(a, b, 3));
}
