#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

fn _gcd<T: Default + std::marker::Copy + std::ops::Rem<Output = T> + std::cmp::PartialEq>(
    a: T,
    b: T,
) -> T {
    if b == T::default() {
        return a;
    }
    _gcd(b, a % b)
}

fn check(s: &[Vec<i32>], d_arr: &[usize], cnt: &HashMap<i32, Vec<usize>>) -> usize {
    let mut c = HashMap::new();
    for x in d_arr {
        for d in &s[*x] {
            *c.entry(d).or_insert(0) += 1;
        }
    }
    let mut res = 0;
    for (k, v) in &c {
        if *v == cnt.get(k).unwrap().len() {
            res += 1;
        }
    }
    res
}

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n: usize = sc.sc();
    let mut s = vec![vec![]; n];
    for x in s.iter_mut() {
        let t: usize = sc.sc();
        *x = (0..t).map(|_| sc.sc()).collect::<Vec<i32>>();
    }
    let mut cnt = HashMap::new();
    for (i, x) in s.iter().enumerate() {
        for y in x {
            cnt.entry(*y).or_insert(vec![]).push(i);
        }
    }
    let all = cnt.keys().len();
    let mut res = 0;
    for v in cnt.values() {
        res = res.max(all - check(&s, v, &cnt));
    }
    writeln!(out, "{res}");
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
