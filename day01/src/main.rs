use day01::*;
use util::*;

fn main() {
    let s = include_str!("../input.txt");

    time(|| {
        println!("Num larger: {}", solve(s));
        println!("Num larger v2: {}", bonus(s));
    });
}
