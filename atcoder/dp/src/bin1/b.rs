use std::io;
use std::cmp;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let hn = read::<isize>();
    let mut dp = vec![::std::isize::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        let mut score = ::std::isize::MAX;
        for j in 1..cmp::min(k + 1, i + 1) {
            score = cmp::min(
                score,
                (hn[i - j] - hn[i]).abs() + dp[i - j]
            );
        }
        dp[i] = score;
    }

    println!("{}", dp[n - 1]);
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
