#![feature(drain_filter)]

type BoardState = Vec<Option<usize>>;

fn place_num(board: &mut BoardState, num: usize) -> Option<usize> {
    for opt in board.iter_mut() {
        match opt {
            Some(n) if *n == num => {
                *opt = None;
            }
            _ => (),
        }
    }

    for i in 0..5 {
        if (0..5).all(|j| board[i * 5 + j].is_none()) || (0..5).all(|j| board[j * 5 + i].is_none())
        {
            return Some(num * board.iter().map(|opt| opt.unwrap_or(0)).sum::<usize>());
        }
    }

    None
}

fn parse(s: &str) -> (Vec<usize>, Vec<BoardState>) {
    let pieces = s.split("\n\n").collect::<Vec<_>>();

    let numbers = pieces[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let boards: Vec<BoardState> = pieces[1..]
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| Some(s.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (numbers, boards)
}

pub fn solve(s: &str) -> usize {
    let (numbers, mut boards) = parse(s);

    loop {
        for &num in &numbers {
            for board in &mut boards {
                if let Some(winning_score) = place_num(board, num) {
                    return winning_score;
                }
            }
        }
    }
}

pub fn bonus(s: &str) -> usize {
    let (numbers, mut boards) = parse(s);

    let mut last_winning_score = 0;

    loop {
        for &num in &numbers {
            boards.drain_filter(|board| {
                if let Some(winning_score) = place_num(board, num) {
                    last_winning_score = winning_score;
                    return true;
                }

                return false;
            });

            if boards.len() == 0 {
                return last_winning_score;
            }
        }
    }
}

#[test]
fn test_solve() {
    let s = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    assert_eq!(solve(s), 4512);
    assert_eq!(bonus(s), 1924)
}
