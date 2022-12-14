use std::collections::VecDeque;

fn neighbors(grid: &Vec<Vec<usize>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = vec![];
    if 0 < x && grid[x - 1][y] <= grid[x][y] + 1 {
        v.push((x - 1, y));
    }
    if x + 1 < grid.len() && grid[x + 1][y] <= grid[x][y] + 1 {
        v.push((x + 1, y));
    }
    if 0 < y && grid[x][y - 1] <= grid[x][y] + 1 {
        v.push((x, y - 1));
    }
    if y + 1 < grid[0].len() && grid[x][y + 1] <= grid[x][y] + 1 {
        v.push((x, y + 1));
    }
    v
}

fn neighbors2(grid: &Vec<Vec<usize>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = vec![];
    if 0 < x && grid[x - 1][y] + 1 >= grid[x][y] {
        v.push((x - 1, y));
    }
    if x + 1 < grid.len() && grid[x + 1][y] + 1 >= grid[x][y] {
        v.push((x + 1, y));
    }
    if 0 < y && grid[x][y - 1] + 1 >= grid[x][y] {
        v.push((x, y - 1));
    }
    if y + 1 < grid[0].len() && grid[x][y + 1] + 1 >= grid[x][y] {
        v.push((x, y + 1));
    }
    v
}

pub fn solve(grid: &Vec<Vec<usize>>, s: (usize, usize), e: (usize, usize)) -> usize {
    let (n, m) = (grid.len(), grid[0].len());
    let mut q = VecDeque::new();
    q.push_back((0, s.0, s.1));
    let mut done = vec![vec![false; m]; n];
    let mut dist = vec![vec![0; m]; n];
    while let Some((d, x, y)) = q.pop_front() {
        if !done[x][y] {
            done[x][y] = true;
            dist[x][y] = d;
            for (xx, yy) in neighbors(grid, (x, y)) {
                q.push_back((d + 1, xx, yy));
            }
        }
    }
    dist[e.0][e.1]
}

pub fn more(grid: &Vec<Vec<usize>>, e: (usize, usize)) -> usize {
    let (n, m) = (grid.len(), grid[0].len());
    let mut q = VecDeque::new();
    q.push_back((0, e.0, e.1));
    let mut done = vec![vec![false; m]; n];
    let mut dist = vec![vec![0; m]; n];
    while let Some((d, x, y)) = q.pop_front() {
        if !done[x][y] {
            done[x][y] = true;
            dist[x][y] = d;
            for (xx, yy) in neighbors2(grid, (x, y)) {
                q.push_back((d + 1, xx, yy));
            }
        }
    }
    (0..n)
        .flat_map(|i| (0..m).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 0 && done[i][j])
        .map(|(i, j)| dist[i][j])
        .min()
        .unwrap()
}
