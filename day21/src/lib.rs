use std::{
    iter::Sum,
    ops::{Add, Mul},
};

pub fn solve(s: &str) -> usize {
    let mut pos = s
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1usize)
        .collect::<Vec<_>>();

    let mut score = vec![0, 0];

    let mut rolls = 0;

    while score[0] < 1000 && score[1] < 1000 {
        let turn = (rolls / 3) % 2;
        let move_by = rolls + 1 + rolls + 2 + rolls + 3;
        pos[turn] = (pos[turn] + move_by) % 10;
        score[turn] += pos[turn] + 1;

        rolls += 3;
    }

    rolls * score[0].min(score[1])
}

const DICE_ROLLS: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

/// nr of steps -> in how many universes you can reach 21 steps this way
#[derive(Debug, Clone)]
struct NumberOfSteps(Vec<u64>);

impl NumberOfSteps {
    fn zero() -> Self {
        NumberOfSteps(vec![])
    }

    fn shift(mut self) -> Self {
        self.0.insert(0, 0);
        self
    }
}

impl Mul<u64> for NumberOfSteps {
    type Output = NumberOfSteps;

    fn mul(self, m: u64) -> Self::Output {
        NumberOfSteps(self.0.into_iter().map(|k| m * k).collect())
    }
}

impl Add<NumberOfSteps> for NumberOfSteps {
    type Output = NumberOfSteps;

    fn add(self, rhs: Self) -> Self::Output {
        let n = self.0.len().max(rhs.0.len());
        NumberOfSteps(
            (0..n)
                .map(|i| self.0.get(i).unwrap_or(&0) + rhs.0.get(i).unwrap_or(&0))
                .collect(),
        )
    }
}

impl Sum for NumberOfSteps {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(NumberOfSteps::zero(), NumberOfSteps::add)
    }
}

fn numsteps_both(turn: usize, pos: [u64; 2], score: [u64; 2]) -> [NumberOfSteps; 2] {
    if score[0] >= 21 {
        return [NumberOfSteps(vec![0, 1]), NumberOfSteps::zero()];
    } else if score[1] >= 21 {
        return [NumberOfSteps::zero(), NumberOfSteps(vec![0, 1])];
    }

    DICE_ROLLS
        .iter()
        .map(|&(steps, num_universes)| {
            let target_pos = (pos[turn] + steps - 1) % 10 + 1;

            let mut new_pos = pos;
            new_pos[turn] = target_pos;

            let mut new_score = score;
            new_score[turn] += target_pos;

            let steps = numsteps_both(1 - turn, new_pos, new_score);

            steps.map(|s| s.shift() * num_universes)
        })
        .fold(
            [NumberOfSteps::zero(), NumberOfSteps::zero()],
            |[accum0, accum1], [n0, n1]| [accum0 + n0, accum1 + n1],
        )
}

pub fn bonus(s: &str) -> u64 {
    let pos = s
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let res = numsteps_both(0, [pos[0], pos[1]], [0, 0]);

    let sums = res.map(|ns| ns.0.iter().sum::<u64>());

    sums[0].max(sums[1])
}

#[test]
fn test_solve() {
    let s = "Player 1 starting position: 4
Player 2 starting position: 8
";

    assert_eq!(solve(s), 739785);
    assert_eq!(bonus(s), 444356092776315);
}
