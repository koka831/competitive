use std::io;


/// https://beta.atcoder.jp/contests/agc017/tasks/agc017_b
/// a[i+1]がとりうる値は[a[i]-D, a[i]-C], [a[i]+C, a[i]+D]
/// 最終的にこの範囲にBが含まれていればok
/// [a-D, a-C]: 減らす操作
/// [a+C, a+D]: 増やす操作
/// として, 各操作はiに依存しないので, それぞれ何回行ったか.
/// 減らす操作: p回, 増やす操作: q = N - 1 - p回とすると, とりうる範囲は
/// [A + p*C - q*D, a + D*p - q*C]
/// 各i = [0, N-1]に対してO(N).
fn main() {
    let (n, a, b, c, d) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3], i[4])
    };

    let mut flg = false;
    for i in 0..n {
        let lower = a + i * c - (n - 1 - i) * d;
        let upper = a + i * d - (n - 1 - i) * c;
        if lower <= b && b <= upper { flg = true; }
    }

    if flg { println!("YES"); }
    else { println!("NO"); }
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
