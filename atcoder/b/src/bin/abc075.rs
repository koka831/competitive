use std::io;


/// https://beta.atcoder.jp/contests/abc075/tasks/abc075_b
fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut s = vec![vec![0; w + 2]; h + 2];
    let mut buf = Vec::new();
    for i in 0..h {
        let m = read_one::<String>().chars().collect::<Vec<char>>();
        buf.push(m.clone());
        for j in 0..w {
            s[i + 1][j + 1] = if m[j] == '#' { 1 } else { 0 };
        }
    }
    let mut ans = vec![vec!["#".to_string(); w]; h];
    for i in 0..h { for j in 0..w {
        if buf[i][j] == '.' {
            let mut cnt = 0;
            for x in 0..3 { for y in 0..3 {
                cnt += s[y + i][x + j];
            }}
            ans[i][j] = cnt.to_string();
        }
    }}

    for row in ans {
        println!("{}", row.join(""));
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
