use std::io;


/// https://beta.atcoder.jp/contests/arc058/tasks/arc058_a
/// x > nに対して各桁の数字がDに含まれていないか見るだけ
fn main() {
    let (n, _k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let d = read::<usize>();
    let mut ans = 0;
    for i in n.. {
        let mut tmp = i;
        let mut flg = true;
        while tmp > 0 {
            if d.binary_search(&(tmp % 10)).is_ok() { flg = false; }
            tmp /= 10;
        }
        if flg { ans = i; break; }
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
