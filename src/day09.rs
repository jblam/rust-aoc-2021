mod windower;
mod blob;

use windower::Windower;
use blob::{Blob, LineRange};

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
    let mut active_blobs = Vec::<Blob>::new();
    let mut closed_blobs = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let ranges = LineRange::get_line_ranges(row, line.as_bytes())
            .filter(|r| r.size() > 0)
            .collect::<Vec<_>>();
        // we now have active_blobs X ranges, which can bifurcate in two ways:
        // - an existing blob can link to two new ranges, or
        // - a single new range can unite two previously-separate blobs
        // The former is represented by a single blob having mutliple "tails";
        // the latter requires we show a candidate range to each active blob,
        // add to the first match and merge subsequent matches.
        for r in ranges {
            let mut merge_target: Option<&mut Blob> = None;
            for a in &mut active_blobs {
                if a.matches(&r) {
                    if let Some(ref mut target) = merge_target {
                        target.merge(a);
                    } else {
                        a.push(r.clone());
                        merge_target = Some(a);
                    }
                }
            }
            if merge_target.is_none() {
                active_blobs.push(Blob::new_with(r));
            }
        }

        for maybe_inactive in active_blobs.iter_mut() {
            match maybe_inactive.tail_row() {
                Some(r) if r < row => {
                    let mut replacement = Blob::empty();
                    std::mem::swap(&mut replacement, maybe_inactive);
                    closed_blobs.push(replacement);
                }
                _ => (),
            };
        }

        active_blobs.retain(|b| !b.is_empty());
    }
    closed_blobs.append(&mut active_blobs);
    closed_blobs.sort_by_key(|v| -(v.size() as isize));
    assert!(
        closed_blobs.len() >= 3,
        "Expected at least 3 blobs; found {}.",
        closed_blobs.len()
    );
    closed_blobs[..3].iter().fold(1, |prev, ranges| {
        prev * ranges.size()
    })
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
