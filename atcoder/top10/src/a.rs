use std::io;
use std::str::FromStr;
#[allow(unused_imports)] use std::collections::*;
#[allow(unused_imports)] use std::cmp::{max, min};

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

fn main() {
    main2();
}

fn main1() {
    let (a, b) {
        let input = read::<usize>();
        (input[0], input[1])
    };

    let ans = if (a * b) == 0 { "Even" } else { "Odd" };
    println!("{}", ans);
}

fn main2() {
    let s = read::<String>()[0];
    // sがバイト列であればs.into_bytes().filter(|&c| c == 1).count();
    // Rustでの文字列はString, &strどちらもindexアクセスできない(slice)
    // そのため.chars()等でVec<char>に変換
    let cnt = s.chars().filter(|&c| c == '1').count();
    println!("{}", cnt);
    let oneline = read::<String>()[0]
            .chars()
            .filter(|&c| c == '1')
            .count();
}

/// a1, ..., anの中で2で割り切れる回数が最小のもの, 最小の回数
fn main3() {
    // 二進数表記した際に、下の0が多いもの
    let n = read::<String>()[0];
    let num = read::<usize>();

    // trailing_zeros()
    let ans = num.map(|&n| n.trailing_zeros())
            // min() はiteratorのlenが0の場合を考慮してOption,
            // 今回はn > 0のためunwrap()
            .min().unwrap();
}


/// s(x) := x はxを10進表記した時の桁の和
/// 1<x<n, a<s(x)<b を満たすxの和
fn main4() {
    let (n, a, b) = {
        let r = read::<usize>();
        (r[0], r[1], r[2])
    };

    let ans = (1..=n)
        .filter(|x| {
            let sum = x.to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as u32)
                .sum::<u32>();
            // return Boolean
            // if True: filter pass x
            a <= sum && sum <= b
        }).sum::<u32>();
}

/// a1, ..., anを降順ソート
/// 偶数番目の和と奇数番目の和の差
fn main5() {
    let n = read::<usize>()[0];
    let mut num = read::<usize>();

    // sort()では昇順なので、sort_byを用いる
    num.sort_by(|x, y| y.cmp(x));
    // またはnum.sort()後にnum.reverse()
    // RustのsortはO(n log n)の安定ソート
    // keyAでソートした後にkeyBでソートした場合、keyBでの優先度が同等の要素はkeyAの優先度ソートされる
    let mut even = 0;
    let mut odd = 0;
    for (i, &x) in num.enumerate() {
        if i % 2 == 0 {
            even += x;
        } else {
            odd += x;
        }
    }

    println!("{}", even - odd);
}


/// a1, ..., anから重複する要素を取り除いた時にいくつ残るか
fn main6() {
    use std::collections::BTreeSet;
    let n = read::<usize>()[0];
    let mut num = read<usize>();
    num.sort();
    //de-duplication/重複排除
    num.dedup();
}

fn main7() {
    let a = Some((1, 2));
    // 失敗があるケースではOptionalにしてunwrap_orで
    let b = a.unwrap_or((-1, -1));
}







