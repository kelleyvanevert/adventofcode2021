use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, iter::Sum};

type Polymer = Vec<(char, char)>;
type Rules = HashMap<(char, char), char>;

#[derive(Debug, Clone)]
struct Histo(HashMap<char, u64>);

impl Histo {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn from<const N: usize>(arr: [(char, u64); N]) -> Self {
        Self(HashMap::from(arr))
    }

    fn add(&mut self, c: char, num: u64) {
        self.0.entry(c).and_modify(|n| *n += num).or_insert(num);
    }
}

impl Sum for Histo {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Histo::new();

        for item in iter {
            sum = add_histos(sum, item);
        }

        sum
    }
}

impl IntoIterator for Histo {
    type Item = (char, u64);
    type IntoIter = std::collections::hash_map::IntoIter<char, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn add_histos(mut x: Histo, y: Histo) -> Histo {
    for (c, num) in y {
        x.add(c, num);
    }

    x
}

fn parse(s: &str) -> (Polymer, Rules) {
    let (template, rules) = s.split_once("\n\n").unwrap();
    let template = template
        .chars()
        .tuple_windows::<(char, char)>()
        .collect::<Vec<_>>();

    let rules = rules
        .trim()
        .lines()
        .map(|line| {
            let (pair, new) = line.split_once(" -> ").unwrap();
            let pair = pair.chars().collect::<Vec<_>>();
            let pair = (pair[0], pair[1]);
            let new = new.chars().nth(0).unwrap();
            (pair, new)
        })
        .collect();

    (template, rules)
}

std::thread_local! {
  static CALC_MEMO: RefCell<HashMap<(char, char, usize), Histo>> = RefCell::new(HashMap::new());
}

fn calc(le: char, ri: char, steps: usize, rules: &Rules) -> Histo {
    if steps == 0 {
        return Histo::from([(le, 1)]);
    } else if let Some(&insert) = rules.get(&(le, ri)) {
        return add_histos(
            calc_memoized(le, insert, steps - 1, rules),
            calc_memoized(insert, ri, steps - 1, rules),
        );
    }

    Histo::from([(le, 1)])
}

/// Basically just copied what `#[memoize]` generates, except omitting `rules`. Technically incorrect for that reason, if other `rules` would ever be used.
fn calc_memoized(le: char, ri: char, steps: usize, rules: &Rules) -> Histo {
    let r = CALC_MEMO.with(|cell| {
        let memo = cell.borrow_mut();
        memo.get(&(le, ri, steps)).cloned()
    });

    if let Some(r) = r {
        return r;
    }

    let r = calc(le, ri, steps, rules);

    CALC_MEMO.with(|cell| {
        let mut memo = cell.borrow_mut();
        memo.insert((le, ri, steps), r.clone());
    });

    r
}

fn run(template: Polymer, rules: Rules, steps: usize) -> Histo {
    CALC_MEMO.with(|cell| {
        cell.borrow_mut().clear();
    });

    let mut histo = template
        .iter()
        .map(|&(le, ri)| calc_memoized(le, ri, steps, &rules))
        .sum::<Histo>();

    let last_char = template.last().unwrap().1;
    histo.add(last_char, 1);

    histo
}

fn score(histo: Histo) -> u64 {
    let mut histo = histo.into_iter().collect_vec();
    histo.sort_by_key(|t| t.1);

    histo.last().unwrap().1 - histo.first().unwrap().1
}

pub fn solve(s: &str) -> u64 {
    let (template, rules) = parse(s);

    let histo = run(template, rules, 10);

    score(histo)
}

pub fn bonus(s: &str) -> u64 {
    let (template, rules) = parse(s);

    let histo = run(template, rules, 40);

    score(histo)
}

#[test]
fn test_solve() {
    let s = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    let actual = include_str!("../input.txt");

    assert_eq!(solve(s), 1588);
    assert_eq!(solve(actual), 2703);
    assert_eq!(bonus(s), 2188189693529);
    assert_eq!(bonus(actual), 2984946368465);
}
