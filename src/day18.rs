use std::collections::{HashMap, HashSet};

pub fn solve(cubes: &[(usize, usize, usize)]) -> usize {
    let mut xs = HashMap::<_, usize>::new();
    let mut ys = HashMap::<_, usize>::new();
    let mut zs = HashMap::<_, usize>::new();
    for &(x, y, z) in cubes {
        *xs.entry((x, y, z)).or_default() += 1;
        *xs.entry((x + 1, y, z)).or_default() += 1;
        *ys.entry((x, y, z)).or_default() += 1;
        *ys.entry((x, y + 1, z)).or_default() += 1;
        *zs.entry((x, y, z)).or_default() += 1;
        *zs.entry((x, y, z + 1)).or_default() += 1;
    }
    xs.values().filter(|&&c| c == 1).count()
        + ys.values().filter(|&&c| c == 1).count()
        + zs.values().filter(|&&c| c == 1).count()
}

const BOUND: usize = 30;

fn neighbors((x, y, z): (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    if x > 0 {
        result.push((x - 1, y, z));
    }
    if x + 1 < BOUND {
        result.push((x + 1, y, z));
    }
    if y > 0 {
        result.push((x, y - 1, z));
    }
    if y + 1 < BOUND {
        result.push((x, y + 1, z));
    }
    if z > 0 {
        result.push((x, y, z - 1));
    }
    if z + 1 < BOUND {
        result.push((x, y, z + 1));
    }
    result
}

fn fill(
    cubes: &HashSet<(usize, usize, usize)>,
    grid: &mut HashSet<(usize, usize, usize)>,
    cube: (usize, usize, usize),
) {
    grid.insert(cube);
    for next in neighbors(cube) {
        if !cubes.contains(&next) && !grid.contains(&next) {
            fill(cubes, grid, next);
        }
    }
}

pub fn more(cubes: &[(usize, usize, usize)]) -> usize {
    let cubes: HashSet<_> = cubes.iter().copied().collect();
    let mut grid = HashSet::new();
    fill(&cubes, &mut grid, (0, 0, 0));
    let mut cubes = vec![];
    for x in 0..BOUND {
        for y in 0..BOUND {
            for z in 0..BOUND {
                if !grid.contains(&(x, y, z)) {
                    cubes.push((x, y, z));
                }
            }
        }
    }
    solve(&cubes)
}
