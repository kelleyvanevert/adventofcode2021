use std::{collections::HashMap, str::FromStr};

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, derive_more::Add, derive_more::Sub, derive_more::From,
)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl FromStr for Pt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

fn count(s: &str, include_diagonals: bool) -> usize {
    let mut count = HashMap::new();

    for line in s.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        let mut a = a.parse::<Pt>().unwrap();
        let b = b.parse::<Pt>().unwrap();
        let delta = (b - a).signum();

        if include_diagonals || (delta.x + delta.y).abs() == 1 {
            count.entry(a).and_modify(|c| *c += 1).or_insert(1);

            loop {
                a = a + delta;
                count.entry(a).and_modify(|c| *c += 1).or_insert(1);

                if a == b {
                    break;
                }
            }
        }
    }

    count.iter().filter(|(_, &c)| c >= 2).count()
}

pub fn solve(s: &str) -> usize {
    count(s, false)
}

pub fn bonus(s: &str) -> usize {
    count(s, true)
}

#[test]
fn test_solve() {
    let s = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    assert_eq!(solve(s), 5);
    assert_eq!(bonus(s), 12);
}
