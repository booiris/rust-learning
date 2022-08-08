use std::borrow::Borrow;
#[allow(dead_code)]
#[allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
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
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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

fn dfs(now: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
    if let Some(x) = now {
        println!("{}",x.as_ref().val)
        // dfs(x.borrow().left.clone(), res);
        // dfs(x.borrow().right.clone(), res);
    }
    return 1;
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res;
        dfs(root, &mut res);
        res
    }
}

#[cfg(test)]
pub fn main() {
    println!("res:");
}
