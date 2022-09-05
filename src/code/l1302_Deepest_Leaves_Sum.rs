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

fn dfs(now: Option<Rc<RefCell<TreeNode>>>, depth: i32, maxdepth: &mut i32, key: &mut i32) {
    if let Some(ref x) = now {
        if depth > *maxdepth {
            *maxdepth = depth;
            *key = 0;
        }
        if depth == *maxdepth {
            *key += x.borrow().val;
        }
        dfs(x.borrow_mut().left.take(), depth + 1, maxdepth, key);
        dfs(x.borrow_mut().right.take(), depth + 1, maxdepth, key);
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut maxdepth = 0;
        let mut key = 0;
        dfs(root, 0, &mut maxdepth, &mut key);
        key
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
