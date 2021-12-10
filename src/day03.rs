pub fn part1(s: &'static str) -> usize {
    let output = get_bit_counts(s);
    let (gamma, epsilon) = get_rates(&output);
    gamma * epsilon
}

fn get_bit_counts(s: &'static str) -> Vec<usize> {
    let length = s.lines().next().unwrap().len();
    let mut output = Vec::with_capacity(length);
    output.resize(length, 0);
    let mut count = 0;
    for line in input(s) {
        count += 1;

        for index in line {
            output[index] += 1;
        }
    }
    let threshold = count / 2;
    for i in output.iter_mut() {
        *i = if *i > threshold { 1 } else { 0 }
    }
    output
}
pub const INPUT: &str = include_str!("day03/input.txt");
fn input(s: &'static str) -> impl Iterator<Item = impl Iterator<Item = usize>> {
    s.lines().map(iterate_string)
}

fn iterate_string(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, c)| match c {
            b'0' => None,
            b'1' => Some(idx),
            _ => panic!("Unexpected char in input: {}", c),
        })
}

fn get_rates(bits: &[usize]) -> (usize, usize) {
    let mut big = 0;
    let mut small = 0;
    for i in bits {
        big = (big << 1) | i;
        small = (small << 1) | (1 - i);
    }
    (big, small)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_string() {
        assert_eq!(vec![1], iterate_string("010").collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3], iterate_string("0111").collect::<Vec<_>>());

        // Just as a fun exercise, here's ALL the variations of turbofishes I tried,
        // because the syntax is unfamiliar and difficult to look up:
        // vec![] // unable to infer type
        // Vec::new() // unable to infer type
        // Vec::new<usize>() // fuck off C# peasant, in a _systems language_ we use '::'
        // Vec::new::<usize>() // `new` doesn't want your goddamn turbofish
        // Vec::new() // still can't infer type
        // Vec<usize>::new() // again, C# peasant, go fuck yourself
        // Vec::<usize>::new() // finally!

        assert_eq!(
            Vec::<usize>::new(),
            iterate_string("000").collect::<Vec<_>>()
        );
        assert_eq!(vec![0], iterate_string("1").collect::<Vec<_>>());
    }

    #[test]
    fn gets_rates() {
        assert_eq!((0b000111, 0b111000), get_rates(&[0, 0, 0, 1, 1, 1]));
    }

    const TEST_INPUT: &str = r#"00100
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
01010"#;

    #[test]
    fn gets_test_bit_counts() {
        assert_eq!(vec![1, 0, 1, 1, 0], get_bit_counts(TEST_INPUT));
    }
    #[test]
    fn gets_test_numbers() {
        assert_eq!((22, 9), get_rates(&[1, 0, 1, 1, 0]));
    }
}
