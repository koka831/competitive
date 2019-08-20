pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {

    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] { x }
        else {
            let par = self.par[x];
            self.par[x] = self.find(par);
            self.par[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        use std::cmp::Ordering;
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return; }

        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            Ordering::Less => {
                self.par[x] = y;
            },
            Ordering::Greater => {
                self.par[y] = x;
            }
        }
    }
}
