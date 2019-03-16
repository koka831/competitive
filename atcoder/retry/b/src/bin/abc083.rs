use std::io;


/// https://beta.atcoder.jp/contests/abc083/tasks/abc083_b
fn main() {
    let (n, a, b) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let ans = (1..(n + 1))
        .filter(|&i| a <= digit_sum(i) && digit_sum(i) <= b)
        .sum::<usize>();
    println!("{}", ans);
}

fn digit_sum(n: usize) -> usize {
    let mut x = n;
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x = x / 10;
    }
    sum
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
