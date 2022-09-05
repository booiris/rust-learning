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

fn check(key: &Vec<Vec<i32>>, k: i32, budget: i64) -> bool {
    if k == 0 {
        return true;
    }
    let mut sum = 0;
    for i in 0..k as usize {
        sum += key[i][1] as i64;
        maxn = max(maxn, key[i][0]);
    }
    let mut minn = sum * k as i64 + maxn as i64;
    for i in k..key.len() {
        sum
    }

    minn <= budget
}

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut key = charge_times
            .iter()
            .zip(running_costs.iter())
            .map(|(&x, &y)| vec![x, y])
            .collect::<Vec<_>>();
        let (mut low, mut high) = (0, key.len() as i32);
        while low <= high {
            let mid = (low + high) / 2;
            if check(&key, mid, budget) {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        high
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![11, 12, 74, 67, 37, 87, 42, 34, 18, 90, 36, 28, 34, 20];
    let b = vec![18, 98, 2, 84, 7, 57, 54, 65, 59, 91, 7, 23, 94, 20];
    println!("res:{}", Solution::maximum_robots(a, b, 937));
}
