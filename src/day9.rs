use std::collections::HashSet;

fn follow(tx: &mut isize, ty: &mut isize, hx: isize, hy: isize) {
    if (*tx - hx).abs() <= 1 && (*ty - hy).abs() <= 1 {
        // do nothing
    } else if *tx == hx {
        if *ty + 1 < hy {
            *ty += 1;
        } else if *ty - 1 > hy {
            *ty -= 1;
        }
    } else if *ty == hy {
        if *tx + 1 < hx {
            *tx += 1;
        } else if *tx - 1 > hx {
            *tx -= 1;
        }
    } else if *tx < hx && *ty < hy {
        *tx += 1;
        *ty += 1;
    } else if *tx < hx && *ty > hy {
        *tx += 1;
        *ty -= 1;
    } else if *tx > hx && *ty < hy {
        *tx -= 1;
        *ty += 1;
    } else {
        *tx -= 1;
        *ty -= 1;
    }
}

pub fn solve(moves: Vec<(char, usize)>) -> usize {
    let mut tail_pos = HashSet::new();
    let (mut hx, mut hy): (isize, isize) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    tail_pos.insert((tx, ty));
    for (c, m) in moves {
        for _ in 0..m {
            match c {
                'U' => hy += 1,
                'R' => hx += 1,
                'D' => hy -= 1,
                'L' => hx -= 1,
                _ => panic!("lmao"),
            }
            follow(&mut tx, &mut ty, hx, hy);
            tail_pos.insert((tx, ty));
        }
    }
    tail_pos.len()
}

pub fn more(moves: Vec<(char, usize)>) -> usize {
    let mut tail_pos = HashSet::new();
    let mut xs = [0; 10];
    let mut ys = [0; 10];
    tail_pos.insert((xs[9], ys[9]));
    for (c, m) in moves {
        for _ in 0..m {
            match c {
                'U' => ys[0] += 1,
                'R' => xs[0] += 1,
                'D' => ys[0] -= 1,
                'L' => xs[0] -= 1,
                _ => panic!("lmao"),
            }
            for i in 1..10 {
                let (hx, hy) = (xs[i - 1], ys[i - 1]);
                follow(&mut xs[i], &mut ys[i], hx, hy);
            }
            tail_pos.insert((xs[9], ys[9]));
        }
    }
    tail_pos.len()
}
