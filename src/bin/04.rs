use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
#[display("{from}-{to}")]
pub struct CleaningRange {
    from: u16,
    to: u16,
}

impl CleaningRange {
    fn subsumed_by(&self, other: &CleaningRange) -> bool {
        self.from >= other.from && self.to <= other.to
    }

    fn overlaps_with(&self, other: &CleaningRange) -> bool {
        (self.from <= other.to && self.from >= other.from)
            || (self.to >= other.from && self.to < other.to)
    }
}

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
#[display("{one},{two}")]
pub struct CleaningAssignment {
    one: CleaningRange,
    two: CleaningRange,
}

impl CleaningAssignment {
    fn redundant(&self) -> bool {
        self.one.subsumed_by(&self.two) || self.two.subsumed_by(&self.one)
    }

    fn has_overlap(&self) -> bool {
        self.one.overlaps_with(&self.two) || self.two.overlaps_with(&self.one)
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    Some(
        _input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.parse::<CleaningAssignment>()
                    .unwrap_or_else(|_| panic!("Could not parse {}", l))
            })
            .filter(|ca| ca.redundant())
            .count()
            .try_into()
            .expect("Overflow"),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(
        _input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.parse::<CleaningAssignment>()
                    .unwrap_or_else(|_| panic!("Could not parse {}", l))
            })
            .filter(|ca| ca.has_overlap())
            .count()
            .try_into()
            .expect("Overflow"),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
