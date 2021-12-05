pub fn part1() -> u32 {
    let mut l = numbers().peekable();
    let mut count = 0;
    while let Some(line) = l.next() {
        if let Some(next) = l.peek() {
            let (u, v) = (line, next);
            let increment = if v > &u { 1 } else { 0 };
            count += increment;
        }
    }
    count
}

pub fn part2() -> u32 {
    const LENGTH: usize = 3;
    let mut v = Vec::with_capacity(LENGTH);
    let mut count = 0;
    for (index, n) in numbers().enumerate() {
        if v.len() < LENGTH {
            v.push(n);
        } else {
            let old_sum: u32 = v.iter().sum();
            v[index % LENGTH] = n;
            count += if v.iter().sum::<u32>() > old_sum { 1 } else { 0 };
        }
    }
    count
}

const INPUT: &str = include_str!("day01/input.txt");
fn numbers() -> impl Iterator<Item = u32> {
    INPUT.lines().map(|s| s.parse::<u32>().unwrap())
}
