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

fn checkin(p1: (i32, i32), p2: (i32, i32), r: i32) -> bool {
    r * r >= dis(p1, p2)
}

fn dis(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1)
}

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let towers = towers
            .into_iter()
            .map(|x| (x[0], x[1], x[2]))
            .collect::<Vec<_>>();
        let mut res = [0, 0];
        let mut maxn: i32 = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                let mut now: i32 = 0;
                for x in &towers {
                    if checkin((x.0, x.1), (i, j), radius) {
                        let dis = dis((x.0, x.1), (i, j)) as f64;
                        now += (x.2 as f64 / (1.0 + dis.sqrt())).floor() as i32;
                    }
                }
                if now > maxn {
                    maxn = now;
                    res = [i, j];
                }
            }
        }
        Vec::from(res)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]];
    println!("res:{:?}", Solution::best_coordinate(a, 2));
}
