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
    depth: usize,
    sum: i32,
    len: usize,
    now: usize,
    now_sum: i32,
    res: &mut bool,
    nums: &Vec<i32>,
) {
    if depth == len || *res {
        return;
    }
    // println!("{} {}", now, now_sum);
    if now < len && now > 0 && now_sum * (len - now) as i32 > (sum - now_sum) * now as i32 {
        return;
    }
    if now < len && now > 0 && now_sum * (len - now) as i32 == (sum - now_sum) * now as i32 {
        *res = true;
        return;
    }
    dfs(depth + 1, sum, len, now, now_sum, res, nums);
    dfs(
        depth + 1,
        sum,
        len,
        now + 1,
        now_sum + nums[depth],
        res,
        nums,
    );
}

impl Solution {
    pub fn split_array_same_average(mut nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        nums.sort_unstable();
        let mut res = false;
        dfs(0, sum, nums.len(), 0, 0, &mut res, &nums);
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    let a = vec![18, 0, 16, 2];
    println!("res:{}", Solution::split_array_same_average(a));
}
