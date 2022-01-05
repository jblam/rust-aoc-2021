
pub(crate) struct Windower {
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
    use super::super::tests::TEST_INPUT;

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
}
