use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();
    let mut s = vec![0; n + 1];
    s[1] = an[0];
    for i in 2..n + 1 {
        s[i] = s[i - 1] + an[i - 1];
    }

    let mut si = Vec::new();
    for i in 0..n {
        si.push(s[i] - i);
    }

    let mut ans = 0;
    for i in 0..n { for j in (i + 1)..n + 1 {
        if (s[j] - s[i]) % k == j - i {
            println!("{}, {}", j, i);
            ans += 1;
        }
    }}
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
