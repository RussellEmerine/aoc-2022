fn mix(f: &mut Vec<(usize, i64)>) {
    let n = f.len();
    for i in 0..n {
        let x = f.iter().position(|&(j, _)| i == j).unwrap();
        let (_, d) = f[x];
        f.remove(x);
        f.insert((x as i64 + d).rem_euclid(n as i64 - 1) as usize, (i, d));
    }
}

pub fn solve(f: &[i64]) -> i64 {
    let mut f: Vec<_> = f.iter().copied().enumerate().collect();
    mix(&mut f);
    let f: Vec<_> = f.iter().map(|&(_, a)| a).collect();
    let x = f.iter().position(|&a| a == 0).unwrap();
    f[(x + 1000) % f.len()] + f[(x + 2000) % f.len()] + f[(x + 3000) % f.len()]
}

pub fn more(f: &[i64]) -> i64 {
    let mut f: Vec<_> = f.iter().map(|a| a * 811589153).enumerate().collect();
    for _ in 0..10 {
        mix(&mut f);
    }
    let f: Vec<_> = f.iter().map(|&(_, a)| a).collect();
    let x = f.iter().position(|&a| a == 0).unwrap();
    f[(x + 1000) % f.len()] + f[(x + 2000) % f.len()] + f[(x + 3000) % f.len()]
}
