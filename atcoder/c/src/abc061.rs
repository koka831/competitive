use std::io;


/// Abstract
/// a, b, N < 10^5
/// K < sum(b_i)
/// A = [a_1, a_1, ..(b_1)., a_2, a_2, ..(b_2), ..., a_n]
/// A[K] = ?
fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec: Vec<Vec<usize>> = Vec::new();

    for _ in 0..n {
        vec.push(read::<usize>());
    }

    vec.sort();
    vec.reverse();

    let mut count = 0;
    let mut ans = 0;
    while count < k {
        let n = vec.pop().unwrap();
        count += n[1];
        if count >= k { ans = n[0]; }
    }

    println!("{}", ans);
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
