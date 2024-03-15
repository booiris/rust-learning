#![allow(dead_code, unused_imports, unused_macros, unused_must_use)]

#[cfg(feature = "local")]
extern crate data;
#[cfg(feature = "local")]
use crate::data::TreeNode;

use std::cell::RefCell;
use std::cmp::*;
use std::collections::*;
use std::fmt;
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

pub struct PathType {
    from: usize,
    to: usize,
    v: i64,
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.from, self.to, self.v)
    }
}

pub struct Graph {
    pub paths: Vec<PathType>,
    pub p: Vec<Vec<usize>>,
    pub start_from: usize,
}

impl Graph {
    pub fn new(p_size: usize, start_from: usize) -> Graph {
        Graph {
            paths: vec![],
            p: vec![vec![]; p_size + start_from],
            start_from,
        }
    }

    pub fn add_path(&mut self, from: usize, to: usize, v: i64) {
        self.p[from].push(self.paths.len());
        self.paths.push(PathType { from, to, v });
    }

    pub fn add_bi_path(&mut self, from: usize, to: usize, v: i64) {
        self.p[from].push(self.paths.len());
        self.paths.push(PathType { from, to, v });
        self.p[to].push(self.paths.len());
        self.paths.push(PathType {
            from: to,
            to: from,
            v,
        });
    }

    pub fn get(&self, now_p: usize) -> impl Iterator<Item = &'_ PathType> {
        self.p[now_p]
            .iter()
            .map(move |x| unsafe { self.paths.get_unchecked(*x) })
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for now_p in self.start_from..self.p.len() {
            write!(f, "{} -- ", now_p);
            for path in self.get(now_p) {
                write!(f, "{} ", path)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub trait Num:
    std::cmp::PartialEq
    + std::ops::MulAssign
    + std::ops::RemAssign
    + std::marker::Copy
    + std::default::Default
    + std::ops::ShrAssign
    + std::ops::SubAssign
    + std::ops::Div<Output = Self>
    + std::ops::BitAnd<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Add<Output = Self>
{
    fn one() -> Self;
}

#[allow(unused_macros)]
macro_rules! impl_num {
    ($($t:ty)*) => ($(
        impl Num for $t {
            fn one() -> Self {
                1
            }
        }
    )*)
}

impl_num! {i32 u32 i64 u64 usize}

#[allow(dead_code)]
fn qpow<U: Num>(mut x: i64, mut n: U, p: i64) -> i64 {
    let mut res = 1;
    while n != U::default() {
        if n & U::one() != U::default() {
            res *= x;
            if p != 0 {
                res %= p;
            }
        }
        x *= x;
        if p != 0 {
            x %= p;
        }
        n >>= U::one();
    }
    res
}

#[allow(dead_code)]
fn gcd<T: Num>(a: T, b: T) -> T {
    if b == T::default() {
        return a;
    }
    gcd(b, a % b)
}

#[allow(dead_code)]
fn exgcd<T: Num>(a: T, b: T) -> T {
    let (mut x, mut y) = (T::default(), T::default());
    fn inner_exgcd<T: Num>(a: T, b: T, x: &mut T, y: &mut T) {
        if b == T::default() {
            (*x, *y) = (T::one(), T::default());
            return;
        }
        inner_exgcd(b, a % b, y, x);
        *y -= a / b * *x;
    }
    inner_exgcd(a, b, &mut x, &mut y);
    (x + b) % b
}

#[allow(dead_code)]
static mut INV: Vec<i64> = vec![];
#[allow(dead_code)]
fn get_inv(n: usize, p: i64) -> i64 {
    unsafe {
        if INV.len() > n {
            return INV[n];
        }
        INV.reserve(n.checked_sub(INV.capacity()).unwrap_or(1));
        if INV.is_empty() {
            INV.extend_from_slice(&[0, 1]);
        }
        for i in INV.len() as i64..=n as i64 {
            INV.push((p - p / i) * INV[(p % i) as usize] % p);
        }
        INV[n]
    }
}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct FindElements {
    key: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut key = HashSet::new();
        if let Some(ref root) = root {
            dfs(root, 0, &mut key);
        }
        Self { key }
    }

    fn find(&self, target: i32) -> bool {
        self.key.get(&target).is_some()
    }
}

fn dfs(now: &Rc<RefCell<TreeNode>>, v: i32, key: &mut HashSet<i32>) {
    now.borrow_mut().val = v;
    key.insert(v);
    if let Some(left) = now.borrow().left.as_ref() {
        dfs(left, 2 * v + 1, key);
    }
    if let Some(right) = now.borrow().right.as_ref() {
        dfs(right, 2 * v + 2, key);
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}