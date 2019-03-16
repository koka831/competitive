use std::io;


fn main() {
    let (n, m) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();
    let mut cd = Vec::new();

    for _ in 0..n {
        let i = read::<isize>();
        ab.push((i[0], i[1]));
    }

    for _ in 0..m {
        let i = read::<isize>();
        cd.push((i[0], i[1]));
    }

    for &(a, b) in &ab {
        let mut dist = 20isize.pow(9);
        let mut ans = 0;
        for i in 0..cd.len() {
            let (c, d) = cd[i];
            let tmp_dist = (c - a).abs() + (d - b).abs();
            if tmp_dist < dist {
                dist = tmp_dist;
                ans = i + 1;
            }
        }
        println!("{}", ans);
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
