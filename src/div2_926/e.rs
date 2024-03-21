#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::fmt;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

struct PathType {
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

struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}
impl<B: BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }
    pub fn sc<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

fn dfs(g: &Graph, key: &mut Vec<usize>, now_p: usize, father: usize, emask: &mut Vec<i32>) {
    for path in g.get(now_p) {
        if path.to == father {
            continue;
        }
        dfs(g, key, path.to, now_p, emask);
        emask[now_p] ^= emask[path.to];
    }
    for path in g.get(now_p) {
        if path.to != father && !emask[now_p] & emask[path.to] != 0 {
            key.push(emask[path.to] as usize);
        }
    }
}

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n = sc.sc();
    let mut g = Graph::new(n, 1);
    for _ in 0..n - 1 {
        let (a, b) = (sc.sc(), sc.sc());
        g.add_bi_path(a, b, 1);
    }
    let k = sc.sc();
    let mut emask = vec![0; n + 1];
    for i in 0..k {
        let (a, b) = (sc.sc::<usize>(), sc.sc::<usize>());
        emask[a] |= 1 << i;
        emask[b] |= 1 << i;
    }
    // writeln!(out, "\n{:?}", emask).unwrap();

    let mut key = vec![];
    dfs(&g, &mut key, 1, 0, &mut emask);
    let mut dp = vec![i32::MAX >> 1; (1 << k) + 1];
    dp[0] = 0;
    for x in 0..(1 << k) {
        for y in &key {
            dp[x | *y] = min(dp[x | *y], dp[x] + 1);
        }
    }
    // writeln!(out, "{:?}\n{:?}\n{:?}", key, emask, dp).unwrap();

    writeln!(out, "{}", dp[(1 << k) - 1]).unwrap();
}

pub fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let t: i32 = sc.sc();
    for _ in 0..t {
        solve(&mut sc, &mut out);
    }
}
