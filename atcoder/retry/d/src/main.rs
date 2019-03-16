use std::io;


fn main() {

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


#[allow(dead_code)]
fn next_permutation<T>(list: &mut[T]) -> bool
    where T: std::cmp::PartialOrd 
{
    let mut i = list.len() - 1;
    while i > 0 && list[i - 1] >= list[i] { i -= 1; }
    if i <= 0 { return false; }

    let mut j = list.len() - 1;
    while list[j] <= list[i - 1] { j -= 1; }

    list.swap(i - 1, j);
    list[i..].reverse();
    true
}
