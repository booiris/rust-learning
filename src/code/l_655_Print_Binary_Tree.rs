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

fn get_height(now: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(x) = now {
        let deepl = get_height(&x.borrow().left);
        let deepr = get_height(&x.borrow().right);
        return max(deepl, deepr) + 1;
    }
    0
}

fn solve(
    now: &Option<Rc<RefCell<TreeNode>>>,
    res: &mut Vec<Vec<String>>,
    depth: usize,
    index: usize,
    height: usize,
) {
    if let Some(x) = now {
        res[depth][index] = x.borrow().val.to_string();
        solve(
            &x.borrow().left,
            res,
            depth + 1,
            index - (1 << (height - depth - 1)),
            height,
        );
        solve(
            &x.borrow().right,
            res,
            depth + 1,
            index + (1 << (height - depth - 1)),
            height,
        );
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = get_height(&root) as usize;
        let m = (1 << height) - 1;
        let n = height;
        let mut res = vec![vec![String::from(""); m]; n];
        solve(&root, &mut res, 0, (m - 1) / 2, height - 1);
        res
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
