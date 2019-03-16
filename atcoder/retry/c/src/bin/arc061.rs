use std::io;


fn main() {
    let s = read_one::<String>().chars()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<usize>>();

    let mut ans = 0;
    for j in 0..1 << (s.len() - 1) {
        let mut sum = 0;
        for k in 0..s.len() {
            sum *= 10;
            sum += s[k];
            if (j & 1 << k) != 0 {
                ans += sum;
                sum = 0;
            }
        }
        ans += sum;
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
