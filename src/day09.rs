pub const INPUT: &str = include_str!("day09/input.txt");
pub fn part1(input: &'static str) -> usize {
    let windows = Windower::new(input);
    windows.filter_map(|w| {
        let local_min = is_min(&w)?;
        Some(1 + local_min as usize)
    }).sum()
}

fn is_min(window: &[u8; 9]) -> Option<u8> {
    let other_min = [1, 3, 5, 7].iter().map(|&i| window[i]).min().unwrap();
    if other_min > window[4] {
        Some(window[4] - b'0')
    } else {
        None
    }
}

struct Windower {
    _source: &'static str,
    lines: Vec<&'static [u8]>,
    index: usize,
}
impl Windower {
    pub fn new(source: &'static str) -> Self {
        let lines = source.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
        Self {
            _source: source,
            lines,
            index: 0,
        }
    }
}
impl Iterator for Windower {
    type Item = [u8; 9];

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = [0xFFu8; 9];
        let line_length = self.lines[0].len();
        let (row, col) = (self.index / line_length, self.index % line_length);
        if row >= self.lines.len() {
            None
        } else {
            self.index += 1;
            let (col_source, col_dest) = match col {
                0 => (0..=1, 1..=2),
                i if i + 1 < line_length => ((i - 1)..=(i + 1), (0..=2)),
                i => ((i - 1)..=i, 0..=1),
            };

            if row > 0 {
                let source = &self.lines[row - 1][col_source.clone()];
                let dest = &mut output[col_dest.clone()];
                dest.copy_from_slice(source);
            }
            {
                let source = &self.lines[row][col_source.clone()];
                let dest = &mut output[(col_dest.start() + 3)..=(col_dest.end() + 3)];
                dest.copy_from_slice(source);
            }
            if row + 1 < self.lines.len() {
                let source = &self.lines[row + 1][col_source.clone()];
                let dest = &mut output[(col_dest.start() + 6)..=(col_dest.end() + 6)];
                dest.copy_from_slice(source);
            }
            Some(output)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn creates_windows() {
        let w = Windower::new(TEST_INPUT);
        let all = w.collect::<Vec<_>>();
        assert_eq!(10 * 5, all.len());
        assert_eq!([0xff, 0xff, 0xff, 0xff, b'2', b'1', 0xff, b'3', b'9'], all[0]);
        assert_eq!(
            [b'8', b'9', 0xff, b'7', b'8', 0xff, 0xff, 0xff, 0xff],
            all[all.len() - 1]
        );
    }

    #[test]
    fn finds_min() {
        let expected_no_min = [0xff, 0xff, 0xff, 0xff, b'2', b'1', 0xff, b'3', b'9'];
        let expected_min = [0xff, 0xff, 0xff, b'2', b'1', b'9', b'3', b'9', b'8'];
        assert!(is_min(&expected_no_min).is_none());
        assert_eq!(is_min(&expected_min), Some(1));
    }

    #[test]
    fn gets_part_1() {
        assert_eq!(15, part1(TEST_INPUT));
    }
}
