pub fn solve(s: &str) -> usize {
    let mut pos = s
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1usize)
        .collect::<Vec<_>>();

    let mut score = vec![0, 0];

    let mut rolls = 0;

    while score[0] < 1000 && score[1] < 1000 {
        let i = (rolls / 3) % 2;
        let move_by = rolls + 1 + rolls + 2 + rolls + 3;
        pos[i] = (pos[i] + move_by) % 10;
        score[i] += pos[i] + 1;

        rolls += 3;
    }

    rolls * score[0].min(score[1])
}

pub fn bonus(s: &str) -> usize {
    42
}

#[test]
fn test_solve() {
    let s = "Player 1 starting position: 4
Player 2 starting position: 8
";

    assert_eq!(solve(s), 739785);
}
