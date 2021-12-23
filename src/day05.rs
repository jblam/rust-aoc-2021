use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, u32},
    combinator::opt,
    multi::separated_list0,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

#[derive(Debug, PartialEq)]
struct Point(u32, u32);
#[derive(Debug, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
