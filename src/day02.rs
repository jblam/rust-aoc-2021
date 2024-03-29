pub fn part1() -> u32 {
    let final_location =
        instructions().fold(Default::default(), |prev: Location, cur| prev.add(&cur));
    final_location.x * final_location.z
}
pub fn part2() -> u32 {
    let final_location =
        instructions().fold(Default::default(), |prev: Location2, cur| prev.add(&cur));
    final_location.x * final_location.z
}

const INPUT: &str = include_str!("day02/input.txt");
fn instructions() -> impl Iterator<Item = Instruction> {
    INPUT.lines().map(|s| {
        let (dir, num) = s.split_once(' ').unwrap();
        let value: u32 = num.parse().unwrap();
        match dir {
            "forward" => Instruction::Forward(value),
            "up" => Instruction::Up(value),
            "down" => Instruction::Down(value),
            _ => panic!("Unexpected instruction"),
        }
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct Location {
    x: u32,
    z: u32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Location2 {
    x: u32,
    z: u32,
    aim: u32,
}

impl Location {
    fn add(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Forward(f) => Self {
                x: self.x + f,
                z: self.z,
            },
            Instruction::Down(d) => Self {
                x: self.x,
                z: self.z + d,
            },
            Instruction::Up(u) => Self {
                x: self.x,
                z: self.z - u,
            },
        }
    }
}
impl Location2 {
    fn add(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Forward(f) => Self {
                x: self.x + f,
                z: self.z + self.aim * f,
                ..*self
            },
            Instruction::Up(u) => Self {
                aim: self.aim - u,
                ..*self
            },
            Instruction::Down(d) => Self {
                aim: self.aim + d,
                ..*self
            },
        }
    }
}

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}
