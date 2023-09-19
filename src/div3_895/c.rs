#![allow(unused_imports, unused_must_use)]
use std::cmp::*;
use std::collections::*;
use std::io::StdinLock;
use std::io::StdoutLock;
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Bound::*;

fn solve(sc: &mut Scanner<StdinLock>, out: &mut BufWriter<StdoutLock>) {
    let (l, r): (i32, i32) = (sc.sc(), sc.sc());
    if l == r {
        if l == 2 {
            writeln!(out, "-1");
        } else if l % 2 == 0 {
            writeln!(out, "{} {}", l / 2, l / 2);
        } else {
            let mut flag = false;
            for i in 2..(4e3 as i32).min(l) {
                if l % i == 0 {
                    flag = true;
                    let key = l / i;
                    writeln!(out, "{} {}", key * (i / 2), key * (i - i / 2));
                    break;
                }
            }
            if !flag {
                writeln!(out, "-1");
            }
        }
    } else if l % 2 == 0 && l != 2 {
        writeln!(out, "{} {}", l / 2, l / 2);
    } else if r % 2 == 0 && r != 2 {
        writeln!(out, "{} {}", r / 2, r / 2);
    } else if (l + 1) % 2 == 0 && (l + 1) != 2 {
        writeln!(out, "{} {}", (l + 1) / 2, (l + 1) / 2);
    } else if (r - 1) % 2 == 0 && (r - 1) != 2 {
        writeln!(out, "{} {}", (r - 1) / 2, (r - 1) / 2);
    } else {
        writeln!(out, "-1");
    }
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
