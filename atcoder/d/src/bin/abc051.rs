use std::io;
use std::collections::BinaryHeap;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = vec![Vec::new(); n];
    let mut cost = vec![vec![0; n]; n];

    for _ in 0..m {
        let (a, b, c) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        vec[a].push((b, c));
        vec[b].push((a, c));
        cost[a][b] = c;
        cost[b][a] = c;
    }

    let mut dist = vec![Vec::new(); n];
    for j in 0..n {
        dist[j] = dijkstra(&vec, j);
    }

    println!("{:?}", vec);

    let mut used = vec![vec![false; n]; n];
    for i in 0..n { used[i][i] = true; }
    for i in 0..n { for j in 0..n { for k in 0..n { // TODO: while let Some
        if dist[i][j] + cost[j][k] == dist[i][k] {
            used[j][k] = true;
        }
    }}}

    let ans = used.iter().map(|ref v|
        v.iter().filter(|&u| *u == true ).count())
        .sum::<usize>();
    println!("{}", ans / 2);
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

// (pos, cost)
fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut dist = (0..adj.len()).map(|_| ::std::usize::MAX)
        .collect::<Vec<usize>>();

    let mut q = BinaryHeap::new();
    dist[s] = 0;
    q.push((s, 0));

    while let Some((pos, cost)) = q.pop() {
        if cost > dist[pos] { continue; }
        for e in &adj[pos] {
            let s = (e.0, e.1 + cost);
            if s.1 < dist[s.0] {
                q.push(s);
                dist[s.0] = s.1;
            }
        }
    }

    dist
}
