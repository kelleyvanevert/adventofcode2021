use std::{
    cmp::Reverse,
    collections::HashSet,
    ops::{Index, IndexMut},
    str::FromStr,
};

type Pos = (usize, usize);

struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn neighbors(&self, (x, y): Pos) -> Vec<Pos> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y < self.height - 1 {
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
        &self.data[y][x]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.data[y][x]
    }
}

impl FromStr for Grid<usize> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c as usize - '0' as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = data.len();
        let width = data[0].len();

        Ok(Self {
            data,
            width,
            height,
        })
    }
}

pub fn solve(s: &str) -> usize {
    let heightmap: Grid<usize> = s.parse().unwrap();

    let mut risk = 0;

    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let p = (x, y);

            if heightmap
                .neighbors(p)
                .iter()
                .all(|&n| heightmap[p] < heightmap[n])
            {
                risk += heightmap[p] + 1;
            }
        }
    }

    risk
}

pub fn bonus(s: &str) -> usize {
    let heightmap: Grid<usize> = s.parse().unwrap();

    let mut mapped = heightmap.map(|&height| height == 9);
    let mut basin_sizes: Vec<Reverse<usize>> = vec![];

    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let p = (x, y);

            if !mapped[p] {
                // discover new basin greedily
                let mut basin = HashSet::new();
                let mut queue = vec![p];
                while let Some(p) = queue.pop() {
                    if basin.insert(p) {
                        mapped[p] = true;
                        for n in heightmap.neighbors(p) {
                            if !mapped[n] {
                                queue.push(n);
                            }
                        }
                    }
                }

                basin_sizes.push(Reverse(basin.len()));
                basin_sizes.sort();
                if basin_sizes.len() > 3 {
                    basin_sizes.remove(3);
                }
            }
        }
    }

    basin_sizes.iter().fold(1, |m, a| m * a.0)
}

#[test]
fn test_solve() {
    let s = "2199943210
3987894921
9856789892
8767896789
9899965678";

    assert_eq!(solve(s), 15);
    assert_eq!(bonus(s), 1134);
}
