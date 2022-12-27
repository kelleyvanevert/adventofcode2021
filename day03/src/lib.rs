pub fn solve(s: &str) -> usize {
    let lines = s.lines().collect::<Vec<&str>>();
    let n = lines.len();
    let w = lines[0].len();
    let mut count = vec![0; w];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            count[i] += if c == '1' { 1 } else { 0 };
        }
    }

    let gamma_rate: usize = count
        .iter()
        .enumerate()
        .map(|(i, &count)| if count > n / 2 { 1 << (w - i - 1) } else { 0 })
        .sum();

    let epsilon_rate: usize = count
        .iter()
        .enumerate()
        .map(|(i, &count)| if count < n / 2 { 1 << (w - i - 1) } else { 0 })
        .sum();

    gamma_rate * epsilon_rate
}

pub fn bonus(s: &str) -> usize {
    let to_bit = |c: char| if c == '1' { 1 } else { 0 };

    let bits: Vec<Vec<usize>> = s
        .lines()
        .map(|line| line.chars().map(to_bit).collect())
        .collect();

    let w = bits[0].len();

    let select = |mut bits: Vec<Vec<usize>>, oxygen: bool| {
        let mut at_bit = 0;

        while bits.len() > 1 {
            let count_1 = bits.iter().filter(|line| line[at_bit] == 1).count();
            let count_0 = bits.len() - count_1;

            let choose = if oxygen {
                if count_1 as f32 >= bits.len() as f32 / 2.0 {
                    1
                } else {
                    0
                }
            } else {
                if count_0 as f32 <= bits.len() as f32 / 2.0 {
                    0
                } else {
                    1
                }
            };

            bits = bits
                .clone()
                .into_iter()
                .filter(|v| v[at_bit] == choose)
                .collect();

            at_bit += 1;
        }

        bits[0]
            .iter()
            .enumerate()
            .map(|(i, &b)| b * (1 << (w - i - 1)))
            .sum::<usize>()
    };

    select(bits.clone(), true) * select(bits, false)
}

#[test]
fn test_solve() {
    let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    assert_eq!(solve(s), 198);
    assert_eq!(bonus(s), 230);
}
