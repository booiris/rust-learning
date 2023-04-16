#![allow(dead_code, unused_imports, unused_macros)]
use std::cmp::*;
use std::collections::*;
use std::f32::consts::E;
use std::ops::AddAssign;
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

#[derive(Clone, Copy)]
pub struct Node {
    num: i32,
    cnt: i32,
}

impl Node {
    #[inline]
    fn new(num: i32, cnt: i32) -> Self {
        Node { num, cnt }
    }
}

impl AddAssign for Node {
    fn add_assign(&mut self, rhs: Self) {
        if self.num == rhs.num {
            self.cnt += rhs.cnt;
        } else if self.cnt >= rhs.cnt {
            self.cnt -= rhs.cnt;
        } else {
            self.num = rhs.num;
            self.cnt = rhs.cnt - self.cnt;
        }
    }
}

pub struct SegTree {
    tree: Vec<Node>,
    val: Vec<i32>,
    len: usize,
}

impl SegTree {
    #[inline]
    pub fn new(arr: Vec<i32>) -> Self {
        let len = arr.len();
        SegTree {
            tree: vec![Node::new(0, 0); len * 4 + 1],
            val: arr,
            len,
        }
    }

    pub fn build(&mut self, id: usize, l: usize, r: usize) {
        if l == r {
            self.tree[id].num = self.val[l];
            self.tree[id].cnt = 1;
            return;
        }

        let mid = (l + r) / 2;
        self.build(id * 2 + 1, l, mid);
        self.build(id * 2 + 2, mid + 1, r);
        let temp = self.tree[id * 2 + 1];
        self.tree[id] += temp;
        let temp = self.tree[id * 2 + 2];
        self.tree[id] += temp;
    }

    pub fn query(&self, id: usize, l: usize, r: usize, ql: usize, qr: usize, ans: &mut Node) {
        if l > qr || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            *ans += self.tree[id];
            return;
        }
        let mid = (l + r) / 2;
        self.query(id * 2 + 1, l, mid, ql, qr, ans);
        self.query(id * 2 + 2, mid + 1, r, ql, qr, ans);
    }
}

struct MajorityChecker {
    seg_tree: SegTree,
    loc: HashMap<i32, Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut loc = HashMap::new();
        for (i, v) in arr.iter().enumerate() {
            loc.entry(*v).or_insert(vec![]).push(i);
        }
        let mut seg_tree = SegTree::new(arr);
        seg_tree.build(0, 0, seg_tree.len - 1);
        MajorityChecker { seg_tree, loc }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let mut ans = Node::new(0, 0);
        self.seg_tree.query(
            0,
            0,
            self.seg_tree.len - 1,
            left as usize,
            right as usize,
            &mut ans,
        );
        let key = self.loc.get(&ans.num);
        if key.is_none() {
            return -1;
        }
        let key = key.unwrap();
        let cnt = key.partition_point(|x| *x <= right as usize)
            - key.partition_point(|x| *x < left as usize);
        if cnt >= threshold as usize {
            ans.num
        } else {
            -1
        }
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */

#[cfg(feature = "local")]
pub fn main() {
    let arr = [1, 1, 2, 2, 1, 1];
    let obj = MajorityChecker::new(arr.into());
    let ini = [[0, 5, 4], [0, 3, 3], [2, 3, 2]];
    for x in ini {
        println!("res:{}", obj.query(x[0], x[1], x[2]));
    }
}
