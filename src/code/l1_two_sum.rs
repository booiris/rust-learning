#![warn(dead_code)]
#[allow(unused_imports)]
use std::collections::*;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![0];
    }
}

#[cfg(test)]
pub fn main() {
    let t = vec![4, 3, 3];
    println!("{:?}", Solution::two_sum(t, 6));
}
