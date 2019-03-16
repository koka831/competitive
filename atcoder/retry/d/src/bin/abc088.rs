use std::io;
use std::collections::VecDeque;


/// https://beta.atcoder.jp/contests/abc088/tasks/abc088_d
fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();
    for _ in 0..h {
        let row = read_one::<String>().chars().collect::<Vec<char>>();
        vec.push(row);
    }

    println!("{:?}", vec);

    const DXDY: &[(isize, isize); 4] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut que = VecDeque::new();
    let mut cost = 0;
    let mut arrived = vec![vec![false; w]; h];
    que.push_back((0, 0));
    while let Some((x, y)) = que.pop_front() {
        println!("{}, {}", x, y);
        if x == w - 1 && y == h - 1 { break; }
        for (dx, dy) in DXDY.into_iter() {
            if check_bound(x, y, *dx, *dy, h, w) {
                // (i i ) as u
                if !arrived[x + *dy as usize][y + *dy as usize] {
                    arrived[x + *dy as usize][y + *dy as usize] = true;
                    que.push_back((x + *dx as usize, y + *dy as usize));
                }
            }
        }
        cost += 1;
    }

    fn check_bound(x: usize, y: usize, dx: isize, dy: isize, h: usize, w: usize) -> bool {
        let xx = x as isize + dx;
        let yy = y as isize + dy;
        if 0 <= xx && (xx as usize) < w && 0 <= yy && (yy as usize) < h { true }
        else { false }
    }

    let black = vec.iter().flatten().map(|&x| if x == '#' { 1 } else { 0 }).sum::<usize>();
    println!("black: {}", black);
    println!("{}", h * w - black - cost);
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
