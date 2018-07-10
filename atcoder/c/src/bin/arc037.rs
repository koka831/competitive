#![allow(unused_imports)]
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
    let mut edge = vec![0; n];

    for uv in vec {
        uf.unite(uv[0] - 1, uv[1] - 1);
        edge[uv[0] - 1] += 1;
        edge[uv[1] - 1] += 1;
    }

    let mut hm = HashMap::new();

    for j in 0..n {
        hm.entry(uf.find(j))
            .or_insert(Vec::new()).push(j);
    }

    println!("{:?}", hm);
    println!("{:?}", edge);
    let a = hm.into_iter()
        .filter(|&(_, ref v)| 
            2 * (v.len() - 1) == v.iter().map(|&i| edge[i]).sum()
        )
        .count();
    println!("{}", a);
    println!("{:?}", uf.par);
    println!("{:?}", uf.rank);
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

pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                },
                _ => { high = mid; }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                },
                _ => { low = mid + 1; }
            }
        }
        low
    }
}


