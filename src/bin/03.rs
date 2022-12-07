use itertools::Itertools;
use std::collections::HashSet;

fn get_rucksacks(input: &str) -> Vec<(&str, &str)> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| line.split_at(line.len() / 2))
        .collect()
}

fn get_compartments(input: Vec<(&str, &str)>) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .iter()
        .map(|(left, right)| {
            (
                HashSet::from_iter(left.chars()),
                HashSet::from_iter(right.chars()),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks = get_compartments(get_rucksacks(input));
    let rscores = rucksacks
        .iter()
        .map(|(left, right)| {
            let intersection = left.intersection(right).at_most_one().unwrap().unwrap();
            if intersection.is_ascii_uppercase() {
                (*intersection as u32 - 64) + 26
            } else {
                *intersection as u32 - 96
            }
        })
        .collect_vec();
    Some(rscores.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_get_rucksacks() {
        let input = advent_of_code::read_file("examples", 3);
        let rucksacks = get_rucksacks(&input)[0];
        assert_eq!(rucksacks.0, "vJrwpWtwJgWr");
        assert_eq!(rucksacks.1, "hcsFMMfFFhFp");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
