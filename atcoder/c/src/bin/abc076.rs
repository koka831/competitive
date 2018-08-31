use std::io;


/// https://beta.atcoder.jp/contests/abc076/tasks/abc076_c
/// Dubious Document 2
/// 辞書順最小の文字列なので ? -> a としてよい
/// ?を任意文字とするcontains()でマッチした範囲をtで置換
/// 後方一致だと?b??, ab -> abab (abaaが真)となりうるので
/// 候補はpushしておき最小を出力
fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let t = read_one::<String>().chars().collect::<Vec<char>>();

    if t.len() > s.len() { println!("UNRESTORABLE"); return; }

    let mut ans = Vec::new();
    for i in 0..s.len() - t.len() + 1 {
        let mut buf = s.clone();
        let mut flg = true;
        // println!("{:?}", &buf[i..i + t.len()]);
        // check if s contains t
        for j in 0..t.len() {
            if !(buf[i + j] == '?' || buf[i + j] == t[j]) { flg = false; }
        }
        if !flg { continue; }
        // replace 
        for j in 0..t.len() {
            buf[i + j] = t[j];
        }
        ans.push(buf.iter().map(|&c| if c == '?' { 'a' } else { c }).collect::<Vec<_>>());
    }
    ans.sort();
    if ans.len() > 0 {
        for c in &ans[0] {
            print!("{}", c);
        }
        println!();
    } else {
        println!("UNRESTORABLE");
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
