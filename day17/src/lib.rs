use tuple::Map;

type InclusiveBounds = ((i32, i32), (i32, i32));

type Pos = (i32, i32);
type Velocity = (i32, i32);

fn within((x, y): Pos, ((xmin, xmax), (ymin, ymax)): InclusiveBounds) -> bool {
    xmin <= x && x <= xmax && ymin <= y && y <= ymax
}

fn parse(s: &str) -> InclusiveBounds {
    let pieces = s.trim().split("=").collect::<Vec<&str>>();
    let y_range = pieces[2];
    let x_range = pieces[1].split_once(",").unwrap().0;

    (
        x_range
            .split_once("..")
            .unwrap()
            .map(|p| p.parse::<i32>().unwrap()),
        y_range
            .split_once("..")
            .unwrap()
            .map(|p| p.parse::<i32>().unwrap()),
    )
}

fn simulate_trajectory(mut v: Velocity, bounds: InclusiveBounds) -> Option<i32> {
    let mut pos: Pos = (0, 0);
    let mut max_y_reached = 0;

    loop {
        let new_pos = (pos.0 + v.0, pos.1 + v.1);
        let new_v = (v.0 - v.0.signum(), v.1 - 1);
        max_y_reached = max_y_reached.max(new_pos.1);

        if within(new_pos, bounds) {
            return Some(max_y_reached);
        }

        if new_pos.1 < bounds.1 .0 {
            return None;
        }

        v = new_v;
        pos = new_pos;
    }
}

pub fn solve(s: &str) -> i32 {
    let bounds = parse(s);

    let found = (1..(bounds.0 .1 + 1))
        .flat_map(|vx| {
            ((bounds.1 .0)..bounds.1 .0.abs())
                .filter_map(move |vy| simulate_trajectory((vx, vy), bounds))
        })
        .max()
        .unwrap();

    found
}

pub fn bonus(s: &str) -> usize {
    let bounds = parse(s);

    let num = (1..(bounds.0 .1 + 1))
        .flat_map(|vx| {
            ((bounds.1 .0)..bounds.1 .0.abs())
                .filter_map(move |vy| simulate_trajectory((vx, vy), bounds))
        })
        .count();

    num
}

#[test]
fn test_solve() {
    let s = "target area: x=20..30, y=-10..-5
";

    assert_eq!(solve(s), 45);
    assert_eq!(bonus(s), 112);
}
