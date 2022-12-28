fn find(s: &str, dist: impl Fn(i32, i32) -> i32) -> i32 {
    let nums = s
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    (min..=max)
        .map(|c| nums.iter().map(|&n| dist(c, n)).sum())
        .min()
        .unwrap()
}

pub fn solve(s: &str) -> i32 {
    find(s, |a, b| (a - b).abs())
}

pub fn bonus(s: &str) -> i32 {
    find(s, |a, b| {
        let d = (a - b).abs();
        (d * (d + 1)) / 2
    })
}

#[test]
fn test_solve() {
    let s = "7,1,16,1,4,2,2,2,0,14
";

    assert_eq!(solve(s), 37);
    assert_eq!(bonus(s), 168);

    let s = include_str!("../input.txt");

    assert_eq!(solve(s), 348664);
    assert_eq!(bonus(s), 100220525);
}
