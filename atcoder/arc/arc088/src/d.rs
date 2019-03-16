use std::io;
use std::cmp;


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

fn main() {
    let s = read::<String>()[0]
        .chars().collect::<Vec<char>>();
    println!("{:?}", s);
    let len = s.len() - 1;
    let mut pos = 0;
    let mut ans = len;

    while pos < len {
        let c = s[pos];
        pos += 1;

        while pos < len && c == s[pos] {
            pos += 1;
        }

        ans = cmp::min(ans, cmp::max(pos, len - pos));
    }
    println!("{}", ans);
}
