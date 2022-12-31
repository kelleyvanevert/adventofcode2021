use day19::*;
use util::*;

fn main() {
    let s = include_str!("../input.txt");

    time(|| {
        let (solution, bonus) = solve_both_parts(s);
        println!("Solution: {solution}");
        println!("Bonus: {bonus}");
    });
}
