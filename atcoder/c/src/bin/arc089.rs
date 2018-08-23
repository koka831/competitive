use std::io;


/// https://beta.atcoder.jp/contests/arc089/tasks/arc089_a
fn main() {
    let n = read_one::<usize>();
    let mut txy = Vec::new();
    for _ in 0..n {
        let i = read::<isize>();
        txy.push((i[0], i[1], i[2]));
    }

    txy.sort_by_key(|&(t, _, _)| -t);
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut prev_t = 0;
    let mut flg = true;

    while let Some((t, x, y)) = txy.pop() {
        let d = (x - prev_x).abs() + (y - prev_y).abs();
        if d > (t - prev_t) { flg = false; }
        if (t - prev_t) % 2 != d % 2 { flg = false; }
        prev_x = x;
        prev_y = y;
        prev_t = t;
    }

    if flg { println!("Yes"); }
    else { println!("No"); }
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
