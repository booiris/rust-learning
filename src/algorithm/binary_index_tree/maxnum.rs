use std::cmp::*;
pub struct Bitree {
    tree: Vec<i32>,
    val: Vec<i32>,
    len: usize,
}
impl Bitree {
    #[inline]
    pub fn new(len: usize) -> Self {
        Bitree {
            tree: vec![0; len + 1],
            val: vec![0; len + 1],
            len,
        }
    }

    #[inline]
    fn lowbit(x: usize) -> usize {
        return (x as i32 & -(x as i32)) as usize;
    }

    fn update(&mut self, index: usize, v: i32) {
        let mut pos = index;
        self.val[pos] = v;
        while pos <= self.len {
            self.tree[pos] = self.val[pos];
            let low = Self::lowbit(pos);
            let mut i = 1;
            while i < low {
                self.tree[pos] = max(self.tree[pos], self.tree[pos - i]);
                i <<= 1;
            }
            pos += Self::lowbit(pos);
        }
    }

    /// `query` 下标从`1`开始 查询范围 [l,r]
    pub fn query(&self, mut l: i32, mut r: i32) -> i32 {
        let mut res = 0;
        l = max(1, l);
        while r >= l {
            res = max(res, self.val[r as usize]);
            r -= 1;
            while r - Self::lowbit(r as usize) as i32 >= l {
                res = max(res, self.tree[r as usize]);
                r -= Self::lowbit(r as usize) as i32;
            }
        }
        res
    }
}
