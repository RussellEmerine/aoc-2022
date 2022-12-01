pub(crate) fn solve(calories: Vec<Vec<usize>>) -> usize {
    calories
        .iter()
        .map(|v| v.iter().sum::<usize>())
        .max()
        .expect("called solve with empty list")
}
