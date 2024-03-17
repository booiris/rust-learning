#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::f32::consts::E;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

pub fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let t: usize = sc.sc();
    let mut res = vec![vec![0; 55]; 55];
    for _ in 0..t {
        let (n, m) = (sc.sc::<usize>(), sc.sc::<usize>());
        let mut color: bool = true;
        for i in (0..n).step_by(2) {
            let start = color;
            for j in (0..m).step_by(2) {
                if color {
                    res[i][j] = 0;
                    res[i + 1][j + 1] = 0;
                    res[i + 1][j] = 1;
                    res[i][j + 1] = 1;
                } else {
                    res[i][j] = 1;
                    res[i + 1][j + 1] = 1;
                    res[i + 1][j] = 0;
                    res[i][j + 1] = 0;
                }
                color = !color;
            }
            color = !start;
        }

        for i in 0..n {
            for j in 0..m {
                write!(out, "{} ", res[i][j]).unwrap();
            }
            write!(out, "\n").unwrap();
        }
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
