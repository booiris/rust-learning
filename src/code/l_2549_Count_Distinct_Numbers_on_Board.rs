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
static mut PATHS: Vec<PathType> = vec![];
static mut POINT: Vec<Vec<usize>> = vec![];

struct Graph {
    pub start_from: usize,
}

#[allow(dead_code)]
impl Graph {
    pub fn new(p_size: usize, start_from: usize) -> Graph {
        unsafe {
            PATHS.clear();
            POINT.reserve((p_size + start_from).saturating_sub(POINT.capacity()));
            POINT.clear();
            POINT.resize_with(p_size + start_from, || vec![]);
        }
        Graph { start_from }
    }

    pub fn add_path(&mut self, from: usize, to: usize, v: i64) {
        unsafe {
            POINT[from].push(PATHS.len());
            PATHS.push(PathType { from, to, v });
        }
    }

    pub fn add_bi_path(&mut self, from: usize, to: usize, v: i64) {
        unsafe {
            POINT[from].push(PATHS.len());
            PATHS.push(PathType { from, to, v });
            POINT[to].push(PATHS.len());
            PATHS.push(PathType {
                from: to,
                to: from,
                v,
            });
        }
    }

    pub fn get(&self, now_p: usize) -> impl Iterator<Item = &'_ PathType> {
        unsafe { POINT[now_p].iter().map(move |x| PATHS.get_unchecked(*x)) }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            for now_p in self.start_from..POINT.len() {
                write!(f, "{} -- ", now_p);
                for path in self.get(now_p) {
                    write!(f, "{} ", path)?
                }
                writeln!(f)?
            }
        }
        Ok(())
    }
}

fn find_longest_p(g: &Graph, now_p: usize) -> usize {
    fn find_longest_p_inner(
        g: &Graph,
        now_p: usize,
        father: usize,
        depth: i32,
        maxd: &mut i32,
        start: &mut usize,
    ) -> i32 {
        let mut nowd = 0;
        for p in g.get(now_p) {
            if p.to == father {
                continue;
            }
            nowd = nowd.max(find_longest_p_inner(g, p.to, now_p, depth + 1, maxd, start));
        }
        if *maxd < depth {
            *maxd = depth;
            *start = now_p;
        }
        nowd + 1
    }
    let mut start = usize::MAX;
    find_longest_p_inner(g, now_p, now_p, 1, &mut 0, &mut start);
    if start == usize::MAX {
        panic!("can not find longest path")
    }
    start
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

fn to_2_vec<T: Clone, const M: usize, const N: usize>(data: [[T; M]; N]) -> Vec<Vec<T>> {
    data.iter().map(|x| x.to_vec()).collect()
}

#[allow(dead_code)]
impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        n - 1
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}
