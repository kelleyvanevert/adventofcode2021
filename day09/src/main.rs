use day09::*;
use util::*;

fn main() {
    let s = include_str!("../input.txt");

    time(|| {
        println!("Solution: {}", solve(s));
        println!("Bonus: {}", bonus(s));
    });
}
