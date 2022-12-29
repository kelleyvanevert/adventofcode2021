use itertools::Itertools;
use std::collections::HashMap;

type Polymer = Vec<char>;
type Rules = HashMap<(char, char), char>;

fn parse(s: &str) -> (Polymer, Rules) {
    let (template, rules) = s.split_once("\n\n").unwrap();
    let template = template.chars().collect::<Vec<_>>();

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

fn step(polymer: Polymer, rules: &Rules) -> Polymer {
    let mut last = '?';

    let mut new_polymer = polymer
        .iter()
        .tuple_windows()
        .flat_map(|(&a, &b)| {
            last = b;
            if let Some(&c) = rules.get(&(a, b)) {
                vec![a, c]
            } else {
                vec![a]
            }
        })
        .collect_vec();

    new_polymer.push(last);
    new_polymer
}

fn run(s: &str, steps: usize) -> usize {
    let (template, rules) = parse(s);

    let mut polymer = template;
    for i in 0..steps {
        println!("{i}");
        polymer = step(polymer, &rules);
    }

    let histo = polymer.into_iter().counts();
    let mut histo = histo.iter().collect_vec();
    histo.sort_by_key(|t| t.1);

    histo.last().unwrap().1 - histo.first().unwrap().1
}

pub fn solve(s: &str) -> usize {
    run(s, 10)
}

pub fn bonus(s: &str) -> usize {
    run(s, 40)
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

    assert_eq!(solve(s), 1588);
    // assert_eq!(bonus(s), 2188189693529);
}
