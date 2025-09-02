#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    unused_must_use,
    static_mut_refs,
    unexpected_cfgs
)]

#[cfg(feature = "local_build")]
extern crate data;
#[cfg(feature = "local_build")]
use crate::data::TreeNode;

#[cfg(feature = "local")]
use crate::data::TreeNode;

use std::cell::RefCell;
use std::cmp::*;
use std::collections::*;
use std::fmt;
use std::ops::Bound::*;
use std::rc::Rc;
#[cfg(any(feature = "local_build", feature = "local"))]
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
fn qpow<U: Num>(x: impl Into<i64>, mut n: U, p: i64) -> i64 {
    let mut x: i64 = x.into();
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

#[derive(Clone, Debug)]
struct Dsu {
    pub pa: Vec<usize>,
    pub size: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let data = (0..n).map(|i| i).collect();
        let size = vec![1; n];
        Self { pa: data, size }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.pa[x] == x {
            return x;
        }
        self.pa[x] = self.find(self.pa[x]);
        self.pa[x]
    }

    pub fn unit(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.pa[y] = x;
        self.size[x] += self.size[y];
    }

    pub fn erase(&mut self, x: usize) {
        let x = self.find(x);
        self.size[x] -= 1;
        self.pa[x] = x;
    }

    pub fn move_(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx == fy {
            return;
        }
        self.pa[x] = fy;
        self.size[fx] -= 1;
        self.size[fy] += 1;
    }
}

fn comb(n: impl Into<i64>, m: impl Into<i64>, modn: i64) -> i64 {
    let n: i64 = n.into();
    let m = m.into();
    let mut res = 1;
    let mut down = 1;
    for i in 0..m {
        down = (down * (i + 1)) % modn;
        res = (res * (n - i)) % modn;
    }
    let down = exgcd(down, modn);
    down * res % modn
}

macro_rules! p {
    ($arg:expr) => {
        #[cfg(any(feature = "local_build", feature = "local"))]
        println!("{} = {:?}", stringify!($arg), $arg)
    };

    ($($arg:expr),+ $(,)?) => {
        #[cfg(any(feature = "local_build", feature = "local"))]
        println!(
            concat!($(stringify!($arg), " = {:?}, ",)+),
            $($arg,)+
        )
    };
}

#[allow(dead_code)]
#[cfg(any(feature = "local_build", feature = "local"))]
pub fn main() {
    println!(
        "res:{}",
        Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2)
    );
}

#[derive(Debug, Clone, Copy)]
struct ClassRatio {
    unpass: i32,
    total: i32,
    val: f64,
}

const EPS: f64 = 1e-8;
const N: usize = 100005;

impl ClassRatio {
    fn new(pass: i32, total: i32) -> Self {
        let unpass = total - pass;
        Self {
            unpass,
            total,
            val: 0.0,
        }
    }

    fn get(&mut self) {
        self.val = self.unpass as f64 / (self.total as f64 * (self.total + 1) as f64);
    }

    const fn default() -> Self {
        Self {
            unpass: 0,
            total: 0,
            val: 0.0,
        }
    }
}

fn func(d: i32, b: i32, y: f64) -> i32 {
    ((d as f64 * y + 0.25).sqrt() - b as f64 + 0.5 + EPS) as i32
}

static mut A: [ClassRatio; N] = [ClassRatio::default(); N];

fn find(a: &mut [ClassRatio], n: usize, mut r: i32) {
    if r == 0 {
        return;
    }

    let mid = rand::random::<usize>() % n;
    let mut s = 0;
    let v = a[mid].val;
    let mut i = 0 as i32;
    let mut j = n as i32 - 1;
    loop {
        while a[i as usize].val < v {
            i += 1;
        }
        while a[j as usize].val > v {
            j -= 1;
        }
        if i <= j {
            a.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }
        if i > j {
            break;
        }
    }
    let mid = i as usize;
    let v = 1.0 / v;
    for i in mid..n {
        s += func(a[i].unpass, a[i].total, v);
    }
    if s >= r {
        find(&mut a[mid..], n - mid, r);
    } else {
        for i in mid..n {
            let now = func(a[i].unpass, a[i].total, v);
            a[i].total += now;
            r -= now;
            a[i].get();
        }
        if n == 1 {
            a[0].total += 1;
            r -= 1;
            a[0].get();
        }
        find(a, n, r);
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut n = 0;
        let mut res = 0.0;
        for class in &classes {
            if class[0] != class[1] {
                unsafe {
                    A[n] = ClassRatio::new(class[0], class[1]);
                    A[n].get();
                    n += 1;
                }
            }
        }
        if n == 0 {
            return 1.0;
        }
        unsafe {
            find(&mut A, n, extra_students);
        }
        for i in 0..n {
            unsafe {
                res += A[i].unpass as f64 / A[i].total as f64;
            }
        }
        1.0 - res / classes.len() as f64
    }
}
