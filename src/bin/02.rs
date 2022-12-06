pub enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

pub fn parse_rps(input: &str) -> Vec<(RPS, RPS)> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (them, us) = line.split_at(1);
            let them = match them {
                "A" => RPS::ROCK,
                "B" => RPS::PAPER,
                "C" => RPS::SCISSORS,
                _ => panic!("Invalid value: {them}"),
            };
            let us = match us.trim() {
                "X" => RPS::ROCK,
                "Y" => RPS::PAPER,
                "Z" => RPS::SCISSORS,
                _ => panic!("Invalid value: {us}"),
            };
            (them, us)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let strat = parse_rps(input);
    Some(
        strat
            .iter()
            .map(|round| {
                let shape_score = match round.1 {
                    RPS::ROCK => 1,
                    RPS::PAPER => 2,
                    RPS::SCISSORS => 3,
                };
                let round_score = match round {
                    // We win
                    (RPS::ROCK, RPS::PAPER)
                    | (RPS::PAPER, RPS::SCISSORS)
                    | (RPS::SCISSORS, RPS::ROCK) => 6,
                    // Tie
                    (RPS::ROCK, RPS::ROCK)
                    | (RPS::PAPER, RPS::PAPER)
                    | (RPS::SCISSORS, RPS::SCISSORS) => 3,
                    // We lost
                    _ => 0,
                };
                shape_score + round_score
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
