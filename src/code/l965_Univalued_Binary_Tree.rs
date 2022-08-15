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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_val(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            if let Some(x) = root {
                // RefCell.borrow()不会转移所有权
                // x.borrow().left不clone()的话，会报error[E0507]错误
                // x.borrow().val == val
                //     && is_val(x.borrow().left.clone(), val)
                //     && is_val(x.borrow().right.clone(), val)
                false
            } else {
                true
            }
        }
        //unwrap()调用会转移所有权，所以先clone()
        let val = root.clone().unwrap().borrow().val;
        is_val(root, val)
    }
}

#[cfg(test)]
pub fn main() {
    println!("res:");
}
