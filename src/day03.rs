pub fn part1(s: &'static str) -> usize {
    let output = get_bit_counts(s);
    let (gamma, epsilon) = get_rates(&output);
    gamma * epsilon
}
pub fn part2(s: &'static str) -> u32 {
    let length = s.lines().next().unwrap().len();
    let mut values = s.lines().map(to_integer).collect::<Vec<_>>();
    values.sort_unstable();

    let (o2, co2) = (
        get_candidate(&values, length, true),
        get_candidate(&values, length, false),
    );
    o2.expect("Failed to determine O2") * co2.expect("Failed to determine CO2")
}

fn get_candidate(values: &[u32], length: usize, wants_majority: bool) -> Option<u32> {
    let mut partition = values;
    for i in (0..length).rev() {
        partition = get_partition(partition, i, wants_majority);
        if partition.len() == 1 {
            return Some(partition[0]);
        }
    }
    None
}

fn get_partition(t: &[u32], bit_index: usize, wants_majority: bool) -> &[u32] {
    let sentinel = 1u32 << bit_index;
    let point = t.partition_point(|i| (i & sentinel) == 0);
    // point_is_low is true if it lies before the midpoint of the original slice,
    // in which case the "high" half is the majority.

    let wants_top_half = match point * 2 {
        l if l <= t.len() => wants_majority,
        _ => !wants_majority,
    };

    // let point_is_low = point < t.len() / 2;
    // let wants_top_half = point_is_low == wants_majority;
    if wants_top_half {
        &t[..point]
    } else {
        &t[point..]
    }
}

fn get_bit_counts(s: &'static str) -> Vec<usize> {
    let length = s.lines().next().unwrap().len();
    let mut output = vec![0; length];
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
fn to_integer(s: &str) -> u32 {
    s.as_bytes().iter().fold(0, |prev, cur| {
        (prev << 1)
            | match cur {
                b'0' => 0,
                b'1' => 1,
                _ => panic!("Unexpected char in input: {}", cur),
            }
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
    fn gets_test_part1() {
        assert_eq!((22, 9), get_rates(&[1, 0, 1, 1, 0]));
    }

    #[test]
    fn gets_number() {
        assert_eq!(0b111000, to_integer("111000"))
    }

    #[test]
    fn gets_partition() {
        let input = vec![0x00, 0xFF, 0xFF];
        let min = get_partition(&input, 2, false);
        let max = get_partition(&input, 2, true);
        assert_eq!(&[0x00], min);
        assert_eq!(&[0xFF, 0xFF], max);
    }

    #[test]
    fn gets_test_part2() {
        assert_eq!(230, part2(TEST_INPUT))
    }
}
