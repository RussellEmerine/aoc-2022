use std::collections::HashSet;

const ROCKS: [&[(usize, usize)]; 5] = [
    &[(2, 3), (3, 3), (4, 3), (5, 3)],
    &[(2, 4), (3, 3), (3, 4), (3, 5), (4, 4)],
    &[(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)],
    &[(2, 3), (2, 4), (2, 5), (2, 6)],
    &[(2, 3), (2, 4), (3, 3), (3, 4)],
];

#[derive(Debug, Clone, Default)]
struct Tower {
    points: HashSet<(usize, usize)>,
    top: usize,
}

impl Tower {
    fn insert(&mut self, x: usize, y: usize) {
        self.points.insert((x, y));
        self.top = self.top.max(y);
    }

    fn contains(&mut self, x: usize, y: usize) -> bool {
        self.points.contains(&(x, y))
    }
}

pub fn solve(directions: &str) -> usize {
    let mut directions = directions.chars().cycle();
    let mut tower = Tower::default();
    for i in 0..2022 {
        let mut rocks = ROCKS[i % ROCKS.len()];
        todo!();
    }
    todo!()
}
