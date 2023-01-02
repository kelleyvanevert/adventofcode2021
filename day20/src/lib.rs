use std::ops::{Index, IndexMut};

type Algo = Vec<usize>;
type Pos = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<usize>>,
}

impl Image {
    fn from_pixels(pixels: Vec<Vec<usize>>) -> Self {
        Image {
            height: pixels.len(),
            width: pixels[0].len(),
            pixels,
        }
    }

    fn map(&self, f: impl Fn(usize, (usize, usize)) -> usize) -> Self {
        Self {
            width: self.width,
            height: self.height,
            pixels: self
                .pixels
                .iter()
                .enumerate()
                .map(|(y, row)| row.iter().enumerate().map(|(x, &b)| f(b, (x, y))).collect())
                .collect::<Vec<_>>(),
        }
    }

    fn viz(&self) {
        println!(
            "{}",
            self.pixels
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&b| if b == 1 { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn contract(&mut self, amount: usize) {
        for row in &mut self.pixels {
            for _ in 0..amount {
                row.remove(0);
                row.remove(row.len() - 1);
            }
        }

        for _ in 0..amount {
            self.pixels.remove(0);
            self.pixels.remove(self.pixels.len() - 1);
        }

        self.width -= 2 * amount;
        self.height -= 2 * amount;
    }

    fn expand(&mut self, amount: usize) {
        for _ in 0..amount {
            self.pixels.insert(0, vec![0; self.width]);
            self.pixels.push(vec![0; self.width]);
        }

        self.pixels = self
            .pixels
            .clone()
            .into_iter()
            .map(|mut row| {
                row.splice(0..0, vec![0; amount]);
                row.append(&mut vec![0; amount]);
                row
            })
            .collect();

        self.width += 2 * amount;
        self.height += 2 * amount;
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, (usize, usize))> {
        let mut neighbors = vec![];

        if y > 0 {
            if x > 0 {
                neighbors.push((256, (x - 1, y - 1)));
            }
            neighbors.push((128, (x, y - 1)));
            if x < self.width - 1 {
                neighbors.push((64, (x + 1, y - 1)));
            }
        }

        if x > 0 {
            neighbors.push((32, (x - 1, y)));
        }
        neighbors.push((16, (x, y)));
        if x < self.width - 1 {
            neighbors.push((8, (x + 1, y)));
        }

        if y < self.height - 1 {
            if x > 0 {
                neighbors.push((4, (x - 1, y + 1)));
            }
            neighbors.push((2, (x, y + 1)));
            if x < self.width - 1 {
                neighbors.push((1, (x + 1, y + 1)));
            }
        }

        neighbors
    }

    fn enhance(&self, algo: &Algo) -> Image {
        self.map(|_, (x, y)| {
            let num = self
                .neighbors(x, y)
                .into_iter()
                .map(|(k, n)| k * self[n])
                .sum::<usize>();

            algo[num]
        })
    }

    fn checksum(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum()
    }
}

impl Index<Pos> for Image {
    type Output = usize;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.pixels[y][x]
    }
}

impl IndexMut<Pos> for Image {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.pixels[y][x]
    }
}

pub fn solve(s: &str) -> usize {
    let (algo, im) = s.split_once("\n\n").unwrap();

    let algo = algo
        .trim()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Algo>();

    let mut image = Image::from_pixels(
        im.trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    println!();
    println!("Initial");
    image.viz();

    image.expand(10);

    println!();
    println!("First pass");
    image.enhance(&algo);
    image.viz();

    println!();
    println!("Second pass");
    image.enhance(&algo);
    image.contract(8);
    image.viz();

    image.checksum()
}

pub fn bonus(s: &str) -> usize {
    let (algo, im) = s.split_once("\n\n").unwrap();

    let algo = algo
        .trim()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Algo>();

    let mut image = Image::from_pixels(
        im.trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    for _ in 0..25 {
        image.expand(10);
        image = image.enhance(&algo);
        image = image.enhance(&algo);
        image.contract(8);
    }

    image.checksum()
}

#[test]
fn test_solve() {
    let s = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    assert_eq!(solve(s), 35);
    assert_eq!(bonus(s), 3351);
}
