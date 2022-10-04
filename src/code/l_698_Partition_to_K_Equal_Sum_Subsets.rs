// Solution
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

fn dfs(nums: &Vec<i32>, state: i32, aim: i32, now: i32, vis: &mut HashMap<i32, bool>) -> bool {
    if state == 0 {
        return true;
    }
    if let Some(x) = vis.get(&state) {
        return *x;
    }
    for i in 0..nums.len() {
        if nums[i] + now > aim {
            break;
        }
        if (state >> i) & 1 == 1 && vis.get(&(state ^ (1 << i))).is_none() {
            let state = state ^ (1 << i);
            let res = dfs(nums, state, aim, (now + nums[i]) % aim, vis);
            vis.insert(state, res);
            if res {
                return true;
            }
        }
    }
    false
}

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        if nums.iter().sum::<i32>() % k != 0 {
            return false;
        }
        let aim = nums.iter().sum::<i32>() / k;
        if nums.iter().max().unwrap() > &aim {
            return false;
        }
        let mut vis = HashMap::new();
        nums.sort_unstable();
        dfs(&nums, (1 << nums.len()) - 1, aim, 0, &mut vis)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    //[1,1,1,1,2,2,2,2] 2
    let a = vec![
        129, 17, 74, 57, 1421, 99, 92, 285, 1276, 218, 1588, 215, 369, 117, 153, 22,
    ];
    println!("res:{}", Solution::can_partition_k_subsets(a, 3));
}
