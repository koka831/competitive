use std::io;


// 長さnの正の整数列 a1,a2,…,ana1,a2,…,an と整数 xx が与えられる。
// 整数列の連続する部分列で、その総和が xx 以下となるものを数え上げよ
// (実際の出題は QQ 個のクエリがあって各クエリごとに xx が与えられる)。
// しゃくとり法例題
// https://qiita.com/drken/items/ecd1a472d3a0e7db8dce

fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let a = read::<usize>();
    let mut res = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {

        while right < n && sum + a[right] <= x {
            sum += a[right];
            right += 1;
        }

        res += right - left;
        if right == left { right += 1; }
        else { sum -= a[left]; }
    }

    println!("{}", res);
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
