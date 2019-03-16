use std::io;


/// https://atc001.contest.atcoder.jp/tasks/dfs_a
/// DFS
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

    let mut sx = 0;
    let mut sy = 0;
    for i in 0..h { for j in 0..w {
        if vec[i][j] == 's' { sx = j; sy = i; }
    }}

    let mut stack = Vec::new();
    let mut arrived = vec![vec![false; w]; h];
    let mut flg = false;
    static DX: &'static [isize] = &[0, 1, 0, -1];
    static DY: &'static [isize] = &[1, 0, -1, 0];
    stack.push((sx, sy));
    while let Some((x, y)) = stack.pop() {
        for i in 0..4 {
            let xx = x as isize + DX[i];
            let yy = y as isize + DY[i];
            if 0 <= xx && xx < w as isize && 0<= yy && yy < h as isize && !arrived[yy as usize][xx as usize] && vec[yy as usize][xx as usize] != '#' {
                arrived[yy as usize][xx as usize] = true;
                if vec[yy as usize][xx as usize] == 'g' { flg = true; }
                else { stack.push((xx as usize, yy as usize)); }
            }
        }
    }

    if flg { println!("Yes"); }
    else { println!("No"); }
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
