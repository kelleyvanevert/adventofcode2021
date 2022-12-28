use std::collections::{HashMap, HashSet};

fn find_paths(s: &str, allow_double_visit: bool) -> usize {
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

    let mut todo = vec![(HashMap::new(), "start", false)];

    while let Some((visited, at, has_double_visit)) = todo.pop() {
        for &dest in &adj[at] {
            if dest == "end" {
                completed += 1;
            } else if !is_small[dest] || !visited.contains_key(&dest) {
                let mut visited = visited.clone();
                visited.insert(at, 1);
                todo.push((visited, dest, has_double_visit));
            } else if allow_double_visit
                && !has_double_visit
                && (dest != "start" && visited[dest] < 2)
            {
                let mut visited = visited.clone();
                visited.insert(at, 2);
                todo.push((visited, dest, true));
            }
        }
    }

    completed
}

pub fn solve(s: &str) -> usize {
    find_paths(s, false)
}

pub fn bonus(s: &str) -> usize {
    find_paths(s, true)
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
    assert_eq!(bonus(s), 36);

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
    assert_eq!(bonus(s), 103);

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
    assert_eq!(bonus(s), 3509);
}
