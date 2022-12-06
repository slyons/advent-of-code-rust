use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

lazy_static! {
    static ref SHAPE_SCORES: HashMap<RPS, u32> = {
        let mut m = HashMap::new();
        m.insert(RPS::ROCK, 1);
        m.insert(RPS::PAPER, 2);
        m.insert(RPS::SCISSORS, 3);
        m
    };
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum WLD {
    WIN,
    LOSE,
    DRAW,
}

lazy_static! {
    static ref SHAPE_WLD: HashMap<RPS, HashMap<WLD, RPS>> = {
        let mut shapes = HashMap::new();
        let mut outcomes = HashMap::new();

        //Rock
        outcomes.insert(WLD::WIN, RPS::PAPER);
        outcomes.insert(WLD::DRAW, RPS::ROCK);
        outcomes.insert(WLD::LOSE, RPS::SCISSORS);
        shapes.insert(RPS::ROCK, outcomes);

        let mut outcomes = HashMap::new();
        //Paper
        outcomes.insert(WLD::WIN, RPS::SCISSORS);
        outcomes.insert(WLD::DRAW, RPS::PAPER);
        outcomes.insert(WLD::LOSE, RPS::ROCK);
        shapes.insert(RPS::PAPER, outcomes);

        let mut outcomes = HashMap::new();
        //Scissors
        outcomes.insert(WLD::WIN, RPS::ROCK);
        outcomes.insert(WLD::DRAW, RPS::SCISSORS);
        outcomes.insert(WLD::LOSE, RPS::PAPER);
        shapes.insert(RPS::SCISSORS, outcomes);
        shapes
    };
}

fn round_score(them: &RPS, us: &RPS) -> u32 {
    match (them, us) {
        // We win
        (RPS::ROCK, RPS::PAPER) | (RPS::PAPER, RPS::SCISSORS) | (RPS::SCISSORS, RPS::ROCK) => 6,
        // Tie
        (RPS::ROCK, RPS::ROCK) | (RPS::PAPER, RPS::PAPER) | (RPS::SCISSORS, RPS::SCISSORS) => 3,
        // We lost
        _ => 0,
    }
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

pub fn parse_wld(input: &str) -> Vec<(RPS, WLD)> {
    parse_rps(input)
        .into_iter()
        .map(|(them, us)| match us {
            RPS::ROCK => (them, WLD::LOSE),
            RPS::PAPER => (them, WLD::DRAW),
            RPS::SCISSORS => (them, WLD::WIN),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let strat = parse_rps(input);
    Some(
        strat
            .iter()
            .map(|(them, us)| {
                let shape_score = SHAPE_SCORES[us];
                let round_score = round_score(them, us);
                shape_score + round_score
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_wld(input)
            .iter()
            .map(|(them, outcome)| {
                let our_shape = &SHAPE_WLD[them][outcome];
                let shape_score = SHAPE_SCORES[our_shape];
                let round_scorez = round_score(them, our_shape);
                shape_score + round_scorez
            })
            .sum(),
    )
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
        let wld = parse_wld(&input);
        assert_eq!(
            wld,
            vec!(
                (RPS::ROCK, WLD::DRAW),
                (RPS::PAPER, WLD::LOSE),
                (RPS::SCISSORS, WLD::WIN)
            )
        );
        assert_eq!(part_two(&input), Some(12));
    }
}
