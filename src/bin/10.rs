use itertools::Itertools;
use parse_display::{Display, FromStr};

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[derive(Debug)]
struct CPU {
    X: i64,
    IC: usize,
    instructions: Vec<Instruction>,
    cycle: usize,
    icc: usize,
}

impl CPU {
    pub fn new(insts: Vec<Instruction>) -> CPU {
        let first = insts[0];
        CPU {
            X: 1,
            IC: 0,
            instructions: insts,
            cycle: 0,
            icc: CPU::cycle_count(&first),
        }
    }

    fn cycle_count(inst: &Instruction) -> usize {
        match inst {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn on_finish(&mut self) {
        match self.instructions[self.IC] {
            Instruction::AddX(x) => {
                println!("CYCLE: {}, X: {} -> {}", self.cycle, self.X, self.X + x);
                self.X += x
            }
            Instruction::NoOp => (),
        }
    }

    pub fn step(&mut self) -> bool {
        if self.IC >= self.instructions.len() {
            return false;
        }
        self.cycle += 1;
        self.icc -= 1;
        if self.icc == 0 {
            self.on_finish();
            self.IC += 1;
            if let Some(inst) = self.instructions.get(self.IC) {
                self.icc = CPU::cycle_count(inst);
            }
        }
        true
    }
}

#[derive(Display, FromStr, Debug, Copy, Clone)]
pub enum Instruction {
    #[display("noop")]
    NoOp,

    #[display("addx {0}")]
    AddX(i64),
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if !l.is_empty() {
                Some(l.parse::<Instruction>().unwrap())
            } else {
                None
            }
        })
        .collect_vec()
}

pub fn sample_cycles(input: &str, mut cycles: Vec<usize>) -> Vec<i64> {
    let insts = parse_instructions(input);

    let mut cpu = CPU::new(insts);
    let mut output = Vec::<i64>::new();
    cycles.reverse();
    let mut sample_cycle = cycles.pop().unwrap();
    while let true = cpu.step() {
        if cpu.cycle == sample_cycle - 1 {
            println!("CYCLE: {}, X: {}", cpu.cycle, cpu.X);
            output.push(sample_cycle as i64 * cpu.X);
            if let Some(next_cycle) = cycles.pop() {
                sample_cycle = next_cycle;
            } else {
                break;
            }
        }
    }
    output
}

pub fn part_one(input: &str) -> Option<i64> {
    let cycles = sample_cycles(input, vec![20, 60, 100, 140, 180, 220]);
    println!("{:#?}", cycles);
    Some(cycles.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_small() {
        let input = r#"noop
        addx 3
        addx -5"#;
        let insts = parse_instructions(&input);
        let mut cpu = CPU::new(insts);

        let r = cpu.step();
        // End of first cycle
        assert_eq!(r, true);
        assert_eq!(cpu.cycle, 1);
        assert_eq!(cpu.X, 1);
        assert_eq!(cpu.IC, 1);

        cpu.step();
        // End of second cycle
        assert_eq!(cpu.X, 1);
        assert_eq!(cpu.IC, 1);
        assert_eq!(cpu.cycle, 2);
        assert_eq!(cpu.icc, 1);

        cpu.step();
        // End of third cycle
        assert_eq!(cpu.X, 4);
        assert_eq!(cpu.IC, 2);
        assert_eq!(cpu.cycle, 3);
        assert_eq!(cpu.icc, 2);

        cpu.step();
        // End of fourth cycle
        assert_eq!(cpu.X, 4);
        assert_eq!(cpu.IC, 2);
        assert_eq!(cpu.cycle, 4);
        assert_eq!(cpu.icc, 1);

        cpu.step();
        // End of fifth cycle
        assert_eq!(cpu.X, -1);
        assert_eq!(cpu.IC, 3);
        assert_eq!(cpu.cycle, 5);
        assert_eq!(cpu.icc, 0);
    }
}
