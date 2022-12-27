pub fn solve(s: &str) -> usize {
    let mut depth = 0;
    let mut pos = 0;

    for line in s.lines() {
        if line.starts_with("forward") {
            pos += line[8..].parse::<usize>().unwrap();
        } else if line.starts_with("up") {
            depth -= line[3..].parse::<usize>().unwrap();
        } else if line.starts_with("down") {
            depth += line[5..].parse::<usize>().unwrap();
        }
    }

    depth * pos
}

pub fn bonus(s: &str) -> usize {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for line in s.lines() {
        if line.starts_with("forward") {
            let x = line[8..].parse::<usize>().unwrap();
            pos += x;
            depth += x * aim;
        } else if line.starts_with("up") {
            aim -= line[3..].parse::<usize>().unwrap();
        } else if line.starts_with("down") {
            aim += line[5..].parse::<usize>().unwrap();
        }
    }

    depth * pos
}

#[test]
fn test_solve() {
    let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    assert_eq!(solve(s), 150);
    assert_eq!(bonus(s), 900);
}
