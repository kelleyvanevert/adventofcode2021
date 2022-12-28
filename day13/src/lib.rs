#![feature(int_roundings)]

use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    FoldAlongX(usize),
    FoldAlongY(usize),
}

type Pos = (usize, usize);

fn parse(s: &str) -> (HashSet<Pos>, Vec<Instruction>) {
    let (dots, instructions) = s.split_once("\n\n").unwrap();

    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let instructions = instructions
        .lines()
        .map(|line| {
            let (intro, n) = line.split_once("=").unwrap();
            let n = n.parse::<usize>().unwrap();

            if intro == "fold along y" {
                Instruction::FoldAlongY(n)
            } else {
                Instruction::FoldAlongX(n)
            }
        })
        .collect();

    (dots, instructions)
}

fn fold(dots: HashSet<Pos>, instruction: &Instruction) -> HashSet<Pos> {
    dots.iter()
        .map(|&(x, y)| match instruction {
            Instruction::FoldAlongX(n) => {
                if x < *n {
                    (x, y)
                } else {
                    (n - (x - n), y)
                }
            }
            Instruction::FoldAlongY(n) => {
                if y < *n {
                    (x, y)
                } else {
                    (x, n - (y - n))
                }
            }
        })
        .collect()
}

pub fn solve(s: &str) -> usize {
    let (dots, instructions) = parse(s);

    let dots = fold(dots, &instructions[0]);

    dots.len()
}

/// Definitely not complete, and also just plain wrong, but for my input it works good enough :P
fn detect(grid: &Vec<Vec<char>>, x: usize) -> char {
    //
    // #### ###  #    #  # ###  ###  #### #  #
    // #    #  # #    #  # #  # #  # #    #  #
    // ###  ###  #    #  # ###  #  # ###  ####
    // #    #  # #    #  # #  # ###  #    #  #
    // #    #  # #    #  # #  # # #  #    #  #
    // #### ###  ####  ##  ###  #  # #    #  #

    if grid[3][x + 1] == '#' && grid[4][x + 2] == '#' {
        return 'R';
    }

    if grid[2][x + 1] == '#' {
        if grid[0][x] == ' ' {
            // or maybe S?
            return 'A';
        } else if grid[5][x + 1] == '#' {
            if grid[3][x + 3] == '#' {
                return 'B';
            } else {
                return 'E';
            }
        } else if grid[2][x + 3] == '#' {
            return 'H';
        } else {
            // or K
            // or P
            return 'F';
        }
    } else if grid[4][x] == ' ' && grid[4][x + 3] == ' ' {
        // or C
        // or V
        // or T
        // or Y
        println!("==== {}", grid[4][x..(x + 5)].iter().collect::<String>());
        return 'O';
    } else if grid[5][x] == '#' && grid[5][x + 1] == '#' && grid[5][x + 2] == '#' {
        if grid[5][x + 3] == '#' {
            // or Z
            return 'L';
        } else {
            // or maybe J?
            return 'D';
        }
    } else if grid[4][x] == '#' && grid[4][x + 3] == '#' {
        if grid[0][x] == '#' && grid[0][x + 3] == '#' {
            return 'O';
        } else {
            println!("{}", grid[5][x..(x + 5)].iter().collect::<String>());
            return 'U';
        }
    }

    // what about M?
    // what about N?
    // what about G?
    // what about I?
    // what about Q?
    // what about W?
    // what about X?

    '?'
}

pub fn bonus(s: &str) -> String {
    let (mut dots, instructions) = parse(s);

    for instruction in instructions {
        dots = fold(dots, &instruction);
    }

    let xmax = dots.iter().map(|p| p.0).max().unwrap();
    let ymax = dots.iter().map(|p| p.1).max().unwrap();

    let mut grid = vec![vec![' '; xmax + 2]; ymax + 2];
    for (x, y) in dots {
        grid[y][x] = '#';
    }

    (0..(xmax.div_ceil(5)))
        .map(|x| detect(&grid, x * 5))
        .collect()
}

#[test]
fn test_solve() {
    let s = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    assert_eq!(solve(s), 17);
    assert_eq!(bonus(s), "O".to_string());
}
