use std::io;


fn main() {
    let (n, d, k) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut lr = Vec::new();
    let mut st = Vec::new();
    for _ in 0..d {
        let (l, r) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        lr.push((l, r));
    }
    for _ in 0..k {
        let (s, t) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        st.push((s, t));
    }

    for &(s, t) in st.iter() {
        let mut cnt = 0;
        let mut cur = s;
        for &(l, r) in lr.iter() {
            cnt += 1;
            // s < t
            if s < t {
                if l <= cur && cur < r { cur = r; }
                if t <= cur { println!("{}", cnt); break; }
            } else {
                if l < cur && cur <= r { cur = l; }
                if t >= cur { println!("{}", cnt); break; }
            }
        }
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
