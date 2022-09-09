#![allow(dead_code, unused_imports, unused_macros)]
use std::cell::RefCell;
use std::cmp::*;
use std::collections::*;
use std::ops::Bound::*;
use std::rc::Rc;
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

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let now = Rc::new(RefCell::new(TreeNode::new(val)));
            now.borrow_mut().left = root;
            return Some(now);
        }

        let mut key = VecDeque::new();
        key.push_front(root.as_ref().unwrap().clone());
        let mut d = 1;
        while !key.is_empty() {
            if depth - 1 == d {
                while !key.is_empty() {
                    let now = key.pop_front().unwrap();
                    let temp = Rc::new(RefCell::new(TreeNode::new(val)));
                    if now.borrow().left.is_some() {
                        temp.borrow_mut().left = Some(now.borrow().left.as_ref().unwrap().clone());
                    }
                    now.borrow_mut().left = Some(temp.clone());
                    let temp = Rc::new(RefCell::new(TreeNode::new(val)));
                    if now.borrow().right.is_some() {
                        temp.borrow_mut().right =
                            Some(now.borrow().right.as_ref().unwrap().clone());
                    }
                    now.borrow_mut().right = Some(temp.clone());
                }
                break;
            }
            let len = key.len();
            for _ in 0..len {
                let now = key.pop_front().unwrap();
                if now.borrow().left.is_some() {
                    key.push_back(now.borrow().left.as_ref().unwrap().clone());
                }
                if now.borrow().right.is_some() {
                    key.push_back(now.borrow().right.as_ref().unwrap().clone());
                }
            }
            d += 1;
        }
        root
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
