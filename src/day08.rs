
#[derive(PartialEq, Debug)]
struct Digit<'a>(&'a str);
impl Digit<'static> {
    const DEFAULT: Digit<'static> = Digit("");
}
impl<'a> Digit<'a> {
    fn from_str(input: &'a str) -> Self {
        if input.is_ascii() && !input.is_empty() {
            Digit(input)
        } else {
            panic!("Unexpected or missing token.");
        }
    }
}
#[derive(PartialEq, Debug)]
struct Entry<'a>([Digit<'a>; 10], [Digit<'a>; 4]);

impl<'a> Entry<'a> {
    fn parse(line: &'a str) -> Entry<'a> {
        let mut tokens = line.split_ascii_whitespace();
        let mut output_left = [Digit::DEFAULT; 10];
        let mut output_right = [Digit::DEFAULT; 4];
        
        for i in 0..10 {
            let token = tokens.next().unwrap();
            output_left[i] = Digit::from_str(token);
        }
        if tokens.next() != Some("|") {
            panic!("Missing expected central delimiter");
        }
        for i in 0..4 {
            let token = tokens.next().unwrap();
            output_right[i] = Digit::from_str(token);
        }

        Self(output_left, output_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ROW: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn can_parse() {
        let e = Entry::parse(TEST_ROW);
        assert_eq!([Digit("cdfeb"), Digit("fcadb"), Digit("cdfeb"), Digit("cdbaf")], e.1);
        assert!(e.0.iter().all(|d| !d.0.is_empty()));
    }
}