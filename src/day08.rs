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
struct Digit<'a>(&'a str);
impl Digit<'static> {
    const DEFAULT: Digit<'static> = Digit("");
}
impl<'a> Digit<'a> {
    fn from_str(input: &'a str) -> Self {
        if input.len() > 7 {
            panic!("Unexpectedly-long string");
        }
        if !input.is_ascii() {
            panic!("Illegal (non-ASCII) chars");
        }
        if input.as_bytes().iter().any(|&b| b < b'a' || b > b'g') {
            panic!("unexpected ASCII char value");
        }
        Digit(input)
    }

    fn infer_value(&'a self) -> Option<u8> {
        match self.0.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
}
#[derive(PartialEq, Debug)]
struct Entry<'a> {
    reference: [Digit<'a>; 10],
    output: [Digit<'a>; 4],
}

impl<'a> Entry<'a> {
    fn parse(line: &'a str) -> Entry<'a> {
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
                Digit("cdfeb"),
                Digit("fcadb"),
                Digit("cdfeb"),
                Digit("cdbaf")
            ],
            e.output
        );
        assert!(e.reference.iter().all(|d| !d.0.is_empty()));
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
