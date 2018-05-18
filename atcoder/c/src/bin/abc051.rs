use std::io;


/// https://beta.atcoder.jp/contests/abc051/tasks/abc051_c
fn main() {
    let (sx, sy, gx, gy) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };

    let mut buf = MyStr::new();
    let dx = (gx - sx).abs();
    let dy = (gy - sy).abs();

    buf.append("U", dy);
    buf.append("R", dx); // (t_x, t_y)
    buf.append("U", 1);
    buf.append("L", dx + 1);
    buf.append("D", dy + 1);
    buf.append("R", 1); // (s_x, s_y)
    buf.append("R", dx);
    buf.append("U", dy); // (t_x, t_y)
    buf.append("R", 1);
    buf.append("D", dy + 1);
    buf.append("L", dx + 1);
    buf.append("U", 1); // (s_x, s_y)

    println!("{}", buf);
}


struct MyStr {
    s: String,
}

impl MyStr {
    fn new() -> MyStr {
        MyStr { s: String::new(), }
    }

    fn append(&mut self, dir: &str, time: isize) {
        for _ in 0..time { self.s += dir; }
    }
}

impl std::fmt::Display for MyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
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
