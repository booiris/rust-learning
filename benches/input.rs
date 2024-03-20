use core::time;
use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, StdinLock},
    time::Instant,
};

// https://bheisler.github.io/criterion.rs/book/getting_started.html
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut f = File::open("bench_in").unwrap();
    unsafe {
        f.read_to_end(&mut DATA);
        println!("{}", DATA.len());
    }

    let mut group = c.benchmark_group("test in");
    group.bench_function("safe", |b| {
        b.iter_custom(|iters| {
            let mut res = time::Duration::default();
            for _i in 0..iters {
                unsafe {
                    let cursor = Cursor::new(&DATA);
                    let mut reader = BufReader::new(cursor);
                    IN = Box::into_raw(Box::new(Scanner::new(reader)))
                }
                let start = Instant::now();
                black_box(test_in());
                res += start.elapsed();
                unsafe { drop(Box::from_raw(IN)) }
            }
            res
        })
    });
    group.bench_function("unsafe", |b| {
        b.iter_custom(|iters| {
            let mut res = time::Duration::default();
            for _i in 0..iters {
                STDIN.with(|x| {
                    let mut x = x.borrow_mut();
                    unsafe {
                        let cursor = Cursor::new(&DATA);
                        let mut reader = BufReader::new(cursor);
                        *x = Scanner::new(reader);
                    }
                });

                let start = Instant::now();
                black_box(test_safe_in());
                res += start.elapsed();
            }
            res
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

static mut DATA: Vec<u8> = vec![];

fn test_in() {
    unsafe {
        let t = (*IN).sc::<i32>();
        for _ in 0..t {
            let n = (*IN).sc::<i32>();
            for _ in 0..n {
                let d = (*IN).sc::<i32>();
            }
        }
    };
}

fn test_safe_in() {
    let t = safe_input();
    for _ in 0..t {
        let n = safe_input();
        for _ in 0..n {
            let d = safe_input();
        }
    }
}

#[inline(always)]
fn safe_input() -> i32 {
    STDIN.with(|r| {
        let mut r = r.borrow_mut();
        r.sc::<i32>()
    })
}

struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}
impl<B: std::io::BufRead> Scanner<B> {
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

static mut IN: *mut Scanner<BufReader<Cursor<&Vec<u8>>>> = std::ptr::null_mut();
#[allow(unused_macros)]
macro_rules! i {
    () => {{
        i!(i32)
    }};
    ($t:ty) => {{
        unsafe { (*IN).sc::<$t>() }
    }};
}

thread_local! {
    pub static STDIN: std::cell::RefCell<Scanner<BufReader<Cursor<&'static Vec<u8>>>>> = std::cell::RefCell::new(Scanner::new((|| {
        unsafe{
            let cursor = Cursor::new(&DATA);
            BufReader::new(cursor)
        }
    })()));
}
#[allow(unused_macros)]
macro_rules! safe_i {
    () => {{
        safe_i!(i32)
    }};
    ($t:ty) => {
        STDIN.with(|r| {
            let mut r = r.borrow_mut();
            r.sc::<$t>()
        })
    };
}

thread_local! {
    pub static STDIN1: std::cell::RefCell<Scanner<StdinLock<'static>>> =
    std::cell::RefCell::new(Scanner::new(io::stdin().lock()));
}

// 测试数据生成代码
// import random

// with open("bench_in", "w") as f:
//     f.write("1000\n")
//     for i in range(1000):
//         f.write("5000\n")
//         for _ in range(5000):
//             rand_data = random.randint(1, 10000)
//             f.write(str(rand_data) + " ")
//         f.write("\n")
