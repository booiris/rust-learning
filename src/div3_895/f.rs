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
    let a = (0..n)
        .map(|_| sc.sc())
        .map(|x: usize| x - 1)
        .collect::<Vec<usize>>();
    let c = (0..n).map(|_| sc.sc()).collect::<Vec<i32>>();
    let mut inkey = vec![0; n];
    for i in 0..n {
        inkey[a[i]] += 1;
    }

    let mut key = inkey
        .iter()
        .enumerate()
        .filter(|(_, x)| x == &&0)
        .map(|(i, _)| i)
        .collect::<VecDeque<_>>();
    while let Some(x) = key.pop_front() {
        write!(out, "{} ", x + 1);
        inkey[a[x]] -= 1;
        if inkey[a[x]] == 0 {
            key.push_back(a[x]);
        }
    }
    let mut s = inkey
        .iter()
        .enumerate()
        .filter(|(_, x)| x != &&0)
        .map(|(i, _)| (Reverse(c[i]), i))
        .collect::<BinaryHeap<(Reverse<i32>, usize)>>();

    let mut vis = vec![false; n];

    while let Some(x) = s.pop() {
        if vis[x.1] {
            continue;
        }
        let init = x.1;
        let mut now = x.1;
        vis[now] = true;
        while a[now] != init {
            write!(out, "{} ", a[now] + 1);
            vis[a[now]] = true;
            now = a[now];
        }
        write!(out, "{} ", init + 1);
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
pub struct Scanner<B> {
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
