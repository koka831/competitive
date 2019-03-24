use std::io;


const MOD: usize = 1e9 as usize + 7;

fn main() {
    let n = read_one::<usize>();

    let mut ans: usize = 16;
    let mut prev = vec![1; 16];
    for _ in 2..n {
        for j in 0..16 {
            let mut next = vec![0; 16];
            let x = prev[j];
            if x == 0 { continue; }

            match j {
                0 => { // AA
                    if next[0] == 0 { next[0] = 1; }
                    if next[1] == 0 { next[1] = 1; }
                    if next[3] == 0 { next[3] = 1; }
                    next[0] += x; next[1] += x; next[2] += x; next[3] += x;
                },
                1 => { // AC
                    if next[4] == 0 { next[4] = 1; }
                    if next[5] == 0 { next[5] = 1; }
                    if next[7] == 0 { next[7] = 1; }
                    next[4] += x; next[5] += x; next[7] += x;
                },
                2 => { // AG
                    if next[8] == 0 { next[8] = 1; }
                    if next[9] == 0 { next[9] = 1; }
                    if next[11] == 0 { next[11] = 1; }

                    next[8] += x; next[9] += x; next[11] += x;
                },
                3 => { // AT
                    if next[12] == 0 { next[12] = 1; }
                    if next[13] == 0 { next[13] = 1; }
                    if next[14] == 0 { next[14] = 1; }
                    if next[15] == 0 { next[15] = 1; }

                    next[12] += x; next[13] += x; next[14] += x; next[15] += x;
                },
                4 => { // CC
                    if next[4] == 0 { next[4] = 1; }
                    if next[5] == 0 { next[5] = 1; }
                    if next[6] == 0 { next[6] = 1; }
                    if next[7] == 0 { next[7] = 1; }

                    next[4] += x; next[5] += x; next[6] += x; next[7] += x;
                },
                5 => { // CA
                    if next[0] == 0 { next[0] = 1; }
                    if next[1] == 0 { next[1] = 1; }
                    if next[2] == 0 { next[2] = 1; }
                    if next[3] == 0 { next[3] = 1; }

                    next[0] += x; next[1] += x; next[2] += x; next[3] += x;
                },
                6 => { // CG
                    if next[8] == 0 { next[8] = 1; }
                    if next[9] == 0 { next[9] = 1; }
                    if next[10] == 0 { next[10] = 1; }
                    if next[11] == 0 { next[11] = 1; }
                    next[8] += x; next[9] += x; next[10] += x; next[11] += x;
                },
                7 => { // CT
                    if next[12] == 0 { next[12] = 1; }
                    if next[13] == 0 { next[13] = 1; }
                    if next[14] == 0 { next[14] = 1; }
                    if next[15] == 0 { next[15] = 1; }

                    next[12] += x; next[13] += x; next[14] += x; next[15] += x;
                },
                8 => { // GG
                    if next[8] == 0 { next[8] = 1; }
                    if next[9] == 0 { next[9] = 1; }
                    if next[10] == 0 { next[10] = 1; }
                    if next[11] == 0 { next[11] = 1; }

                    next[8] += x; next[9] += x; next[10] += x; next[11] += x;
                },
                9 => { // GA
                    if next[0] == 0 { next[0] = 1; }
                    if next[1] == 0 { next[1] = 1; }
                    if next[3] == 0 { next[3] = 1; }

                    next[0] += x; next[2] += x; next[3] += x;
                },
                10 => { // GC
                    if next[4] == 0 { next[4] = 1; }
                    if next[5] == 0 { next[5] = 1; }
                    if next[6] == 0 { next[6] = 1; }
                    if next[7] == 0 { next[7] = 1; }

                    next[4] += x; next[5] += x; next[6] += x; next[7] += x;
                },
                11 => { // GT
                    if next[12] == 0 { next[12] = 1; }
                    if next[13] == 0 { next[13] = 1; }
                    if next[14] == 0 { next[14] = 1; }
                    if next[15] == 0 { next[15] = 1; }

                    next[12] += x; next[13] += x; next[14] += x; next[15] += x;
                },
                12 => { // TT
                    if next[12] == 0 { next[12] = 1; }
                    if next[13] == 0 { next[13] = 1; }
                    if next[14] == 0 { next[14] = 1; }
                    if next[15] == 0 { next[15] = 1; }

                    next[12] += x; next[13] += x; next[14] += x; next[15] += x;
                },
                13 => { // TA
                    if next[0] == 0 { next[0] = 1; }
                    if next[1] == 0 { next[1] = 1; }
                    if next[2] == 0 { next[2] = 1; }
                    if next[3] == 0 { next[3] = 1; }

                    next[0] += x; next[1] += x; next[2] += x; next[3] += x;
                },
                14 => { // TC
                    if next[4] == 0 { next[4] = 1; }
                    if next[5] == 0 { next[5] = 1; }
                    if next[6] == 0 { next[6] = 1; }
                    if next[7] == 0 { next[7] = 1; }

                    next[4] += x; next[5] += x; next[6] += x; next[7] += x;
                },
                15 => { // TG
                    if next[8] == 0 { next[8] = 1; }
                    if next[9] == 0 { next[9] = 1; }
                    if next[10] == 0 { next[10] = 1; }
                    if next[11] == 0 { next[11] = 1; }

                    next[8] += x; next[9] += x; next[10] += x; next[11] += x;
                },
                _ => {},
            }
            for i in 0..16 {
                if next[i] > 0 {
                    ans = ans + next[i] % MOD;
                }
            }
            prev = next;
        }
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
