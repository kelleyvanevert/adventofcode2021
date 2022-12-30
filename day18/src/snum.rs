use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::Add,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Snum {
    Reg(usize),
    Pair(Box<Snum>, Box<Snum>),
}

impl Snum {
    pub fn magnitude(&self) -> usize {
        match self {
            Snum::Reg(n) => *n,
            Snum::Pair(le, ri) => 3 * le.magnitude() + 2 * ri.magnitude(),
        }
    }

    fn add_rightmost(self: Snum, add: usize) -> Snum {
        match self {
            Snum::Reg(n) => Snum::Reg(n + add),
            Snum::Pair(box le, box ri) => Snum::Pair(Box::new(le), Box::new(ri.add_rightmost(add))),
        }
    }

    fn add_leftmost(self: Snum, add: usize) -> Snum {
        match self {
            Snum::Reg(n) => Snum::Reg(n + add),
            Snum::Pair(box le, box ri) => Snum::Pair(Box::new(le.add_leftmost(add)), Box::new(ri)),
        }
    }

    fn explode_at_depth(self: Snum, depth: usize) -> (Snum, Option<(usize, usize)>) {
        // println!("{}explode {} {}", vec![""; depth + 1].join("  "), depth, n);

        match self {
            Snum::Pair(box Snum::Reg(a), box Snum::Reg(b)) if depth >= 4 => {
                (Snum::Reg(0), Some((a, b)))
            }
            Snum::Pair(box le, box ri) => match le.explode_at_depth(depth + 1) {
                (le, Some((add_to_left, add_to_right))) => (
                    Snum::Pair(Box::new(le), Box::new(ri.add_leftmost(add_to_right))),
                    Some((add_to_left, 0)),
                ),
                (le, None) => match ri.explode_at_depth(depth + 1) {
                    (ri, Some((add_to_left, add_to_right))) => (
                        Snum::Pair(Box::new(le.add_rightmost(add_to_left)), Box::new(ri)),
                        Some((0, add_to_right)),
                    ),
                    (ri, None) => (Snum::Pair(Box::new(le), Box::new(ri)), None),
                },
            },
            n => (n, None),
        }
    }

    fn explode(self: Snum) -> (Snum, bool) {
        let (n, exploded) = self.explode_at_depth(0);
        (n, exploded.is_some())
    }

    fn split(self: Snum) -> (Snum, bool) {
        match self {
            Snum::Reg(k) if k < 10 => (Snum::Reg(k), false),
            Snum::Reg(k) => (
                Snum::Pair(
                    Box::new(Snum::Reg(k / 2)),
                    Box::new(Snum::Reg(k.div_ceil(2))),
                ),
                true,
            ),
            Snum::Pair(box le, box ri) => match le.split() {
                (le, true) => (Snum::Pair(Box::new(le), Box::new(ri)), true),
                (le, false) => {
                    let (ri, has_split) = ri.split();
                    (Snum::Pair(Box::new(le), Box::new(ri)), has_split)
                }
            },
        }
    }

    fn reduce(mut self: Snum) -> Snum {
        let mut cont = true;

        while cont {
            (self, cont) = self.explode();
            if cont {
                continue;
            }

            (self, cont) = self.split();
            if cont {
                continue;
            }
        }

        self
    }
}

impl Debug for Snum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Snum::Reg(n) => {
                write!(f, "{}", n)
            }
            Snum::Pair(box le, box ri) => {
                write!(f, "[{},{}]", le, ri)
            }
        }
    }
}

impl Display for Snum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Add for Snum {
    type Output = Snum;

    fn add(self, rhs: Self) -> Self::Output {
        Snum::Pair(Box::new(self), Box::new(rhs)).reduce()
    }
}

impl Sum for Snum {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut sum = iter
            .next()
            .expect("Summation of snailfish numbers is only defined on non-empty lists.");

        while let Some(next) = iter.next() {
            sum = sum + next;
        }

        sum
    }
}

#[test]
fn test_solve() {
    use crate::parse::parse_snum;

    let snum = parse_snum("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");

    assert_eq!(snum.magnitude(), 4140);

    assert_eq!(
        parse_snum("[[[[[9,8],1],2],3],4]").explode_at_depth(0),
        (parse_snum("[[[[0,9],2],3],4]"), Some((9, 0)))
    );

    assert_eq!(
        parse_snum("[7,[6,[5,[4,[3,2]]]]]").explode_at_depth(0),
        (parse_snum("[7,[6,[5,[7,0]]]]"), Some((0, 2)))
    );

    assert_eq!(
        parse_snum("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").explode_at_depth(0),
        (
            parse_snum("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            Some((0, 0)),
        ),
    );

    assert_eq!(
        parse_snum("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").explode_at_depth(0),
        (parse_snum("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), Some((0, 2)))
    );

    assert_eq!(
        parse_snum("[[[[4,3],4],4],[7,[[8,4],9]]]") + parse_snum("[1,1]"),
        parse_snum("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );

    assert_eq!(
        parse_snum("[[[[1,1],[2,2]],[3,3]],[4,4]]") + parse_snum("[5,5]"),
        parse_snum("[[[[3,0],[5,3]],[4,4]],[5,5]]")
    );

    assert_eq!(
        parse_snum("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")
            + parse_snum("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
        parse_snum("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]")
    );

    assert_eq!(
        vec![
            parse_snum("[1,1]"),
            parse_snum("[2,2]"),
            parse_snum("[3,3]"),
            parse_snum("[4,4]"),
        ]
        .into_iter()
        .sum::<Snum>(),
        parse_snum("[[[[1,1],[2,2]],[3,3]],[4,4]]")
    );

    assert_eq!(
        vec![
            parse_snum("[1,1]"),
            parse_snum("[2,2]"),
            parse_snum("[3,3]"),
            parse_snum("[4,4]"),
            parse_snum("[5,5]"),
        ]
        .into_iter()
        .sum::<Snum>(),
        parse_snum("[[[[3,0],[5,3]],[4,4]],[5,5]]")
    );
}
