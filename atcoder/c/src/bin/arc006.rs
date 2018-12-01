use std::io;


/// https://beta.atcoder.jp/contests/arc006/tasks/arc006_3
/// Greedy
fn main() {
    let n = read_one::<usize>();
    let mut wn = Vec::new();
    for _ in 0..n {
        wn.push(read_one::<isize>());
    }

    let mut state = vec![0; n];
    let mut cnt = 0;
    for w in wn {
        let mut flg = true;
        let mut id = 0;
        let mut diff = std::isize::MAX;
        for i in 0..n {
            if state[i] >= w {
                flg = false;
                if diff > (state[i] - w).abs() {
                    diff = (state[i] - w).abs();
                    id = i;
                }
            }
        }

        if flg {
            state[cnt] = w;
            cnt += 1;
        } else {
            state[id] = w;
        }
    }

    // println!("{:?}", state);

    println!("{}", cnt);
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
