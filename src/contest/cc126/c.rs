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

fn to_2_vec<T: Clone, const M: usize, const N: usize>(data: [[T; M]; N]) -> Vec<Vec<T>> {
    data.iter().map(|x| x.to_vec()).collect()
}

#[allow(unused_macros)]
macro_rules! curry2 (
    ($f:expr) => {
        |a| move |b|  $f(a, b)
    };
);

#[allow(unused_macros)]
macro_rules! curry3 (
    ($f:expr) => {
        |a| move |b| move |c| $f(a, b, c)
    };
);

#[allow(unused_macros)]
macro_rules! curry4 (
    ($f:expr) => {
        |a| move |b| move |c| move |d| $f(a, b, c, d)
    };
);

#[allow(unused_macros)]
macro_rules! curry5 (
    ($f:expr) => {
        |a| move |b| move |c| move |d| move |e| $f(a, b, c, d, e)
    };
);

#[allow(dead_code)]
impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let mut key = s.chars().filter(|x| x != &'?').fold([0; 30], |mut cur, x| {
            cur[(x as u8 - 'a' as u8) as usize] += 1;
            cur
        });
        let mut res = vec![];
        for x in s.chars() {
            if x == '?' {
                let mut minn = i32::MAX;
                let mut index = 0;
                for i in 0..26 {
                    if minn > key[i] {
                        minn = key[i];
                        index = i;
                    }
                }
                key[index] += 1;
                res.push((index as u8 + 'a' as u8) as char);
            }
        }
        res.sort_unstable();
        let mut index = 0;
        let mut ans = "".to_string();
        for x in s.chars() {
            if x == '?' {
                ans.push(res[index]);
                index += 1;
            } else {
                ans.push(x);
            }
        }

        ans
    }
}

#[cfg(feature = "local")]
pub fn main() {
    println!("res:");
}