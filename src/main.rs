use std::io::stdin;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    println!("What day?");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).expect("Couldn't read stdin");
    let day = input.trim().parse::<u32>().expect("Input was not a number.");
    match day {
        1 => println!("Day 1: {}, {}", day01::part1(), day01::part2()),
        2 => println!("Day 2: {}, {}", day02::part1(), day02::part2()),
        3 => println!("Day 3: {}, {}", day03::part1(day03::INPUT), day03::part2(day03::INPUT)),
        4 => println!("Day 4: {}, {}", day04::part1(day04::INPUT), day04::part2(day04::INPUT)),
        5 => println!("Day 5: {}, {}", day05::part1(day05::INPUT), day05::part2(day05::INPUT)),
        6 => println!("Day 6: {}, {}", day06::part1(day06::INPUT), day06::part2(day06::INPUT)),
        7 => println!("Day 7: {}, {}", day07::part1(day07::INPUT), day07::part2(day07::INPUT)),
        8 => println!("Day 8: {}, {}", day08::part1(day08::INPUT), "Nope"),// day07::part2(day07::INPUT)),
        _ => println!("NOPE."),
    };
}
