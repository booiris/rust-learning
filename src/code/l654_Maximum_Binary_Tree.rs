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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sta: Vec<usize> = vec![];
        let mut key: Vec<Rc<RefCell<TreeNode>>> = vec![];
        for (i, &x) in nums.iter().enumerate() {
            let now = Rc::new(RefCell::new(TreeNode::new(x)));
            while let Some(y) = sta.last() {
                if nums[*y] > x {
                    break;
                }
                now.borrow_mut().left = Some(key[*y].clone());
                sta.pop();
            }
            if let Some(y) = sta.last() {
                key[*y].borrow_mut().right = Some(now.clone());
            }
            key.push(now.clone());
            sta.push(i);
        }
        Some(key[*sta.first().unwrap()].clone())
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
