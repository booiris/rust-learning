use std::borrow::Borrow;
#[allow(dead_code)]
#[allow(unused_imports)]
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

// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq)]
#[cfg(feature = "local")]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[cfg(feature = "local")]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn dfs(mut father: Option<Rc<RefCell<TreeNode>>>, is_left: bool, nums: Vec<i32>, index: usize) {
    let mut now;
    let mut maxnum = -1;
    if is_left {
        for i in 0..index {
            if maxnum < nums[i] {
                maxnum = nums[i];
                now = i;
            }
        }
        let temp = Some(TreeNode::new(maxnum));
        father = temp;
        dfs(temp.borrow())
    } else {
        for i in index + 1..nums.len() {
            if maxnum < nums[i] {
                maxnum = nums[i];
                now = i;
            }
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
