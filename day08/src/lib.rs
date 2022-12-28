#![feature(array_zip)]

pub fn solve(s: &str) -> usize {
    // 1 uses 2
    // 4 uses 4
    // 7 uses 3
    // 8 uses 7

    s.lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .flat_map(|line| line.split(" "))
        .map(|code| code.len())
        .filter(|&len| len == 2 || len == 4 || len == 3 || len == 7)
        .count()
}

// how many? [a's, b's, c's, d's, e's, f's, g's]
type Count = [usize; 7];

fn code_to_count(code: &str) -> Count {
    let mut count = [0; 7];

    for c in code.chars() {
        let i = (c as usize) - ('a' as usize);
        count[i] += 1;
    }

    count
}

fn add_counts(x: Count, y: Count) -> Count {
    x.zip(y).map(|(x, y)| x + y)
}

fn display([a, b, c, d, e, f, g]: Count) -> usize {
    let sum = a + b + c + d + e + f + g;

    if sum == 7 {
        8
    } else if sum == 2 {
        1
    } else if sum == 3 {
        7
    } else if sum == 4 {
        4
    } else if sum == 5 && b == 1 {
        5
    } else if sum == 5 && e == 1 {
        2
    } else if sum == 5 {
        3
    } else if sum == 6 && c == 0 {
        6
    } else if sum == 6 && d == 1 {
        9
    } else {
        0
    }
}

fn decode(line: &str) -> usize {
    let (pattern, message) = line.split_once(" | ").unwrap();

    // incorrect -> count
    let total_count = pattern
        .split(" ")
        .map(code_to_count)
        .reduce(add_counts)
        .unwrap();

    //     a
    //  b     c
    //     d
    //  e     f
    //     g

    let b = total_count.iter().position(|&k| k == 6).unwrap();
    let e = total_count.iter().position(|&k| k == 4).unwrap();
    let f = total_count.iter().position(|&k| k == 9).unwrap();

    let mut one = pattern
        .split(" ")
        .find(|&code| code.len() == 2)
        .map(code_to_count)
        .unwrap();

    one[f] -= 1;
    let c = one.iter().position(|&k| k == 1).unwrap();

    let a = total_count
        .iter()
        .enumerate()
        .position(|(i, &k)| k == 8 && i != c)
        .unwrap();

    let mut four = pattern
        .split(" ")
        .find(|&code| code.len() == 4)
        .map(code_to_count)
        .unwrap();

    four[b] -= 1;
    four[c] -= 1;
    four[f] -= 1;
    let d = four.iter().position(|&k| k == 1).unwrap();

    let g = (0..7)
        .find(|&i| i != a && i != b && i != c && i != d && i != e && i != f)
        .unwrap();

    // incorrect -> correct
    let mut found = [0; 7];
    found[a] = 0;
    found[b] = 1;
    found[c] = 2;
    found[d] = 3;
    found[e] = 4;
    found[f] = 5;
    found[g] = 6;

    let decode = |encoded: Count| {
        let mut decoded = [0; 7];

        for i in 0..7 {
            if encoded[i] == 1 {
                decoded[found[i]] = 1;
            }
        }

        decoded
    };

    let digits = message
        .split(" ")
        .map(code_to_count)
        .map(decode)
        .map(display)
        .collect::<Vec<_>>();

    let n = digits.len();

    let output = digits
        .iter()
        .enumerate()
        .map(|(i, &digit)| digit * 10usize.pow((n - i - 1) as u32))
        .sum::<usize>();

    output
}

#[test]
fn test_decode() {
    assert_eq!(
        decode(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        ),
        5353
    );
}

pub fn bonus(s: &str) -> usize {
    s.lines().map(decode).sum()
}

#[test]
fn test_solve() {
    let s = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    assert_eq!(solve(s), 26);
    assert_eq!(bonus(s), 61229);
}
