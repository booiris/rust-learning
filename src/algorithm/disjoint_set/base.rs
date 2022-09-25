pub struct DisjointSet {
    father: Vec<usize>,
}

impl DisjointSet {
    #[inline]
    pub fn new(len: usize) -> Self {
        DisjointSet {
            father: (0..len).into_iter().map(|x| x).collect::<Vec<_>>(),
        }
    }

    #[inline]
    pub fn find(&mut self, x: usize) -> usize {
        if self.father[x] != x {
            self.father[x] = self.find(self.father[x]);
        }
        self.father[x]
    }

    #[inline]
    /// 将 y 合并到 x
    pub fn union(&mut self, x: usize, y: usize) {
        let x_father = self.find(x);
        self.father[x_father] = self.find(y);
    }
}

//#[test]
fn main() {
    let mut a = DisjointSet::new(5);
    println!("{:?}", a.father);
    a.union(2, 3);
    a.union(1, 3);
    println!("{:?}", a.father);
    assert_eq!(a.find(2), a.find(3));
}
