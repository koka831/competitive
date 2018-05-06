use std::io;


fn main() {
    let (sx, sy, gx, gy) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };

    let mut buf = String::new();
    let dx = (gx - sx).abs();
    let dy = (gy - sy).abs();
    // sx, sy ----> gx, sy
    //   |             |
    //   |             |
    // sx, gy <---- gx, gy
    // 1. sx -> gx
    // 2. sy -> gy
    // 3. gx -> sx
    // 4. gy -> sy
    for _ in 0..dx {
        buf += &"U";
    }

    for _ in 0..dy {
        buf += &"R";
    }

    for _ in 0..dx {
        buf += &"D";
    }

    for _ in 0..dy {
        buf += &"L";
    }

    println!("{}", buf);

}

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
