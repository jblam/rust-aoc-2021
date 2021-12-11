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
}
