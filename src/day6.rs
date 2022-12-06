use std::collections::HashSet;

pub fn marker<const N: usize>(s: &str) -> Option<usize> {
    s.chars()
        .collect::<Vec<_>>()
        .windows(N)
        .enumerate()
        .filter(|(_, a)| a.iter().copied().collect::<HashSet<_>>().len() == N)
        .next()
        .map(|(i, _)| i + N)
}
