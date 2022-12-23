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
        self.top = self.top.max(y + 1);
    }

    fn contains(&mut self, x: usize, y: usize) -> bool {
        self.points.contains(&(x, y))
    }
}

pub fn solve(directions: &str) -> usize {
    let mut directions = directions.chars().cycle();
    let mut tower = Tower::default();
    for i in 0..2022 {
        let mut rocks = ROCKS[i % ROCKS.len()].to_vec();
        for point in rocks.iter_mut() {
            point.1 += tower.top;
        }
        rocks = loop {
            let mut next = rocks.clone();
            let direction = directions.next().unwrap();
            if direction == '<' {
                if next.iter().all(|&(x, _)| x > 0) {
                    for point in next.iter_mut() {
                        point.0 -= 1;
                    }
                }
            } else {
                if next.iter().all(|&(x, _)| x + 1 < 7) {
                    for point in next.iter_mut() {
                        point.0 += 1;
                    }
                }
            }
            if next.iter().any(|&(x, y)| tower.contains(x, y)) {
                next = rocks.clone();
            } else {
                rocks = next.clone();
            }
            if next.iter().all(|&(_, y)| y > 0) {
                for point in next.iter_mut() {
                    point.1 -= 1;
                }
            } else {
                break rocks;
            }
            if next.iter().any(|&(x, y)| tower.contains(x, y)) {
                break rocks;
            }
            rocks = next;
        };
        for (x, y) in rocks {
            tower.insert(x, y);
        }
    }
    tower.top
}
