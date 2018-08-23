use std::io;


/// https://beta.atcoder.jp/contests/agc026/tasks/agc026_a
fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let mut cnt = Vec::new();
    let mut prev = an[0];
    let mut same = 1;
    for i in 1..n {
        if an[i] == prev { same += 1; }
        else { cnt.push(same); same = 1; prev = an[i]; }
        if i == n - 1 { cnt.push(same); }
    }
    // println!("{:?}", cnt);
    let ans = cnt.iter().map(|&n| n / 2).sum::<usize>();
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
