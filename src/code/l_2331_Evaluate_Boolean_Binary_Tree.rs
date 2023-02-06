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
#[cfg(feature = "local")]
#[derive(Debug, PartialEq, Eq)]
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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let r = root.unwrap();
        let left = r.borrow_mut().left.take();
        let right = r.borrow_mut().right.take();
        let res = match r.borrow().val {
            0 => false,
            1 => true,
            2 => Self::evaluate_tree(left) || Self::evaluate_tree(right),
            _ => Self::evaluate_tree(left) && Self::evaluate_tree(right),
        };
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
