use std::io;
use std::cmp;
use std::collections::VecDeque;


fn main() {
    let (r, c) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let (sy, sx) = {
        let i = read::<usize>();
        (i[0] - 1, i[1] - 1)
    };
    let (gy, gx) = {
        let i = read::<usize>();
        (i[0] - 1, i[1] - 1)
    };
    let mut g = Vec::new();
    for _ in 0..r {
        g.push(read_one::<String>().chars().collect::<Vec<_>>());
    }

    let mut q = VecDeque::new();
    let mut done = vec![vec![false; c]; r];
    let mut min = 1_000_000_000;
    done[sy][sx] = true;
    q.push_back((sy, sx, 1));
    while let Some((y, x, cost)) = q.pop_front() {
        let mut p = Vec::new();
        if y > 0 && g[y - 1][x] != '#' { p.push((y - 1, x)); }
        if y < r - 1 && g[y + 1][x] != '#' { p.push((y + 1, x)); }
        if x > 0 && g[y][x - 1] != '#' { p.push((y, x- 1)); }
        if x < c - 1 && g[y][x + 1] != '#' { p.push((y, x + 1)); }
        for (vy, vx) in p {
            if vx == gx && vy == gy { min = cmp::min(min, cost); continue; }
            if done[vy][vx] { continue; }
            done[vy][vx] = true;
            q.push_back((vy, vx, cost + 1));
        }
    }

    println!("{}", min);
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
