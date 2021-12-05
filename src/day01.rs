pub fn part1() -> u32 {
    let input = include_str!("day01/input.txt");
    let mut l = input.lines().map(|s| s.parse::<u32>()).peekable();
    let mut count = 0;
    while let Some(line) = l.next() {
        if let Some(next) = l.peek() {
            let (u, v) = (line.unwrap(), next.as_ref().unwrap());
            let increment = if v > &u { 1 } else { 0 };
            count += increment;
        }
    }
    count
}