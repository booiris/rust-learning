#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::fmt;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rand {
    s: [u64; 4],
}

impl Rand {
    pub fn new(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut seed = <[u64; 4]>::default();
        for chunk in &mut seed {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            *chunk = z;
        }
        Self { s: seed }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let result_plusplus = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(23)
            .wrapping_add(self.s[0]);

        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result_plusplus
    }
}

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

#[allow(dead_code)]
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

#[allow(dead_code)]
static mut TREENODES: Vec<Option<TreeNode>> = vec![];
struct Tree {
    start_from: usize,
}

#[allow(dead_code)]
impl Tree {
    pub fn new(n: usize, start_from: usize) -> Tree {
        unsafe {
            TREENODES.reserve((n + start_from).saturating_sub(INV.capacity()));
            TREENODES.clear();
            TREENODES.resize_with(n + start_from, Default::default)
        }
        Tree { start_from }
    }

    pub fn add_node(&self, root: usize, left: Option<usize>, right: Option<usize>, val: i64) {
        unsafe {
            TREENODES[root] = Some(TreeNode {
                _left: left,
                _right: right,
                val,
                index: root,
            })
        }
    }

    pub fn get_node(&self, p: usize) -> Option<&mut TreeNode> {
        unsafe { TREENODES[p].as_mut() }
    }

    pub fn get(&self, p: usize) -> &TreeNode {
        unsafe { TREENODES[p].as_mut().unwrap() }
    }

    pub fn get_mut(&mut self, p: usize) -> &mut TreeNode {
        unsafe { TREENODES[p].as_mut().unwrap() }
    }

    pub fn root(&mut self) -> &mut TreeNode {
        self.get_mut(self.start_from)
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut q = VecDeque::new();
        q.push_back((self.get(self.start_from), 1));
        let mut maxd = 0;
        while let Some((now, depth)) = q.pop_front() {
            if let Some(left) = now.left() {
                q.push_back((left, depth + 1));
            }
            if let Some(right) = now.right() {
                q.push_back((right, depth + 1));
            }
            maxd = max(maxd, depth);
        }
        let mut q = VecDeque::new();
        let line_len = (qpow(2, maxd + 1, 0) + 10) as usize;
        let mut nowd = 1;
        let mut nowrow = 0;
        let mut div = "".to_string();
        q.push_back((
            self.get(self.start_from),
            nowd,
            line_len / 2,
            format!("({},{})", self.start_from, self.get(self.start_from).val),
        ));
        while let Some((now, depth, aim_index, symbol)) = q.pop_front() {
            if depth > nowd {
                nowd = depth;
                writeln!(f, "\n{}", div);
                nowrow = 0;
                div.clear();
            }
            write!(
                f,
                "{}",
                " ".repeat(aim_index.checked_sub(nowrow).unwrap_or(1))
            );
            write!(f, "{}", symbol);
            nowrow = nowrow + aim_index.checked_sub(nowrow).unwrap_or(1) + symbol.len();
            if let Some(left) = now.left() {
                div += &" ".repeat((aim_index).checked_sub(div.len()).unwrap_or(1));
                div += "/";
                let (index, val) = (left.index, left.val);
                q.push_back((
                    left,
                    depth + 2,
                    aim_index - 3,
                    format!("({},{})", index, val),
                ));
            }
            if let Some(right) = now.right() {
                div += &" ".repeat((nowrow - 1).checked_sub(div.len()).unwrap_or(1));
                div += "\\";
                let (index, val) = (right.index, right.val);
                q.push_back((right, depth + 2, nowrow - 2, format!("({},{})", index, val)));
            }
        }
        Ok(())
    }
}

struct TreeNode {
    _left: Option<usize>,
    _right: Option<usize>,
    pub val: i64,
    pub index: usize,
}

impl TreeNode {
    pub fn right(&self) -> Option<&mut TreeNode> {
        unsafe { self._right.and_then(|x| TREENODES[x].as_mut()) }
    }

    pub fn left(&self) -> Option<&mut TreeNode> {
        unsafe { self._left.and_then(|x| TREENODES[x].as_mut()) }
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

static mut OUT: *mut std::io::BufWriter<std::io::StdoutLock<'_>> = std::ptr::null_mut();
#[allow(unused_macros)]
macro_rules! w {
    ($fmt:expr) => {
    unsafe{ write!(*OUT, "{}", $fmt);}
    };
    ($fmt:expr, $($args:tt)*) => {
    unsafe{  write!(*OUT, $fmt, $($args)*);}
    };
}
#[allow(unused_macros)]
macro_rules! wln {
    () => {
    unsafe{ writeln!(*OUT);}
    };
    ($fmt:expr) => {
    unsafe{ writeln!(*OUT, "{}", $fmt);}
    };
    ($fmt:expr, $($args:tt)*) => {
    unsafe{  writeln!(*OUT, $fmt, $($args)*);}
    };
}
#[allow(unused_macros)]
macro_rules! flush {
    () => {
        unsafe {
            (*OUT).flush();
        }
    };
}

static mut IN: *mut Scanner<StdinLock<'static>> = std::ptr::null_mut();
#[allow(unused_macros)]
macro_rules! i {
    () => {{
        i!(i32)
    }};
    ($t:ty) => {{
        unsafe { (*IN).sc::<$t>() }
    }};
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

pub fn main() {
    unsafe {
        OUT = Box::leak(Box::new(io::BufWriter::new(io::stdout().lock())))
            as *mut std::io::BufWriter<std::io::StdoutLock<'_>>;
        IN = Box::leak(Box::new(Scanner::new(io::stdin().lock()))) as *mut Scanner<StdinLock<'_>>;
    }
    solve();
    flush!();
}

fn solve() {
    const MAXN: usize = 100;
    let (n, k) = (i!(usize), i!(usize));
    let mut g = [[0; MAXN]; MAXN];
    let mut color = [0; MAXN];
    for i in 1..=n {
        for j in 1..=n {
            g[i][j] = i!(i32);
        }
    }
    let mut res = i32::MAX;
    let mut rng = Rand::new(0);
    for _ in 0..5000 as i32 {
        for i in 1..=n {
            color[i] = rng.next_u32() % 2;
        }
        let mut dp = [[i32::MAX / 2; MAXN]; MAXN];
        dp[0][1] = 0;
        for step in 1..=k {
            for i in 1..=n {
                for j in 1..=n {
                    if color[i] != color[j] {
                        dp[step][i] = dp[step][i].min(dp[step - 1][j] + g[j][i]);
                    }
                }
            }
        }
        res = res.min(dp[k][1]);
    }
    wln!(res);
}
