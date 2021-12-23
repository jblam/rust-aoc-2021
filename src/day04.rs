use std::collections::HashSet;

const SIZE: usize = 5;
pub const INPUT: &str = include_str!("day04/input.txt");

pub fn part1(input: &str) -> usize {
    let (numbers, mut boards) = parse_input(input).unwrap();
    play(&numbers, &mut boards).expect("No board won")
}

pub fn part2(input: &str) -> usize {
    let (numbers, mut boards) = parse_input(input).unwrap();
    play_last(&numbers, boards).expect("No single unique board lost")
}

fn play(numbers: &[u8], boards: &mut [Board]) -> Option<usize> {
    let mut h = HashSet::new();
    for &n in numbers {
        h.insert(n);
        for b in boards.iter_mut() {
            if b.mark(n) {
                return Some(b.score(h) * (n as usize));
            }
        }
    }
    None
}
fn play_last(numbers: &[u8], mut boards: Vec<Board>) -> Option<usize> {
    let mut h = HashSet::new();
    for &n in numbers {
        h.insert(n);
        if let [last] = boards.as_mut_slice() {
            if last.mark(n) {
                return Some(last.score(h) * (n as usize));
            }
        } else if boards.is_empty() {
            return None;
        } else {
            let finished = boards
                .iter_mut()
                .enumerate()
                .filter_map(|(idx, b)| if b.mark(n) { Some(idx) } else { None })
                .collect::<Vec<_>>();
            for &idx in finished.iter().rev() {
                boards.remove(idx);
            }
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> Result<(Vec<u8>, Vec<Board>), String> {
    let (first_line, input) =
        one_line(input).ok_or_else(|| "Did not start with the call numbers".to_string())?;
    let numbers = first_line
        .split(',')
        .map(|t| t.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let (_, input) = match one_line(input) {
        Some((line, rest)) if line.is_empty() => Ok((line, rest)),
        _ => Err("Call numbers not followed by an empty line".to_string()),
    }?;
    let first = Board::consume(input)?.ok_or("No boards")?;
    let boards = std::iter::successors(Some(Ok(first)), |prev| {
        // I'm sure there's a more elegant way to do this, but Clippy is not complaining,
        // so ?
        if let Ok((_, input)) = prev {
            let result = Board::consume(input);
            match result {
                Err(e) => Some(Err(e)),
                Ok(None) => None,
                Ok(Some(t)) => Some(Ok(t)),
            }
        } else {
            None
        }
    });
    let boards = boards
        .map(|r| r.map(|t| t.0))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((numbers, boards))
}

fn one_line(input: &str) -> Option<(&str, &str)> {
    let (line, rest) = input.split_once('\n')?;
    let line = line.trim_end_matches('\r');
    Some((line, rest))
}

struct Board {
    numbers: [u8; SIZE * SIZE],
    row_score: [u8; SIZE],
    col_score: [u8; SIZE],
}
impl Board {
    /// Marks the number, if present on the board.
    /// Returns `true` if the board has now won.
    fn mark(&mut self, number: u8) -> bool {
        if let Some(index) = self.numbers.iter().position(|i| i == &number) {
            let col = index % SIZE;
            let row = index / SIZE;
            assert!(col < SIZE);
            assert!(row < SIZE);
            self.row_score[row] += 1;
            self.col_score[col] += 1;

            self.row_score[row] as usize == SIZE || self.col_score[col] as usize == SIZE
        } else {
            false
        }
    }

    /// Calculates the (puzzle-defined) board "score" being the sum of all
    /// unmarked numbers. Does not validate if the board is already won.
    fn score(&self, numbers: HashSet<u8>) -> usize {
        self.numbers
            .iter()
            .filter(|&i| !numbers.contains(i))
            .map(|&i| i as usize)
            .sum()
    }

    /// Produces a board by consuming some part of the string, or `None`
    /// if the input is exhausted.
    fn consume(input: &str) -> Result<Option<(Self, &str)>, String> {
        if input.is_empty() {
            Ok(None)
        } else {
            let mut arr = [0u8; SIZE * SIZE];
            let mut input = input;
            for i in 0..SIZE {
                let (line, rest) = one_line(input).unwrap_or((input, ""));
                let part = &mut arr[i * SIZE..][..SIZE];
                let mut j = 0;
                for token in line.split_ascii_whitespace() {
                    // why do I need to add this error type annotation?
                    // rust-analyzer didn't need it.
                    let result: u8 = token
                        .parse()
                        .map_err(|op: std::num::ParseIntError| op.to_string())?;
                    part[j] = result;
                    j += 1;
                }
                if j != SIZE {
                    return Err(format!("line {} of the board only has {} numbers", i, j));
                }
                input = rest;
            }

            let (blank, rest) = if input.is_empty() {
                (input, input)
            } else {
                input
                    .split_once('\n')
                    .ok_or_else(|| "Board did not terminate with an empty line".to_string())?
            };
            if blank.trim().is_empty() {
                let output = Self {
                    numbers: arr,
                    row_score: [0; SIZE],
                    col_score: [0; SIZE],
                };
                Ok(Some((output, rest)))
            } else {
                Err("Board did not terminate with an empty line".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn smaller(c: char) -> Option<char> {
        const BASIS: u32 = 0x2080;
        if let Some(i) = c.to_digit(10) {
            char::from_u32(BASIS + i)
        } else {
            None
        }
    }
    #[test]
    fn emits_small_digits() {
        assert_eq!(Some('₁'), smaller('1'));
        assert_eq!(Some('₉'), smaller('9'));
        assert_eq!(None, smaller('_'));
    }

    #[test]
    fn marks_board() {
        let mut board = Board {
            numbers: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24,
            ],
            col_score: [0; 5],
            row_score: [0; 5],
        };
        for i in 0..(SIZE as u8) - 1 {
            assert_eq!(false, board.mark(i));
        }
        assert!(board.mark(SIZE as u8 - 1));
        assert_eq!([SIZE as u8, 0, 0, 0, 0], board.row_score);

        let n = (0..SIZE as u8).collect::<HashSet<_>>();
        assert_eq!((SIZE..(SIZE * SIZE)).sum::<usize>(), board.score(n));
    }

    #[test]
    fn parses_board() -> Result<(), String> {
        const BOARD_TEXT: &str = r#"46 53 14 17 75
71  4 70 99 48
65 96 68 80 72
 3 97 62 37 88
82 35 36 23 39

rest"#;
        let (Board { numbers, .. }, rest) =
            Board::consume(BOARD_TEXT)?.ok_or("Consume unexpectedly returned None")?;
        assert_eq!("rest", rest);
        assert_eq!(
            [
                46, 53, 14, 17, 75, 71, 4, 70, 99, 48, 65, 96, 68, 80, 72, 3, 97, 62, 37, 88, 82,
                35, 36, 23, 39,
            ],
            numbers
        );
        Ok(())
    }
    #[test]
    fn parses_no_board() -> Result<(), String> {
        let result = Board::consume("")?;
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn parses_error() {
        let result = Board::consume("not a good line");
        assert!(result.is_err())
    }

    const TEST_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
    #[test]
    fn parses_test_input() -> Result<(), String> {
        let (numbers, mut boards) = parse_input(TEST_INPUT)?;
        let score = play(&numbers, &mut boards).ok_or("No board won".to_string())?;
        assert_eq!(4512, score);
        Ok(())
    }

    #[test]
    fn runs_part_two() -> Result<(), String> {
        let (numbers, mut boards) = parse_input(TEST_INPUT)?;
        let score = play_last(&numbers, boards).ok_or("No single unique board lost".to_string())?;
        assert_eq!(1924, score);
        Ok(())
    }
}
