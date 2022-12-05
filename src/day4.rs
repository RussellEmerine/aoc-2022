pub fn solve(v: Vec<((usize, usize), (usize, usize))>) -> usize {
    v.iter()
        .filter(|&&((i, j), (k, l))| (i <= k && l <= j) || (k <= i && j <= l))
        .count()
}

pub fn more(v: Vec<((usize, usize), (usize, usize))>) -> usize {
    v.iter()
        .filter(|&&((i, j), (k, l))| i <= l && k <= j)
        .count()
}
