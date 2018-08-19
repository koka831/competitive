

pub struct UnionFind {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { x }
        else {
            let y = self.par[x];
            self.par[x] = self.find(y);
            self.par[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return; }

        match self.rank[x].cmp(&self.rank[y]) {
            cmp::Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            cmp::Ordering::Less => {
                self.par[x] = y;
            },
            cmp::Ordering::Greater => {
                self.par[y] = x;
            }
        }
    }
}
