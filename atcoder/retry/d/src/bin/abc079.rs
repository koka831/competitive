use std::io;
use std::cmp;


fn main() {
    let (h, _w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut cost = Vec::new();
    for _ in 0..10 {
        cost.push(read::<usize>());
    }

    for k in 0..10 { for i in 0..10 { for j in 0..10 {
        cost[i][j] = cmp::min(cost[i][j], cost[i][k] + cost[k][j]);
    }}}

    let mut ans = 0;
    for _ in 0..h {
        let aw = read::<isize>();
        for &a in aw.iter() {
            if a != -1 { ans += cost[a as usize][1]; }
        }
    }
    println!("{}", ans);
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
