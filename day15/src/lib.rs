use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    ops::{Index, IndexMut},
    str::FromStr,
};

type Pos = (i32, i32);

fn manhattan((ax, ay): Pos, (bx, by): Pos) -> i32 {
    (bx - ax).abs() + (by - ay).abs()
}

struct Grid<T> {
    data: Vec<Vec<T>>,
    width: i32,
    height: i32,
}

impl<T> Grid<T> {
    fn new(width: i32, height: i32, mut f: impl FnMut(Pos) -> T) -> Grid<T> {
        Grid {
            width,
            height,
            data: (0..height)
                .map(|y| (0..width).map(|x| f((x, y))).collect())
                .collect(),
        }
    }

    fn neighbors(&self, (x, y): Pos) -> Vec<Pos> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < self.width as i32 - 1 {
            neighbors.push((x + 1, y));
        }
        if y < self.height as i32 - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn map<S>(&self, mut f: impl FnMut(&T) -> S) -> Grid<S> {
        Grid {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .map(|line| line.iter().map(|value| f(value)).collect())
                .collect(),
        }
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.data[y as usize][x as usize]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.data[y as usize][x as usize]
    }
}

impl FromStr for Grid<i32> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c as i32 - '0' as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = data.len() as i32;
        let width = data[0].len() as i32;

        Ok(Self {
            data,
            width,
            height,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    cost: i32,
    at: Pos,
    heuristic: i32,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.estimated_cost())
            .cmp(&self.estimated_cost())
            .then_with(|| self.at.cmp(&other.at))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {
    fn estimated_cost(&self) -> i32 {
        self.cost + self.heuristic
    }

    fn next(&self, grid: &Grid<i32>, end: Pos) -> Vec<Path> {
        grid.neighbors(self.at)
            .iter()
            .filter_map(|&n| {
                let child = Path {
                    at: n,
                    cost: self.cost + grid[n],
                    heuristic: manhattan(n, end),
                };

                Some(child)
            })
            .collect()
    }
}

fn search(grid: Grid<i32>) -> i32 {
    let start = (0, 0);
    let end = (grid.width - 1, grid.height - 1);

    let mut best = grid.map(|_| None);
    let mut queue = BinaryHeap::new();

    let first = Path {
        cost: 0,
        at: start,
        heuristic: manhattan(start, end),
    };

    best[start] = Some(0);
    queue.push(first);

    while let Some(path) = queue.pop() {
        if path.at == end {
            return path.cost;
        }

        for child in path.next(&grid, end) {
            match best[child.at] {
                Some(cost) if cost <= child.cost => {
                    // nothing new, skip
                }
                _ => {
                    best[child.at] = Some(child.cost);
                    queue.push(child);
                }
            }
        }
    }

    panic!("not found");
}

pub fn solve(s: &str) -> i32 {
    let grid: Grid<i32> = s.parse().unwrap();

    search(grid)
}

pub fn bonus(s: &str) -> i32 {
    let grid: Grid<i32> = s.parse().unwrap();

    let expanded_grid = Grid::new(grid.width * 5, grid.height * 5, |(x, y)| {
        let original_risk = grid[(x % grid.width, y % grid.height)];
        let increase = x / grid.width + y / grid.height;
        let risk = original_risk + increase;
        (risk - 1) % 9 + 1
    });

    search(expanded_grid)
}

#[test]
fn test_solve() {
    let s = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    assert_eq!(solve(s), 40);
    assert_eq!(bonus(s), 315);
}
