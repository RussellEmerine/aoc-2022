pub fn count_visible(trees: Vec<Vec<usize>>) -> usize {
    let (n, m) = (trees.len(), trees[0].len());
    let mut visible = vec![vec![false; m]; n];

    for i in 0..n {
        visible[i][0] = true;
        let mut max = trees[i][0];
        for j in 1..m {
            if trees[i][j] > max {
                visible[i][j] = true;
                max = trees[i][j];
            }
        }

        visible[i][m - 1] = true;
        let mut max = trees[i][m - 1];
        for j in (0..m - 1).rev() {
            if trees[i][j] > max {
                visible[i][j] = true;
                max = trees[i][j];
            }
        }
    }

    for j in 0..m {
        visible[0][j] = true;
        let mut max = trees[0][j];
        for i in 1..n {
            if trees[i][j] > max {
                visible[i][j] = true;
                max = trees[i][j];
            }
        }

        visible[n - 1][j] = true;
        let mut max = trees[n - 1][j];
        for i in (0..n - 1).rev() {
            if trees[i][j] > max {
                visible[i][j] = true;
                max = trees[i][j];
            }
        }
    }

    visible.iter().flatten().filter(|&&a| a).count()
}

pub fn score(v: Vec<Vec<usize>>) -> usize {
    let (n, m) = (v.len(), v[0].len());
    let mut a = vec![vec![0; m]; n];
    for i in 0..n {
        let mut last = [0; 10];
        for j in 0..m {
            a[i][j] = j - last[v[i][j]..].iter().max().unwrap();
            last[v[i][j]] = j;
        }
    }

    let mut b = vec![vec![0; m]; n];
    for i in 0..n {
        let mut last = [m - 1; 10];
        for j in (0..m).rev() {
            b[i][j] = last[v[i][j]..].iter().min().unwrap() - j;
            last[v[i][j]] = j;
        }
    }

    let mut c = vec![vec![0; m]; n];
    for j in 0..m {
        let mut last = [0; 10];
        for i in 0..n {
            c[i][j] = i - last[v[i][j]..].iter().max().unwrap();
            last[v[i][j]] = i;
        }
    }

    let mut d = vec![vec![0; m]; n];
    for j in 0..m {
        let mut last = [n - 1; 10];
        for i in (0..n).rev() {
            d[i][j] = last[v[i][j]..].iter().min().unwrap() - i;
            last[v[i][j]] = i;
        }
    }

    (0..n)
        .map(|i| (0..m).map(move |j| (i, j)))
        .flatten()
        .map(|(i, j)| a[i][j] * b[i][j] * c[i][j] * d[i][j])
        .max()
        .unwrap()
}
