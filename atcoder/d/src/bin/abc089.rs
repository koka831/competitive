use std::io;


/// https://beta.atcoder.jp/contests/abc089/tasks/abc089_d
/// abst:
/// h*mマスでのL1距離の累積和
/// cost[i] = cost[i-D] + (L1距離)
fn main() {
    let (h, w, d) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut pos = vec![(0, 0); h * w + 1];
    for i in 0..h {
        let ln = read::<isize>();
        let mut j = 0;
        for a in ln.iter() {
            pos[a.clone() as usize] = (i as isize, j as isize);
            j += 1;
        }
    }

    let mut cost = vec![0; h * w + 1];
    for i in d..(h * w + 1) {
        cost[i] = cost[i - d] + ((pos[i].0 - pos[i - d].0) as isize).abs() + ((pos[i].1 - pos[i - d].1) as isize).abs();
    }

    let q = read_one::<usize>();
    for _ in 0..q {
        let (s, t) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        println!("{}", cost[t] - cost[s]);
    }
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
