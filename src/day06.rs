pub fn part1(input: &str) -> usize {
    let board = Board::parse(input);
    let board = board.step_many(80);
    board.count()
}

pub const INPUT: &str = include_str!("day06/input.txt");

#[derive(Debug, PartialEq)]
struct Board([usize; 9]);

impl Board {
    const BIRTH_CYCLE: usize = 6;
    fn step(&self) -> Self {
        let mut next = self.0.clone();
        next.rotate_left(1);
        let births = next[next.len() - 1];
        next[Self::BIRTH_CYCLE] += births;
        Self(next)
    }
    fn step_many(&self, count: usize) -> Self {
        let mut value = None;
        for _ in 0..count {
            value = Some(value.as_ref().unwrap_or(self).step());
        }
        value.unwrap()
    }
    fn count(&self) -> usize {
        self.0.iter().sum()
    }
    fn parse(line: &str) -> Self {
        let line = {
            let mut lines = line.lines();
            let l = lines.next().expect("No newline in source");
            if let Some(second) = lines.next() {
                if !second.is_empty() {
                    panic!("Unexpected second line of input");
                }
            }
            l
        };
        let mut output = [0; 9];
        for token in line.split(',').map(|s| s.parse::<usize>()) {
            let token = token.unwrap();
            if token > 8 {
                panic!("Unexpected age out of bounds: {}", token);
            }
            output[token] += 1;
        }
        Self(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_parse() {
        let board = Board::parse("3,4,3,1,2\n");
        assert_eq!(Board([0, 1, 1, 2, 1, 0, 0, 0, 0]), board);
    }

    #[test]
    fn does_step() {
        // initial state from problem:
        // 3,4,3,1,2
        // 2,3,2,0,1
        // 1,2,1,6,0,8
        let initial = Board([0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let step_1 = initial.step();
        let step_2 = step_1.step();
        assert_eq!(Board([1, 2, 1, 0, 0, 0, 1, 0, 1]), step_2);
    }

    #[test]
    fn steps_many() {
        let initial = "3,4,3,1,2";
        let expected = "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8";

        let board = Board::parse(initial);
        let board = board.step_many(18);
        assert_eq!(Board::parse(expected), board);
        assert_eq!(26, board.count());
    }
    #[test]
    fn gets_part_1() {
        let initial = "3,4,3,1,2";
        assert_eq!(5934, part1(initial));
    }
}
