use std::io;
use std::collections::VecDeque;


fn main() {
    let n = read_one::<usize>();
    let mut edge = vec![Vec::new(); n];
    for _ in 1..n {
        let (a, b, c) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        edge[a].push((b, c));
        edge[b].push((a, c));
    }

    let (q, k) = {
        let i = read::<usize>();
        (i[0], i[1] - 1)
    };

    let mut dist = vec![0; n];
    let mut que = VecDeque::new();
    que.push_back(k);
    while let Some(v) = que.pop_front() {
        for &(u, w) in edge[v].iter() {
            if dist[u] != 0 { continue; }
            dist[u] = dist[v] + w;
            que.push_back(u);
        }
    }

    for _ in 0..q {
        let (x, y) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        println!("{}", dist[x] + dist[y]);
    }
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
