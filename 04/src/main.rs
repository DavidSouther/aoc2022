use std::ops::RangeInclusive;

pub struct Range(RangeInclusive<u32>);

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        let start = self.0.contains(other.0.start());
        let end = self.0.contains(other.0.end());
        // start && end
        start || end
    }

    pub fn cover(&self, other: &Range) -> bool {
        let a = self.contains(other);
        let b = other.contains(self);

        a || b
    }
}

fn split<'a>(value: &'a str, pattern: &str) -> (&'a str, &'a str) {
    let value = value.trim();
    let split: Vec<&'a str> = value.split(pattern).collect();
    eprintln!("{split:?}");
    let a: &'a str = split[0];
    let b: &'a str = split[1];
    (a, b)
}

fn parse_range(value: &str) -> Range {
    let (a, b) = split(value, "-");
    let a: u32 = a.parse().unwrap();
    let b: u32 = b.parse().unwrap();
    Range(a..=b)
}

pub fn parse_range_pairs(value: &str) -> (Range, Range) {
    let (a, b) = split(value, ",");
    (parse_range(a), parse_range(b))
}

const ASSIGNMENTS: &str = include_str!("../data");

fn main() {
    let overlaps: Vec<(Range, Range)> = ASSIGNMENTS
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.trim())
        .map(parse_range_pairs)
        .filter(|(a, b)| a.cover(&b))
        .collect();
    eprintln!("{}", overlaps.len());
}

#[cfg(test)]
mod test {
    use crate::parse_range_pairs;

    fn parse_cover(value: &str, should_cover: bool) {
        let (a, b) = parse_range_pairs(value);
        assert_eq!(a.cover(&b), should_cover);
    }

    #[test]
    fn ranges() {
        parse_cover("2-4,6-8", false);
        parse_cover("2-3,4-5", false);
        parse_cover("5-7,7-9", false);
        parse_cover("2-8,3-7", true);
        parse_cover("6-6,4-6", true);
        parse_cover("2-6,4-8", false);
    }
}
