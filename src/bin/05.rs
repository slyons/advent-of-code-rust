use parse_display::{Display, FromStr};

#[derive(Display, Debug, FromStr, PartialEq, Eq)]
#[display("move {count} from {from} to {to}")]
pub struct MoveInst {
    count: usize,
    from: usize,
    to: usize,
}

pub fn split_instructions(input: &str) -> (Vec<Vec<char>>, Vec<MoveInst>) {
    let mut stack_lines = Vec::new();
    let mut instructions = Vec::new();
    let mut indexes = Vec::new();
    let mut stacks_width = 0;
    for l in input.lines() {
        if !l.is_empty() {
            if l.starts_with("move") {
                let inst = l
                    .parse::<MoveInst>()
                    .unwrap_or_else(|_| panic!("Unable to parse line {}", l));
                instructions.push(inst);
            } else if !l.starts_with(" 1") {
                stack_lines.push(l);
            } else {
                for (i, c) in l.chars().enumerate() {
                    if !c.is_whitespace() {
                        indexes.push(i);
                        stacks_width = c.to_string().parse::<usize>().unwrap();
                    }
                }
            }
        }
    }
    let mut stacks = Vec::new();
    for _i in 0..stacks_width {
        stacks.push(Vec::new());
    }

    for sline in stack_lines {
        for (i, idx) in indexes.iter().enumerate() {
            println!("IDX is {}", idx);
            if let Some(c) = sline.chars().nth(*idx) {
                if !c.is_whitespace() {
                    stacks[i].insert(0, c);
                }
            }
        }
    }

    (stacks, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, instructions) = split_instructions(input);
    for inst in instructions {
        for _step in 0..inst.count {
            let c = stacks[inst.from - 1].pop().unwrap();
            stacks[inst.to - 1].push(c);
        }
    }
    let mut top_chars = Vec::new();
    for stack in stacks {
        top_chars.push(stack.last().unwrap().to_string());
    }
    Some(top_chars.join(""))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
