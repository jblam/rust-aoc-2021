use std::{cmp::Ordering, collections::HashSet, convert::TryInto, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, u32},
    combinator::opt,
    multi::separated_list0,
    sequence::{separated_pair, terminated},
    IResult,
};

pub const INPUT: &str = include_str!("day05/input.txt");
pub fn part1(input: &str) -> usize {
    let lines = Line::parse_all(input).unwrap();
    let intersections = do_set_things(&lines);
    intersections.len()
}

fn make_partitioins(lines: &[Line]) -> (Vec<Rectilinear>, Vec<Rectilinear>, Vec<Diagonal>) {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut d = Vec::new();
    for l in lines {
        match l.direction().unwrap() {
            Segment::Rectilinear(r) => {
                if r.0 == Direction::X {
                    x.push(r);
                } else {
                    y.push(r);
                }
            }
            Segment::Diagonal(s) => d.push(s),
        }
    }

    x.sort_by(order_by_major);
    y.sort_by(order_by_major);
    return (x, y, d);

    fn order_by_major(u: &Rectilinear, v: &Rectilinear) -> Ordering {
        let outer = u.1.cmp(&v.1);
        match outer {
            Ordering::Equal => u.2.start().cmp(&v.2.start()),
            _ => outer,
        }
    }
}

fn get_self_overlaps<'a, F, G>(
    items: &'a Vec<Rectilinear>,
    m: F,
) -> impl Iterator<Item = (u32, u32)> + 'a
where
    F: FnMut(Rectilinear) -> G + 'a,
    G: Iterator<Item = (u32, u32)> + 'a,
{
    let overlaps = items.iter().scan(None, |state, x| {
        let this_range = x.2.to_owned();
        let prev = state.replace(x);
        match prev {
            Some(first) if first.1 == x.1 => Some(Some(Rectilinear(
                x.0,
                x.1,
                *this_range.start()..=*(first.2.end().min(this_range.end())),
            ))),
            _ => Some(None),
        }
    });
    overlaps.flatten().flat_map(m)
}

fn get_subrange<'a>(items: &'a [Rectilinear], range: &'a RangeInclusive<u32>) -> &'a [Rectilinear] {
    let bigger = &items[items.partition_point(|r| r.1 < *range.start())..];
    &bigger[..bigger.partition_point(|r| r.1 <= *range.end())]
}

