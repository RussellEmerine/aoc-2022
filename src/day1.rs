pub(crate) fn solve(calories: Vec<Vec<usize>>) -> usize {
    calories
        .iter()
        .map(|v| v.iter().sum::<usize>())
        .max()
        .expect("called solve with empty list")
}

pub(crate) fn more(calories: Vec<Vec<usize>>) -> usize {
    let mut sums: Vec<usize> = calories.iter().map(|v| v.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    sums[0] + sums[1] + sums[2]
}
