fn score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn line_score(line: &str) -> (usize, Vec<char>) {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            c => {
                if stack.pop() != Some(c) {
                    return (score(c), stack);
                }
            }
        }
    }

    (0, stack)
}

pub fn solve(s: &str) -> usize {
    s.lines().map(line_score).map(|p| p.0).sum()
}

fn autocomplete_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

pub fn bonus(s: &str) -> usize {
    let mut autocomplete_scores = s
        .lines()
        .map(line_score)
        .filter_map(|(score, mut leftover)| {
            if score == 0 && leftover.len() > 0 {
                leftover.reverse();
                Some(leftover)
            } else {
                None
            }
        })
        .map(|leftover| {
            let mut score = 0;
            for c in leftover {
                score = (score * 5) + autocomplete_score(c);
            }
            score
        })
        .collect::<Vec<_>>();

    autocomplete_scores.sort();

    autocomplete_scores[autocomplete_scores.len() / 2]
}

#[test]
fn test_solve() {
    let s = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    assert_eq!(solve(s), 26397);
    assert_eq!(bonus(s), 288957);
}
