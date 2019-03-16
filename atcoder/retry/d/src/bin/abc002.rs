use std::io;
use std::cmp;
use std::collections::HashMap;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();
    for _ in 0..m {
        vec.push(read::<usize>());
    }

    let mut uf = UnionFind::new(n);
    for mn in vec.iter() {
        uf.unite(mn[0] - 1, mn[1] - 1);
    }

    let mut hm = HashMap::new();
    for j in 0..n {
        hm.entry(uf.find(j))
            .or_insert(Vec::new()).push(j);
    }

    println!("{:?}", hm);
}


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}


struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
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
            let p = self.par[x];
            let pp = self.find(p);
            self.par[x] = pp;
            pp
        }
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
