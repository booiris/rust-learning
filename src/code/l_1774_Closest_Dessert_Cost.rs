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

fn dfs(
    now: i32,
    depth: usize,
    topping_costs: &Vec<i32>,
    traget: i32,
    res: &mut i32,
    diff: &mut i32,
) {
    let key = (now - traget).abs();
    if key < *diff {
        *diff = key;
        *res = now;
    } else if key == *diff && *res > now {
        *res = now;
    }
    if now - traget > *diff {
        return;
    }
    if depth >= topping_costs.len() {
        return;
    }
    dfs(now, depth + 1, topping_costs, traget, res, diff);
    dfs(
        now + topping_costs[depth],
        depth + 1,
        topping_costs,
        traget,
        res,
        diff,
    );
    dfs(
        now + topping_costs[depth] * 2,
        depth + 1,
        topping_costs,
        traget,
        res,
        diff,
    );
}

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut res = i32::MAX;
        let mut diff = i32::MAX;
        for x in base_costs {
            dfs(x, 0, &topping_costs, target, &mut res, &mut diff);
        }
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![3, 10];
    let b = vec![2, 5];
    println!("res:{}", Solution::closest_cost(a, b, 9));
}
