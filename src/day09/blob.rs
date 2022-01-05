pub(crate) struct Blob<'a>(Vec<LineRange<'a>>);
impl<'a> Blob<'a> {
    pub fn empty() -> Self {
        Self(Vec::new())
    }
    pub fn new_with(initial: LineRange<'a>) -> Self {
        Self(vec![initial])
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn size(&self) -> usize {
        self.0.iter().map(|r| r.size()).sum()
    }
    pub fn tail_row(&self) -> Option<usize> {
        self.0.last().map(|r| r.row)
    }
    pub fn matches(&self, range: &LineRange<'a>) -> bool {
        self.0
            .iter()
            .rev()
            .skip_while(|r| r.row >= range.row)
            .take_while(|r| r.row == range.row - 1)
            .filter(|&r| r.has_intersection(range))
            .next()
            .is_some()
    }
    pub fn push(&mut self, range: LineRange<'a>) {
        assert!(self.0.last().map(|r| r.row <= range.row).unwrap_or(true));
        self.0.push(range);
    }
    pub fn merge(&mut self, other: &mut Blob<'a>) {
        self.0.append(&mut other.0);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct LineRange<'a> {
    row: usize,
    col: usize,
    values: &'a [u8],
}

impl<'a> LineRange<'a> {
    pub fn get_line_ranges(row: usize, line: &'a [u8]) -> impl Iterator<Item = LineRange<'a>> + 'a {
        line.split(|b| b == &b'9')
            .enumerate()
            .scan(None, move |state, (index, this_slice)| {
                let yielded_length = state.unwrap_or(0);
                *state = Some(yielded_length + this_slice.len());
                Some(Self {
                    row,
                    col: index + yielded_length,
                    values: this_slice,
                })
            })
    }
    pub fn end(&self) -> usize {
        self.col + self.values.len()
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has_intersection(&self, other: &LineRange) -> bool {
        other.end() > self.col && other.col < self.end()
    }
}
