pub const INPUT: &str = include_str!("day08/input.txt");
pub fn part1(input: &str) -> usize {
    let entries = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(Entry::parse);
    entries
        .map(|e| {
            e.output
                .iter()
                .filter(|d| d.infer_value().is_some())
                .count()
        })
        .sum()
}

#[derive(PartialEq, Debug)]
struct Digit(u8);
impl Digit {
    const DEFAULT: Digit = Digit(0);
}
impl Digit {
    fn from_str(input: &str) -> Self {
        if input.len() > 7 {
            panic!("Unexpectedly-long string");
        }
        let mut output = 0;
        for b in input.bytes() {
            let bit = b - b'a';
            output |= 1u8 << bit;
        }
        Self(output)
    }

    fn infer_value(&self) -> Option<u8> {
        match self.0.count_ones() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn categorise_six_segment(&self, one: &Digit, four: &Digit) -> u8 {
        todo!()
    }
}
#[derive(PartialEq, Debug)]
struct Entry {
    reference: [Digit; 10],
    output: [Digit; 4],
}

impl Entry {
    fn parse(line: &str) -> Entry {
        let mut tokens = line.split_ascii_whitespace();
        let mut output_left = [Digit::DEFAULT; 10];
        let mut output_right = [Digit::DEFAULT; 4];

        for i in 0..10 {
            let token = tokens.next().unwrap();
            output_left[i] = Digit::from_str(token);
        }
        if tokens.next() != Some("|") {
            panic!("Missing expected central delimiter");
        }
        for i in 0..4 {
            let token = tokens.next().unwrap();
            output_right[i] = Digit::from_str(token);
        }

        Self {
            reference: output_left,
            output: output_right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ROW: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn can_parse() {
        let e = Entry::parse(TEST_ROW);
        assert_eq!(
            [
                Digit::from_str("cdfeb"),
                Digit::from_str("fcadb"),
                Digit::from_str("cdfeb"),
                Digit::from_str("cdbaf")
            ],
            e.output
        );
        assert!(e.reference.iter().all(|d| !d.0 > 0));
    }

    const TEST_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | cgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;
    #[test]
    fn gets_part_1() {
        assert_eq!(part1(TEST_INPUT), 26);
    }
}
