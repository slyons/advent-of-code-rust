pub fn find_sopm(buffer_size: usize, input: &str) -> usize {
    let mut idx: usize = 0;
    let mut cbuf = vec!['!'; buffer_size];
    'outer: for c in input.trim().chars() {
        cbuf.copy_within(1..=(buffer_size - 1), 0);
        cbuf[(buffer_size - 1)] = c;
        idx += 1;
        if idx >= buffer_size {
            for i in 0..buffer_size {
                for j in 0..buffer_size {
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
    Some(find_sopm(4, input))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(find_sopm(14, input))
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
