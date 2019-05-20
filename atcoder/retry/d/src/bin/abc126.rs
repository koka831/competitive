use std::io;
use std::collections::VecDeque;


fn main() {
    let n = read_one::<usize>();
    let mut edge = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let (u, v, w) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        edge[u].push((v, w));
        edge[v].push((u, w));
    }

    let mut color = vec![2; n];
    let mut cost = vec![0; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    color[0] = 0;
    while let Some(u) = q.pop_front() {
        for &(v, w) in edge[u].iter() {
            if color[v] != 2 { continue; }
            cost[v] = cost[u] + w;
            if (cost[v] - cost[u]) % 2 == 0 { color[v] = color[u]; }
            else { color[v] = 1 - color[u]; }
            q.push_front(v);
        }
    }

    for i in 0..n { println!("{}", color[i]); }
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
