mod windower;
use windower::Windower;

pub const INPUT: &str = include_str!("day09/input.txt");
pub fn part1(input: &'static str) -> usize {
    let windows = Windower::new(input);
    windows
        .filter_map(|w| {
            let local_min = is_min(&w)?;
            Some(1 + local_min as usize)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut blobs: Vec<usize> = Vec::new();
    let mut cells: Vec<Vec<Option<usize>>> = Vec::new();
    for line in input.lines() {
        let b = line.as_bytes();
        let maybe_prev = cells.last();
        let mut cur = Vec::with_capacity(b.len());
        for (col, cell) in b.iter().enumerate() {
            let blob_id = {
                let up = maybe_prev.map(|r| r[col]).flatten();
                let left = cur.last().copied().flatten();
                match (cell, up, left) {
                    (b'9', _, _) => None,
                    (_, Some(i), None) => Some(i),
                    (_, None, Some(i)) => Some(i),
                    (_, None, None) => {
                        let id = blobs.len();
                        blobs.push(id);
                        Some(id)
                    }
                    (_, Some(i), Some(j)) => {
                        let high = blobs[i].max(blobs[j]);
                        blobs[i] = high;
                        blobs[j] = high;
                        Some(high)
                    }
                }
            };
            cur.push(blob_id);
        }
        cells.push(cur);
    }

    let mut counts = Vec::with_capacity(blobs.len());
    counts.resize(blobs.len(), 0);

    for row in cells {
        for cell in row {
            if let Some(value) = cell {
                counts[value] += 1;
            }
        }
    }

    for (idx, &target) in blobs.iter().enumerate() {
        if idx != target {
            counts[target] += counts[idx];
            counts[idx] = 0;
        }
    }

    counts.sort();
    dbg!(&counts[(counts.len() - 3)..])
        .iter()
        .fold(1, |prev, count| prev * dbg!(count))
}

fn is_min(window: &[u8; 9]) -> Option<u8> {
    let other_min = [1, 3, 5, 7].iter().map(|&i| window[i]).min().unwrap();
    if other_min > window[4] {
        Some(window[4] - b'0')
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    pub const TEST_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

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

    #[test]
    fn gets_part_2() {
        assert_eq!(1134, part2(TEST_INPUT))
    }
}
