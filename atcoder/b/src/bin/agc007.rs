use std::io;


/// https://beta.atcoder.jp/contests/agc007/tasks/agc007_a
fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut a = Vec::new();
    for _ in 0..h {
        let i = read_one::<String>().chars().collect::<Vec<char>>();
        a.push(i);
    }

    let mut x = 0;
    let mut y = 0;
    let mut flg = true;
    for i in 0..h { for j in 0..w {
        if a[i][j] == '#' {
            if !(y <= i && x <= j) { flg = false; }
            x = j; y = i;
        }
    }}
    if flg { println!("Possible"); }
    else { println!("Impossible"); }
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
