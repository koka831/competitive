use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        match other.cost.cmp(&self.cost) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.pos.cmp(&other.pos)
            }
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

fn dijkstra(adj: &Vec<Vec<Edge>>, s: usize) -> Vec<usize> {
    let mut dist = (0..adj.len()).map(|_| ::std::usize::MAX)
        .collect::<Vec<usize>>();

    let mut q = BinaryHeap::new();
    dist[s] = 0;
    q.push( State { cost: 0, pos: s });

    while let Some( State { cost, pos }) = q.pop() {

        if cost > dist[pos] { continue; }

        for e in &adj[pos] {
            let s = State { cost: cost + e.cost, pos: e.node };
            if s.cost < dist[s.pos] {
                q.push(s);
                dist[s.pos] = s.cost;
            }
        }
    }

    dist
}

fn dfs(g: &Vec<Vec<bool>>, n: usize, s: usize) -> usize {
    let mut visited = (0..n).map(|_| false)
        .collect::<Vec<bool>>();
    let mut q = BinaryHeap::new();
    let mut cnt = 0;
    visited[s] = true;
    q.push(s);

    while let Some(pos) = q.pop() {
        if visited.iter().filter(|&v| *v == true).count() == n {
            cnt += 1;
        }

        for e in &g[pos] {
        }
    }
    cnt
}


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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
