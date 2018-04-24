struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {

    fn new(n: usize) -> Self {
        let mut vec = vec![0;n];
        for i in 0..n { vec[i] = i; }
        UnionFind {
            par: vec,
            rank: vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let par_x = self.find(x);
        let par_y = self.find(y);
        if self.rank[par_x] > self.rank[par_y] {
            self.par[par_x] = par_x;
        } else {
            self.par[par_x] = par_y;
            if self.rank[par_x] == self.rank[par_y] {
                self.rank[par_y] += 1;
            }
        }
    }
}


