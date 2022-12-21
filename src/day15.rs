use std::collections::BTreeSet;

pub fn solve(v: Vec<(i64, i64, i64, i64)>, y: i64) -> usize {
    let mut beacons = BTreeSet::new();
    for &(_, _, x2, y2) in &v {
        if y2 == y {
            beacons.insert(x2);
        }
    }
    let mut bounds: Vec<_> = v
        .iter()
        .flat_map(|&(x1, y1, x2, y2)| {
            let d = (x1 - x2).abs() + (y1 - y2).abs();
            let yd = (y - y1).abs();
            if d >= yd {
                vec![(x1 - (d - yd), false), (x1 + (d - yd), true)]
            } else {
                vec![]
            }
        })
        .collect();
    bounds.sort();
    let mut d = 0;
    let mut switches = vec![];
    for (x, end) in bounds {
        if end {
            d -= 1;
            if d == 0 {
                switches.push(x);
            }
        } else {
            if d == 0 {
                switches.push(x);
            }
            d += 1;
        }
    }
    switches
        .chunks_exact(2)
        .map(|a| (a[1] - a[0] + 1) as usize - beacons.range(a[0]..=a[1]).count())
        .sum()
}

#[allow(dead_code)]
pub fn more(v: Vec<(i64, i64, i64, i64)>, bound: i64) -> i64 {
    for y in 0..=bound {
        let mut bounds: Vec<_> = v
            .iter()
            .flat_map(|&(x1, y1, x2, y2)| {
                let d = (x1 - x2).abs() + (y1 - y2).abs();
                let yd = (y - y1).abs();
                if d >= yd {
                    vec![(x1 - (d - yd), false), (x1 + (d - yd), true)]
                } else {
                    vec![]
                }
            })
            .collect();
        bounds.push((0, true));
        bounds.push((bound, false));
        bounds.sort();
        let mut d = 1;
        let mut switches = vec![];
        for (x, end) in bounds {
            if end {
                d -= 1;
                if d == 0 {
                    switches.push(x);
                }
            } else {
                if d == 0 {
                    switches.push(x);
                }
                d += 1;
            }
        }
        for a in switches.chunks_exact(2) {
            if a[0] + 1 < a[1] {
                return (a[0] + 1) * 4000000 + y;
            }
        }
    }
    unreachable!("oops")
}
