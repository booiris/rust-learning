#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let n: usize = sc.sc();
    let a = (0..n).map(|_| sc.sc()).collect::<Vec<i32>>();
    let s: String = sc.sc();
    let s = s.chars().collect::<Vec<_>>();
    let mut key = 0;
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        dp[i + 1] = dp[i] ^ a[i];
        if s[i] == '0' {
            key ^= a[i];
        }
    }
    let q: usize = sc.sc();
    for _ in 0..q {
        let tp: i32 = sc.sc();
        if tp == 1 {
            let (l, r): (usize, usize) = (sc.sc(), sc.sc());
            key ^= dp[r] ^ dp[l - 1];
        } else {
            let aim: i32 = sc.sc();
            if aim == 0 {
                write!(out, "{} ", key);
            } else {
                write!(out, "{} ", dp[n] ^ key);
            }
        }
    }
    writeln!(out);
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
