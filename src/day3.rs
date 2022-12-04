use std::collections::HashSet;

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        (c as u8 - b'a') as usize + 1
    } else {
        (c as u8 - b'A') as usize + 27
    }
}

fn find_shared(rucksack: &str) -> char {
    let (a, b) = rucksack.split_at(rucksack.len() / 2);
    let a: HashSet<_> = a.chars().collect();
    let b: HashSet<_> = b.chars().collect();
    *a.intersection(&b)
        .next()
        .expect("found no shared characters")
}

fn calculate(rucksack: &str) -> usize {
    priority(find_shared(rucksack))
}

pub fn solve(rucksacks: Vec<&str>) -> usize {
    rucksacks.iter().map(|&rucksack| calculate(rucksack)).sum()
}

fn badge(a: &str, b: &str, c: &str) -> usize {
    let a: HashSet<_> = a.chars().collect();
    let b: HashSet<_> = b.chars().collect();
    let c: HashSet<_> = c.chars().collect();
    priority(
        ('a'..='z')
            .chain('A'..='Z')
            .filter(|&x| a.contains(&x) && b.contains(&x) && c.contains(&x))
            .next()
            .expect("didn't find item in all three rucksacks"),
    )
}

pub fn more(rucksacks: Vec<&str>) -> usize {
    rucksacks
        .chunks_exact(3)
        .map(|a| badge(a[0], a[1], a[2]))
        .sum()
}
