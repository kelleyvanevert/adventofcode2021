use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

type Pos = (usize, usize);

#[derive(Debug)]
struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn neighbors(&self, (x, y): Pos) -> Vec<Pos> {
        let mut neighbors = vec![];
        if y > 0 && x > 0 {
            neighbors.push((x - 1, y - 1));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y > 0 && x < self.width - 1 {
            neighbors.push((x + 1, y - 1));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y < self.height - 1 && x > 0 {
            neighbors.push((x - 1, y + 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        if y < self.height - 1 && x < self.width - 1 {
            neighbors.push((x + 1, y + 1));
        }
        neighbors
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

fn step(octos: &mut Grid<usize>) -> usize {
    let mut flashes = 0;

    let mut todo = (0..octos.height)
        .flat_map(|y| (0..octos.width).map(move |x| (x, y)))
        .collect::<Vec<_>>();

    while let Some(p) = todo.pop() {
        octos[p] += 1;

        if octos[p] == 10 {
            flashes += 1;
            for n in octos.neighbors(p) {
                todo.push(n);
            }
        }
    }

    for y in 0..octos.height {
        for x in 0..octos.width {
            let p = (x, y);
            if octos[p] > 9 {
                octos[p] = 0;
            }
        }
    }

    flashes
}

pub fn solve(s: &str) -> usize {
    let mut octos: Grid<usize> = s.parse().unwrap();

    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += step(&mut octos);
    }

    total_flashes
}

pub fn bonus(s: &str) -> usize {
    let mut octos: Grid<usize> = s.parse().unwrap();
    let n = octos.width * octos.height;

    (1..).find(|_| n == step(&mut octos)).unwrap()
}

#[test]
fn test_solve() {
    let s = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    assert_eq!(solve(s), 1656);
    assert_eq!(bonus(s), 195);
}
