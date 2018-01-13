use std::io;
use std::collections::HashMap;


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
    let _ = read::<u32>()[0];
    let arr = read::<u32>();

    let mut hm = HashMap::new();

    for num in arr {
        *hm.entry(num).or_insert(0) += 1;
    }

        let cnt = hm.into_iter()
            .map(|(idx, num)| if num >= idx { num - idx } else { num })
            .sum::<u32>();

        println!("{}", cnt);
}
