use std::io;
use std::cmp;


/// https://beta.atcoder.jp/contests/arc063/tasks/arc063_b
/// a[i]より右をみればO(n log n) => TLE
/// a[0] ~ a[i - 1]までのminを管理すればO(n)
fn main() {
    let (_, _) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let a = read::<isize>();
    let mut min = 10_000_000_000;
    let mut diff_max = 0;
    let mut ans = 0;
    for i in a.into_iter() {
        min = cmp::min(min, i);
        match (i - min).cmp(&diff_max) {
            cmp::Ordering::Greater => {
                diff_max = i - min;
                ans = 1;
            },
            cmp::Ordering::Equal => ans += 1,
            _ => {}
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
