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

fn get_priority(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        (*c as u32 - 64) + 26
    } else {
        *c as u32 - 96
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks = get_compartments(get_rucksacks(input));
    let rscores = rucksacks
        .iter()
        .map(|(left, right)| {
            let intersection = left.intersection(right).at_most_one().unwrap().unwrap();
            get_priority(intersection)
        })
        .collect_vec();
    Some(rscores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks = input.split('\n');
    let groups = &rucksacks
        .filter(|l| !l.is_empty())
        .map(|l| HashSet::from_iter(l.chars()))
        .chunks(3);
    let mut group_results = Vec::new();
    for chunk in groups {
        let intersection = chunk
            .reduce(|accum: HashSet<char>, item| {
                HashSet::from_iter(accum.intersection(&item).cloned())
            })
            .map(|hs| *(hs.iter().next().unwrap()))
            .unwrap();
        group_results.push(get_priority(&intersection));
    }
    Some(group_results.iter().sum())
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
        assert_eq!(part_two(&input), Some(70));
    }
}
