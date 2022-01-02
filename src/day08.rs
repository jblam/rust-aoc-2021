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
pub fn part2(input: &str) -> usize {
    let entries = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(Entry::parse);
    entries
        .map(|mut e| {
            e.solve();
            e.get_output()
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
        match (self.0 & four.0).count_ones() {
            4 => 9,
            3 => match (self.0 & one.0).count_ones() {
                2 => 0,
                1 => 6,
                _ => panic!("Illegal bit representation."),
            },
            _ => panic!("Illegal bit representation."),
        }
    }
    fn categorise_five_segment(&self, one: &Digit, six: &Digit) -> u8 {
        match (self.0 & one.0).count_ones() {
            2 => 3,
            1 => match (self.0 & six.0).count_ones() {
                5 => 5,
                4 => 2,
                _ => panic!(
                    "Illegal 5-bit representation {:#b}, given 1:{:#b}, 6:{:#b}.",
                    self.0, one.0, six.0
                ),
            },
            _ => panic!("Illegal bit representation."),
        }
    }
}
#[derive(PartialEq, Debug)]
struct Entry {
    reference: [Digit; Self::REFERENCE_LENGTH],
    output: [Digit; Self::OUTPUT_LENGTH],
}

impl Entry {
    const REFERENCE_LENGTH: usize = 10;
    const OUTPUT_LENGTH: usize = 4;
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

    fn solve(&mut self) {
        // Asking for [None; 10] gives E0277 because it doesn't know how to copy Some(Digit(asdf)).
        // Firstly: you don't need to fucking copy a digit because (checks notes) None is not a Digit.
        // Secondly: [repeats None 10 times] this is the exact fucking code you'd want to generate?
        // Thirdly: if only I could hire a robot to type out "None" 10 times;
        //          does anyone know a programming language that would allow it?
        // Fourthly: what the fucking fuck.
        let mut scratch = [None, None, None, None, None, None, None, None, None, None];
        let (mut one, mut four, mut six) = (None, None, None);
        for i in &self.reference {
            if let Some(value) = i.infer_value() {
                scratch[value as usize] = Some(Digit(i.0));
                match value {
                    1 => one = Some(i.clone()),
                    4 => four = Some(i.clone()),
                    _ => (),
                };
            }
        }
        let one = one.unwrap();
        let four = four.unwrap();
        for i in self.reference.iter().filter(|d| d.0.count_ones() == 6) {
            let value = i.categorise_six_segment(one, four);
            scratch[value as usize] = Some(Digit(i.0));
            if value == 6 {
                six = Some(i.clone());
            }
        }
        let six = six.unwrap();
        for i in self.reference.iter().filter(|d| d.0.count_ones() == 5) {
            scratch[i.categorise_five_segment(one, six) as usize] = Some(Digit(i.0));
        }

        for i in 0..Self::REFERENCE_LENGTH {
            self.reference[i] = Digit(scratch[i].as_ref().unwrap().0);
        }
    }

    fn get_output(&self) -> usize {
        // check we have now emplaced all the integers.
        assert!([6, 2, 5, 5, 4, 5, 6, 3, 7, 6]
            .iter()
            .copied()
            .eq(self.reference.iter().map(|d| d.0.count_ones())));

        let mut output = 0;
        for i in &self.output {
            output *= 10;
            output += self
                .reference
                .iter()
                .enumerate()
                .find(|(_, digit)| digit == &i)
                .unwrap()
                .0;
        }
        output
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
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
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

    #[test]
    fn can_solve() {
        let mut sut = Entry::parse(TEST_ROW);
        // from the problem statement:
        // So, the unique signal patterns would correspond to the following digits:
        // acedgfb: 8
        // cdfbe: 5
        // gcdfa: 2
        // fbcad: 3
        // dab: 7
        // cefabd: 9
        // cdfgeb: 6
        // eafb: 4
        // cagedb: 0
        // ab: 1

        sut.solve();
        let expected = [
            Digit::from_str("cagedb"),
            Digit::from_str("ab"),
            Digit::from_str("gcdfa"),
            Digit::from_str("fbcad"),
            Digit::from_str("eafb"),
            Digit::from_str("cdfbe"),
            Digit::from_str("cdfgeb"),
            Digit::from_str("dab"),
            Digit::from_str("acedgfb"),
            Digit::from_str("cefabd"),
        ];
        assert_eq!(expected, sut.reference);

        let value = sut.get_output();
        assert_eq!(5353, value);
    }

    #[test]
    fn solves_expected_values() {
        /*
            fdgacbe cefdb cefbgd gcbe: 8394
            fcgedb cgb dgebacf gc: 9781
            cg cg fdcagb cbg: 1197
            efabcd cedba gadfec cb: 9361
            gecf egdcabf bgf bfgea: 4873
            gebdcfa ecba ca fadegcb: 8418
            cefg dcbef fcge gbcadfe: 4548
            ed bcgafe cdgba cbgef: 1625
            gbdfcae bgc cg cgb: 8717
            fgae cfgab fg bagce: 4315
        */
        let expected = vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];
        let actual = TEST_INPUT
            .lines()
            .take_while(|l| !l.is_empty())
            .map(Entry::parse)
            .map(|mut e| {
                e.solve();
                e.get_output()
            })
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn gets_part_2() {
        assert_eq!(61229, part2(TEST_INPUT));
    }
}
