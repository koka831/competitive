use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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

fn main() {
    let (mut a, b) = {
        let i = read::<usize>();
        (std::cmp::max(i[0], i[1]), std::cmp::min(i[0], i[1]))
    };

    // 問題では温度の上下がある
    // a < b として, b => aまで下げる手順とする
    println!("{} {}", a, b);

    let mut cnt = 0;
    while a != b {
        if a >= b + 10 {
            a -= 10;
        } else if a >= b + 5 {
            a -= 5;
        } else {
            a -= 1;
        }
        cnt += 1;
    }

    println!("{}", cnt);
}
