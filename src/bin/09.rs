use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Debug)]
#[display("{dir} {steps}")]
pub struct Movement {
    dir: char,
    steps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(a: &Position, b: &Position) -> (i32, i32) {
        ((a.x - b.x).abs(), (a.y - b.y).abs())
    }

    fn not_touching(a: &Position, b: &Position) -> bool {
        let (d_x, d_y) = Self::distance(a, b);
        d_x > 1 || d_y > 1
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    seen: HashSet<Position>,
    positions: Vec<Position>,
}

impl Rope {
    pub fn from_vec(pos: Vec<Position>) -> Self {
        Self {
            positions: pos,
            ..Default::default()
        }
    }

    pub fn move_one(&mut self, m: &Movement) {
        for _step in 0..m.steps {
            let mut head = self.positions.first().unwrap().clone();
            match m.dir {
                'U' => head.y += 1,
                'D' => head.y -= 1,
                'L' => head.x -= 1,
                'R' => head.x += 1,
                _ => (),
            }

            self.positions[0] = head;

            (0..self.positions.len())
                .map(|i| (i, i + 1))
                .for_each(|(a, b)| {
                    let head = self.positions.get(a).unwrap();
                    if let Some(tail) = self.positions.get(b) {
                        let mut tail = tail.clone();
                        if Position::not_touching(head, &tail) {
                            let (dx, dy) = Position::distance(head, &tail);
                            if dx >= 1 {
                                tail.x += (head.x - tail.x).clamp(-1, 1);
                            }
                            if dy >= 1 {
                                tail.y += (head.y - tail.y).clamp(-1, 1);
                            }
                        }
                        self.positions[b] = tail;
                    }
                });
            self.seen.insert(self.positions.last().unwrap().clone());
        }
    }
}

pub fn run_input(rope_length: usize, input: &str) -> Rope {
    let mut ht = Rope::from_vec(vec![Position::default(); rope_length]);
    for l in input.lines() {
        let m: Movement = l.parse().unwrap();
        ht.move_one(&m);
    }
    ht
}

pub fn part_one(input: &str) -> Option<usize> {
    let ht = run_input(2, input);
    Some(ht.seen.len())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_example() {
        let input = advent_of_code::read_file("examples", 9);
        let ht = run_input(2, &input);
        let mut seen = ht.seen.into_iter().collect_vec();
        seen.sort();
        assert_eq!(
            seen,
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 0 },
                Position { x: 2, y: 2 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 0 },
                Position { x: 3, y: 2 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 4 },
                Position { x: 4, y: 1 },
                Position { x: 4, y: 2 },
                Position { x: 4, y: 3 }
            ]
        )
    }

    #[test]
    fn test_lateral_movement() {
        let mut ht = Rope::from_vec(vec![Position { x: 1, y: 2 }, Position { x: 1, y: 3 }]);
        ht.move_one(&Movement { dir: 'D', steps: 1 });
        let tail = ht.positions.last().unwrap();
        assert_eq!(tail.y, 2);
        assert_eq!(tail.x, 1);
    }

    #[test]
    fn test_diagonal_movement() {
        let mut ht = Rope::from_vec(vec![Position { x: 2, y: 2 }, Position { x: 1, y: 1 }]);
        ht.move_one(&Movement { dir: 'U', steps: 1 });
        let tail = ht.positions.last().unwrap();
        assert_eq!(tail.y, 2);
        assert_eq!(tail.x, 2);
    }
}