fn do_set_things(lines: &[Line]) -> HashSet<(u32, u32)> {
    let (x, y, _) = make_partitioins(lines);
    let overlaps_x = get_self_overlaps(&x, |r| {
        let row = r.1;
        r.2.map(move |col| (row, col))
    });
    let overlaps_y = get_self_overlaps(&y, |r| {
        let col = r.1;
        r.2.map(move |row| (row, col))
    });

    let mut intersections = overlaps_x.chain(overlaps_y).collect::<HashSet<_>>();

    for col in &y {
        let rows = get_subrange(&x, &col.2);
        let rows = rows.iter().filter(|r| r.2.contains(&col.1));
        let points = rows.map(|r| (r.1, col.1));
        for p in points {
            intersections.insert(p);
        }
    }

    intersections
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point(u32, u32);
#[derive(Debug, PartialEq, Clone, Copy)]
struct Line(Point, Point);
impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, (u, v)) = separated_pair(u32, char(','), u32)(input)?;
        Ok((rest, Self(u, v)))
    }
}
impl Line {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, (l, r)) = separated_pair(Point::parse, tag(" -> "), Point::parse)(input)?;
        Ok((rest, Self(l, r)))
    }
    fn parse_all(input: &str) -> Result<Vec<Self>, String> {
        let (rest, vec) =
            terminated(separated_list0(line_ending, Self::parse), opt(line_ending))(input)
                .map_err(|e| e.to_string())?;
        if rest.is_empty() {
            Ok(vec)
        } else {
            Err("Did not consume input".to_string())
        }
    }
    fn direction(&self) -> Result<Segment, String> {
        match *self {
            Line(Point(lx, ly), Point(rx, ry)) if lx == rx => Ok(Segment::Rectilinear(
                Rectilinear(Direction::X, lx, ly.min(ry)..=ly.max(ry)),
            )),
            Line(Point(lx, ly), Point(rx, ry)) if ly == ry => Ok(Segment::Rectilinear(
                Rectilinear(Direction::Y, ly, lx.min(rx)..=lx.max(rx)),
            )),
            Line(Point(lx, ly), Point(rx, ry)) => {
                // doubtless there is a more efficient way to do this,
                // but it is too tedious.
                let dx = (rx as i32) - (lx as i32);
                let dy = (ry as i32) - (ly as i32);
                if dx.abs() != dy.abs() {
                    Err("Non-rectilinear segment was not at a strict 45 degree angle.".to_string())
                } else {
                    let is_positive_x = dx.signum() == dy.signum();
                    let (start, length) = if lx < rx {
                        ((lx, ly), 1 + rx - lx)
                    } else {
                        ((rx, ry), 1 + lx - rx)
                    };
                    Ok(Segment::Diagonal(Diagonal {
                        start,
                        length,
                        is_positive_y: is_positive_x,
                    }))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Segment {
    Rectilinear(Rectilinear),
    Diagonal(Diagonal),
}

#[derive(PartialEq, Eq, Debug)]
struct Rectilinear(Direction, u32, RangeInclusive<u32>);
#[derive(Debug, PartialEq)]
struct Diagonal {
    start: (u32, u32),
    length: u32,
    is_positive_y: bool,
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    X,
    Y,
}
impl Diagonal {
    fn points(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        std::iter::successors(Some(self.start), move |prev| {
            Some((
                prev.0 + 1,
                if self.is_positive_y {
                    prev.1 + 1
                } else {
                    prev.1 - 1
                },
            ))
        })
        .take(self.length.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use is_sorted::IsSorted;
    const TEST_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;
    #[test]
    fn parses_single() {
        let (rest, result) = Line::parse("123,456 -> 78,90").unwrap();
        assert_eq!(result, Line(Point(123, 456), Point(78, 90)));
        assert!(rest.is_empty());
    }
    #[test]
    fn parses_all() -> Result<(), String> {
        let all = Line::parse_all(TEST_INPUT)?;
        assert_eq!(10, all.len());
        Ok(())
    }

    #[test]
    fn partitions_are_sorted() -> Result<(), String> {
        let lines = Line::parse_all(TEST_INPUT)?;
        let (mut x, mut y, _) = make_partitioins(&lines);
        assert!(IsSorted::is_sorted_by_key(&mut x.iter_mut(), |u| u.1));
        assert!(IsSorted::is_sorted_by_key(&mut y.iter_mut(), |u| u.1));
        Ok(())
    }

    #[test]
    fn can_self_overlap() {
        let items = vec![
            Rectilinear(Direction::X, 0, 0..=5),
            Rectilinear(Direction::X, 1, 2..=10),
            Rectilinear(Direction::X, 1, 3..=3),
        ];
        let self_overlaps = get_self_overlaps(&items, |r| {
            let row = r.1;
            r.2.map(move |col| (row, col))
        })
        .collect::<Vec<_>>();
        assert_eq!(vec![(1, 3)], self_overlaps);
    }

    #[test]
    fn gets_part_1() {
        assert_eq!(5, part1(TEST_INPUT))
    }

    #[test]
    fn parses_diagonal_direction() -> Result<(), String> {
        let u = Point(5, 5);
        let v_pos = Point(6, 6);
        let v_neg = Point(6, 4);

        let expected_pos = Diagonal {
            start: (5, 5),
            length: 2,
            is_positive_y: true,
        };
        let expected_neg = Diagonal {
            is_positive_y: false,
            ..expected_pos
        };

        assert_eq!(
            Line(u, v_pos).direction()?,
            Segment::Diagonal(Diagonal { ..expected_pos })
        );
        assert_eq!(Line(v_pos, u).direction()?, Segment::Diagonal(expected_pos));
        assert_eq!(
            Line(u, v_neg).direction()?,
            Segment::Diagonal(Diagonal { ..expected_neg })
        );
        assert_eq!(Line(v_neg, u).direction()?, Segment::Diagonal(expected_neg));
        Ok(())
    }

    #[test]
    fn enumerates_diagonal() {
        let pos = Diagonal {
            start: (5, 5),
            length: 2,
            is_positive_y: true,
        };
        let neg = Diagonal {
            is_positive_y: false,
            ..pos
        };
        assert_eq!(vec![(5, 5), (6, 6)], pos.points().collect::<Vec<_>>());
        assert_eq!(vec![(5, 5), (6, 4)], neg.points().collect::<Vec<_>>());
    }
}
