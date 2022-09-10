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
#[cfg(feature = "local")]
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = root {
            if x.borrow().val < low {
                return Self::trim_bst(x.borrow_mut().right.take(), low, high);
            }
            if x.borrow().val > high {
                return Self::trim_bst(x.borrow_mut().left.take(), low, high);
            }

            let left = Self::trim_bst(x.borrow_mut().left.take(), low, high);
            let right = Self::trim_bst(x.borrow_mut().right.take(), low, high);

            x.borrow_mut().left = left;
            x.borrow_mut().right = right;

            Some(x)
        } else {
            None
        }
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
