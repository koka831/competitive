use std::io;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();

    let mut ans: usize = 0;
    let mut r = 0;
    let mut s = 0;

    for l in 0..n {
        while r < n && (s ^ an[r] == (s + an[r])) {
            s += an[r];
            r += 1;
        }

        ans += r - l;
        if l == r { r += 1; }
        else { s -= an[l]; }
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
