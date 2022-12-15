const X: usize = 1000;
const Y: usize = 200;

pub fn solve(lines: Vec<Vec<(usize, usize)>>) -> usize {
    let mut grid = [[false; Y]; X];
    for line in lines {
        for v in line.windows(2) {
            let ((a, b), (c, d)) = (v[0], v[1]);
            if a == c {
                for j in b.min(d)..=b.max(d) {
                    grid[a][j] = true;
                }
            } else {
                for i in a.min(c)..=a.max(c) {
                    grid[i][b] = true;
                }
            }
        }
    }

    let mut counter = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        while y + 1 < Y {
            if !grid[x][y + 1] {
                y += 1;
            } else if !grid[x - 1][y + 1] {
                x -= 1;
                y += 1;
            } else if !grid[x + 1][y + 1] {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }
        if y + 1 < Y {
            grid[x][y] = true;
            counter += 1;
        } else {
            return counter;
        }
    }
}

pub fn more(lines: Vec<Vec<(usize, usize)>>) -> usize {
    let ymax = lines.iter().flatten().map(|&(_, y)| y).max().unwrap() + 2;
    let mut grid = vec![vec![false; ymax]; X];
    for line in lines {
        for v in line.windows(2) {
            let ((a, b), (c, d)) = (v[0], v[1]);
            if a == c {
                for j in b.min(d)..=b.max(d) {
                    grid[a][j] = true;
                }
            } else {
                for i in a.min(c)..=a.max(c) {
                    grid[i][b] = true;
                }
            }
        }
    }

    let mut counter = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        while y + 1 < ymax {
            if !grid[x][y + 1] {
                y += 1;
            } else if !grid[x - 1][y + 1] {
                x -= 1;
                y += 1;
            } else if !grid[x + 1][y + 1] {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }
        if y != 0 {
            grid[x][y] = true;
            counter += 1;
        } else {
            return counter + 1;
        }
    }
}
