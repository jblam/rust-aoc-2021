use std::collections::HashSet;

const SIZE: usize = 5;
struct Board {
    numbers: [u8; SIZE * SIZE],
    row_score: [u8; SIZE],
    col_score: [u8; SIZE],
}
impl Board {
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

    fn score(&self, numbers: HashSet<u8>) -> usize {
        self.numbers
            .iter()
            .filter(|&i| !numbers.contains(i))
            .map(|&i| i as usize)
            .sum()
    }

    fn consume(input: &str) -> Option<(Self, &str)> {
        if input.is_empty() {
            None
        } else {
            let mut arr = [0u8; SIZE * SIZE];
            let mut input = input;
            for i in 0..SIZE {
                let (line, rest) = input.split_once('\n')?;
                let line = line.trim_end();
                let part = &mut arr[i * SIZE..][..SIZE];
                let mut j = 0;
                for token in line.split_ascii_whitespace() {
                    part[j] = token.parse().unwrap();
                    j += 1;
                }
                if j != SIZE {
                    return None;
                }
                input = rest;
            }

            let (blank, rest) = input.split_once('\n')?;
            if blank.trim().is_empty() {
                let output = Self {
                    numbers: arr,
                    row_score: [0; SIZE],
                    col_score: [0; SIZE],
                };
                Some((output, rest))
            } else {
                None
            }
        }
    }
}

fn smaller(c: char) -> Option<char> {
    const BASIS: u32 = 0x2080;
    if let Some(i) = c.to_digit(10) {
        char::from_u32(BASIS + i)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                 0,  1,  2,  3,  4,
                 5,  6,  7,  8,  9,
                10, 11, 12, 13, 14,
                15, 16, 17, 18, 19,
                20, 21, 22, 23, 24,
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
    fn parses_board() {
        const BOARD_TEXT: &str = r#"46 53 14 17 75
71  4 70 99 48
65 96 68 80 72
 3 97 62 37 88
82 35 36 23 39

rest"#;
        if let Some((Board { numbers, .. }, rest)) = Board::consume(BOARD_TEXT) {
            assert_eq!("rest", rest);
            assert_eq!(
                [
                    46, 53, 14, 17, 75, 71, 4, 70, 99, 48, 65, 96, 68, 80, 72, 3, 97, 62, 37, 88,
                    82, 35, 36, 23, 39,
                ],
                numbers
            );
        } else {
            assert!(false, "failed to parse board");
        }
    }
}
