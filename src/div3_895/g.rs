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
    let a = (0..n).map(|_| sc.sc()).collect::<Vec<i64>>();
    let mut temp = 1;
    let s: i64 = a.iter().sum();
    for i in &a {
        temp *= i;
        if temp >= s * 2 {
            let mut x = 0;
            while x < a.len() && a[x] == 1 {
                x += 1;
            }
            let mut y = n - 1;
            while a[y] == 1 {
                y -= 1;
                if y == 0 {
                    break;
                }
            }

            writeln!(out, "{} {}", x + 1, y + 1);
            return;
        }
    }
    let mut s = vec![0; n + 1];
    let mut p = vec![0; n + 1];
    p[0] = 1;
    let mut key = vec![];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
        p[i + 1] = p[i] * a[i];
        if a[i] != 1 {
            key.push(i);
        }
    }
    let mut res = (0, 0);
    let mut maxn = 0;
    for i in 0..key.len() {
        for j in i..key.len() {
            let temp = s[key[i]] + s[n] - s[key[j] + 1] + p[key[j] + 1] / p[key[i]];
            if temp > maxn {
                maxn = temp;
                res = (key[i], key[j]);
            }
        }
    }
    writeln!(out, "{} {}", res.0 + 1, res.1 + 1);
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
