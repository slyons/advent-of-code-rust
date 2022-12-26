pub fn find_sopm(input: &str) -> usize {
    let mut idx: usize = 0;
    let mut cbuf: [char; 4] = ['!'; 4];
    'outer: for c in input.trim().chars() {
        cbuf.copy_within(1..=3, 0);
        cbuf[3] = c;
        idx += 1;
        if idx >= 4 {
            for i in 0..4 {
                for j in 0..4 {
                    if i != j && cbuf[i] == cbuf[j] {
                        continue 'outer;
                    }
                }
            }
            return idx;
        }
    }
    panic!("No SOP marker found")
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(find_sopm(input))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
