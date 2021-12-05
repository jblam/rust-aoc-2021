use std::io::stdin;

mod day01;
mod day02;

fn main() {
    println!("What day?");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).expect("Couldn't read stdin");
    let day = input.trim().parse::<u32>().expect("Input was not a number.");
    match day {
        1 => println!("Day 1: {}, {}", day01::part1(), day01::part2()),
        2 => println!("Day 2: {}", day02::part1()),
        _ => println!("Nope."),
    };
}
