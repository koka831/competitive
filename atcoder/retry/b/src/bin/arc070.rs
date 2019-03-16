use std::io;


/// https://beta.atcoder.jp/contests/arc070/tasks/arc070_a
/// (1..t) の部分和からXが生成できるときの最小のt
fn main() {
    let x = read_one::<usize>();
    let mut ans = 0;
    for i in 1.. {
        if (i * (i + 1) / 2) >= x { ans = i; break; }
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
