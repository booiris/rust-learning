pub struct Bitree {
    tree: Vec<i32>,
    len: usize,
}
impl Bitree {
    #[inline]
    pub fn new(ini: &Vec<i32>) -> Self {
        let mut res = Bitree {
            tree: vec![0; ini.len() * 4 + 1],
            len: ini.len(),
        };
        for (index, v) in ini.iter().enumerate() {
            res.update(index + 1, *v);
        }
        res
    }

    #[inline]
    fn lowbit(x: usize) -> usize {
        return (x as i32 & -(x as i32)) as usize;
    }

    fn update(&mut self, index: usize, v: i32) {
        let mut pos = index;
        while pos < self.len {
            self.tree[pos] += v;
            pos += Self::lowbit(pos);
        }
    }

    fn get(&self, n: usize) -> i32 {
        let mut res = 0;
        let mut pos = n;
        while pos > 0 {
            res += self.tree[pos];
            pos -= Self::lowbit(pos);
        }
        res
    }

    /// `query` 下标从`1`开始
    pub fn query(&self, l: usize, r: usize) -> i32 {
        self.get(r) - self.get(l - 1)
    }
}

#[cfg(feature = "local")]
fn main() {
    let a = vec![0, 1, 3, 5, 3, 2, 1, 3, 4];
    let solve = Bitree::new(&a);
    println!("{:?}", solve.tree);
    println!("{}", solve.query(1, 5))
}
