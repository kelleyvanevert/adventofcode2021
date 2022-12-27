use itertools::Itertools;

pub fn solve(s: &str) -> usize {
    s.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn bonus(s: &str) -> usize {
    s.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}
