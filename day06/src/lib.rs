fn sim(s: &str, steps: u64) -> u64 {
    let mut fish = vec![0; 9];

    for k in s.trim().split(",").map(|s| s.parse::<usize>().unwrap()) {
        fish[k] += 1;
    }

    for _ in 0..steps {
        let n = fish.remove(0);
        fish[6] += n;
        fish.push(n);
    }

    fish.iter().sum()
}

pub fn solve(s: &str) -> u64 {
    sim(s, 80)
}

pub fn bonus(s: &str) -> u64 {
    sim(s, 256)
}

#[test]
fn test_solve() {
    let s = "3,4,3,1,2
";

    assert_eq!(solve(s), 5934);
    assert_eq!(bonus(s), 26984457539);
}
