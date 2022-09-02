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

fn dfs(now: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
    if let Some(x) = now {
        let ld = dfs(&x.borrow().left, res);
        let lr = dfs(&x.borrow().right, res);
        let mut sum = 1;
        let mut back = 1;
        if let Some(ref y) = x.borrow().left {
            if y.borrow().val == x.borrow().val {
                sum += ld;
                back = max(back, ld + 1);
            }
        }
        if let Some(ref y) = x.borrow().right {
            if y.borrow().val == x.borrow().val {
                sum += lr;
                back = max(back, lr + 1);
            }
        }
        *res = max(*res, sum);
        return back;
    }
    0
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        dfs(&root, &mut res);
        max(0, res - 1)
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
