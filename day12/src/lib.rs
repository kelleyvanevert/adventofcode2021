use std::collections::{HashMap, HashSet};

pub fn solve(s: &str) -> usize {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut is_small = HashMap::new();

    for line in s.lines() {
        let (a, b) = line.split_once("-").unwrap();

        is_small.insert(a, a.chars().all(|c| c.is_ascii_lowercase()));
        is_small.insert(b, b.chars().all(|c| c.is_ascii_lowercase()));

        adj.entry(a)
            .and_modify(|dsts| {
                dsts.insert(b);
            })
            .or_insert_with(|| HashSet::from([b]));

        adj.entry(b)
            .and_modify(|dsts| {
                dsts.insert(a);
            })
            .or_insert_with(|| HashSet::from([a]));
    }

    let mut completed = 0;

    let mut todo: Vec<(Vec<&str>, &str, usize)> = vec![(vec![], "start", 0)];

    while let Some((visited, at, depth)) = todo.pop() {
        for &dest in &adj[at] {
            if dest == "end" {
                completed += 1;
            } else if !is_small[dest] || !visited.contains(&dest) {
                let mut visited = visited.clone();
                visited.push(at);
                todo.push((visited, dest, depth + 1));
            }
        }
    }

    completed
}

pub fn bonus(s: &str) -> usize {
    42
}

#[test]
fn test_solve() {
    let s = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    assert_eq!(solve(s), 10);

    let s = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    assert_eq!(solve(s), 19);

    let s = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    assert_eq!(solve(s), 226);
}
