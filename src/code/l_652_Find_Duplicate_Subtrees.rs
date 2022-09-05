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
#[derive(PartialEq, Eq, Hash)]
struct KeyS(i32, i32, i32);

fn dfs(
    now: &Option<Rc<RefCell<TreeNode>>>,
    index: &mut i32,
    key: &mut HashMap<KeyS, i32>,
    res: &mut HashSet<i32>,
    vis: &mut HashMap<i32, Rc<RefCell<TreeNode>>>,
) -> i32 {
    if let Some(x) = now {
        let temp = KeyS(
            x.borrow().val,
            dfs(&x.borrow().left, index, key, res, vis),
            dfs(&x.borrow().right, index, key, res, vis),
        );
        if let Some(x) = key.get(&temp) {
            res.insert(*x);
            return *x;
        } else {
            key.insert(temp, *index);
            vis.insert(*index, x.clone());
            *index += 1;
            return *index - 1;
        }
    }
    0
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut index = 1;
        let mut key = HashMap::new();
        let mut res = HashSet::new();
        let mut vis = HashMap::new();
        dfs(&root, &mut index, &mut key, &mut res, &mut vis);
        res.iter().map(|x| Some(vis[x].clone())).collect()
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
