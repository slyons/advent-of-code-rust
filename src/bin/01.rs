use itertools::Itertools;

pub fn get_batches(input: &str) -> Vec<Vec<u32>> {
    let (mut acc, left) = input.split("\n")
        .map(|l| match l.parse::<u32>() {
            Ok(u) => Some(u),
            Err(_) => None,
        })
        .fold((Vec::<Vec::<u32>>::new(), Vec::<u32>::new()), |mut acc, x| {
            match x {
                Some(u) => {
                    acc.1.push(u);
                    acc
                },
                None => {
                    acc.0.push(acc.1);
                    (acc.0, Vec::new())
                }
            }
        });
    acc.push(left);
    acc
}

pub fn part_one(input: &str) -> Option<u32> {
    let batches = get_batches(input);
    batches
        .iter()
        .map(|bp| 
            bp.iter().sum()
        )
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = advent_of_code::read_file("inputs", 1);
        
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(30));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
