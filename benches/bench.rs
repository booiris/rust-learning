use core::time;
use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, StdinLock},
    time::Instant,
};

// https://bheisler.github.io/criterion.rs/book/getting_started.html
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(22222);
    let mut test_data = vec![];
    for x in (20000..=100000).step_by(20000) {
        println!("gen {} data....", x);
        test_data.push((x, gen_graph(x, &mut rng)));
    }

    let mut group = c.benchmark_group("test graph");
    for x in test_data {
        group.bench_with_input(BenchmarkId::new("safe", x.0), &x, |b, x| {
            b.iter(|| safe_fn(x.0, &x.1));
        });
        group.bench_with_input(BenchmarkId::new("unsafe", x.0), &x, |b, x| {
            b.iter(|| unsafe_fn(x.0, &x.1))
        });
        group.bench_with_input(BenchmarkId::new("unsafe_inline", x.0), &x, |b, x| {
            b.iter(|| unsafe_inline_fn(x.0, &x.1))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn find(x: usize, fa: &mut [usize]) -> usize {
    if fa[x] != x {
        fa[x] = find(fa[x], fa);
    }
    fa[x]
}

fn gen_graph(n: usize, rng: &mut StdRng) -> Vec<(usize, usize)> {
    let mut fa = vec![0; n + 1];
    for i in 0..n {
        fa[i] = i;
    }
    let mut res = Vec::with_capacity(n);
    for _ in 0..n - 1 {
        while true {
            let (a, b) = (rng.gen_range(0..n), rng.gen_range(0..n));
            let mut find = |x| find(x, &mut fa);
            let (a1, b1) = (find(a), find(b));
            if a1 != b1 {
                fa[a1] = b1;
                res.push((a, b));
                break;
            }
        }
    }
    res
}

fn safe_fn(n: usize, path: &[(usize, usize)]) {
    let mut g = Graph::new(n, 0);
    for x in path {
        g.add_bi_path(x.0, x.1, 1);
    }
    dfs(&g, 0, 0);
}

fn unsafe_fn(n: usize, path: &[(usize, usize)]) {
    let mut g = UnsafeGraph::new(n, 0);
    for x in path {
        g.add_bi_path(x.0, x.1, 1);
    }
    unsafe_dfs(&g, 0, 0);
}

fn unsafe_inline_fn(n: usize, path: &[(usize, usize)]) {
    let mut g = UnsafeInlineGraph::new(n, 0);
    for x in path {
        g.add_bi_path(x.0, x.1, 1);
    }
    unsafe_inline_dfs(&g, 0, 0);
}

struct PathType {
    from: usize,
    to: usize,
    v: i64,
}

struct Graph {
    pub paths: Vec<PathType>,
    pub p: Vec<Vec<usize>>,
    pub start_from: usize,
}

#[allow(dead_code)]
impl Graph {
    pub fn new(p_size: usize, start_from: usize) -> Graph {
        Graph {
            paths: vec![],
            p: vec![vec![]; p_size + start_from],
            start_from,
        }
    }

    pub fn add_path(&mut self, from: usize, to: usize, v: i64) {
        self.p[from].push(self.paths.len());
        self.paths.push(PathType { from, to, v });
    }

    pub fn add_bi_path(&mut self, from: usize, to: usize, v: i64) {
        self.p[from].push(self.paths.len());
        self.paths.push(PathType { from, to, v });
        self.p[to].push(self.paths.len());
        self.paths.push(PathType {
            from: to,
            to: from,
            v,
        });
    }

    pub fn get(&self, now_p: usize) -> impl Iterator<Item = &'_ PathType> {
        self.p[now_p]
            .iter()
            .map(move |x| unsafe { self.paths.get_unchecked(*x) })
    }
}

fn dfs(g: &Graph, now_p: usize, father: usize) -> i64 {
    let mut sum = 0;
    for p in g.get(now_p) {
        if p.to == father {
            continue;
        }
        sum += dfs(g, p.to, now_p);
    }
    sum
}
static mut PATHS: Vec<PathType> = vec![];
static mut POINT: Vec<Vec<usize>> = vec![];

struct UnsafeGraph {
    pub start_from: usize,
}

#[allow(dead_code)]
impl UnsafeGraph {
    pub fn new(p_size: usize, start_from: usize) -> UnsafeGraph {
        unsafe {
            PATHS.clear();
            POINT.reserve((p_size + start_from).saturating_sub(POINT.capacity()));
            POINT.clear();
            POINT.resize_with(p_size + start_from, || vec![]);
        }
        UnsafeGraph { start_from }
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
        unsafe {
            POINT[now_p]
                .iter()
                .map(move |x| unsafe { PATHS.get_unchecked(*x) })
        }
    }
}

fn unsafe_dfs(g: &UnsafeGraph, now_p: usize, father: usize) -> i64 {
    let mut sum = 0;
    for p in g.get(now_p) {
        if p.to == father {
            continue;
        }
        sum += unsafe_dfs(g, p.to, now_p);
    }
    sum
}

static mut PATHS_INLINE: Vec<PathType> = vec![];
static mut POINT_INLINE: Vec<Vec<usize>> = vec![];

struct UnsafeInlineGraph {
    pub start_from: usize,
}

#[allow(dead_code)]
impl UnsafeInlineGraph {
    #[inline(always)]
    pub fn new(p_size: usize, start_from: usize) -> UnsafeInlineGraph {
        unsafe {
            PATHS_INLINE.clear();
            POINT_INLINE.reserve((p_size + start_from).saturating_sub(POINT_INLINE.capacity()));
            POINT_INLINE.clear();
            POINT_INLINE.resize_with(p_size + start_from, || vec![]);
        }
        UnsafeInlineGraph { start_from }
    }

    #[inline(always)]
    pub fn add_path(&mut self, from: usize, to: usize, v: i64) {
        unsafe {
            POINT_INLINE[from].push(PATHS_INLINE.len());
            PATHS_INLINE.push(PathType { from, to, v });
        }
    }

    #[inline(always)]
    pub fn add_bi_path(&mut self, from: usize, to: usize, v: i64) {
        unsafe {
            POINT_INLINE[from].push(PATHS_INLINE.len());
            PATHS_INLINE.push(PathType { from, to, v });
            POINT_INLINE[to].push(PATHS_INLINE.len());
            PATHS_INLINE.push(PathType {
                from: to,
                to: from,
                v,
            });
        }
    }

    #[inline(always)]
    pub fn get(&self, now_p: usize) -> impl Iterator<Item = &'_ PathType> {
        unsafe {
            POINT_INLINE[now_p]
                .iter()
                .map(move |x| unsafe { PATHS_INLINE.get_unchecked(*x) })
        }
    }
}

fn unsafe_inline_dfs(g: &UnsafeInlineGraph, now_p: usize, father: usize) -> i64 {
    let mut sum = 0;
    for p in g.get(now_p) {
        if p.to == father {
            continue;
        }
        sum += unsafe_inline_dfs(g, p.to, now_p);
    }
    sum
}
