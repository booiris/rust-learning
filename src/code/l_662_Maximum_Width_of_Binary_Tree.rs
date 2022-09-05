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

struct Node {
    pos: usize,
    v: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        q.push_back(Node { pos: 0, v: root });
        let mut res = 0;
        while !q.is_empty() {
            let len = q.len();
            let minn = q.front().unwrap().pos;
            let maxn = q.back().unwrap().pos;
            for _ in 0..len {
                let now = q.pop_front().unwrap();
                if let Some(ref x) = now.v {
                    if x.borrow().left.is_some() {
                        q.push_back(Node {
                            pos: now.pos * 2,
                            v: x.borrow().left.clone(),
                        });
                    }
                    if x.borrow().right.is_some() {
                        q.push_back(Node {
                            pos: now.pos * 2 + 1,
                            v: x.borrow().right.clone(),
                        });
                    }
                }
            }
            res = max(res, maxn - minn + 1);
        }
        res as i32
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
