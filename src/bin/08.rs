#![allow(unused_comparisons)]

use rayon::prelude::*;
use std::collections::BinaryHeap;

pub fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    let mut base = Vec::new();
    for l in input.lines() {
        let mut lbase = Vec::new();
        for c in l.chars() {
            lbase.push(c.to_digit(10).unwrap());
        }
        base.push(lbase);
    }
    base
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
struct DirIterator<'a> {
    direction: Direction,
    grid: &'a Vec<Vec<u32>>,
    x: usize,
    y: usize,
}

impl<'a> DirIterator<'a> {
    pub fn new(dir: Direction, x: usize, y: usize, grid: &'a Vec<Vec<u32>>) -> Self {
        Self {
            direction: dir,
            grid,
            x,
            y,
        }
    }
}

impl<'a> Iterator for DirIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::North => match self.y.checked_sub(1) {
                Some(new_y) => self.y = new_y,
                None => return None,
            },
            Direction::East => {
                self.x += 1;
            }
            Direction::South => {
                self.y += 1;
            }
            Direction::West => match self.x.checked_sub(1) {
                Some(new_x) => self.x = new_x,
                None => return None,
            },
        };
        self.grid.get(self.y).and_then(|r| r.get(self.x)).cloned()
    }
}

pub fn tree_is_visible(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
    let tree_height = grid[y][x];
    use Direction::*;
    for dir in [North, East, South, West] {
        let mut iter = DirIterator::new(dir, x, y, grid);
        //println!("{:#?}", iter);
        if iter.all(|h| h < tree_height) {
            return true;
        }
    }
    false
}

pub fn score_for_dir(x: usize, y: usize, grid: &Vec<Vec<u32>>, dir: Direction) -> usize {
    let tree_height = grid[y][x];
    let iter = DirIterator::new(dir, x, y, grid);
    let mut trees = 0_usize;
    for t in iter {
        trees += 1;
        if t >= tree_height {
            break;
        }
    }
    trees
}

pub fn scenic_score(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> usize {
    use Direction::*;

    vec![North, West, East, South]
        .par_iter()
        .map(|dir| score_for_dir(x, y, grid, *dir))
        .reduce(|| 1, |l, r| l * r)
}

pub fn get_visible_trees(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let coords: Vec<(usize, usize, &Vec<Vec<u32>>)> = (0..height)
        .flat_map(move |row| (0..width).map(move |col| (row, col, grid)))
        .collect();

    coords
        .par_iter()
        .filter_map(|(y, x, grid)| {
            if tree_is_visible(*x, *y, grid) {
                Some((*x, *y))
            } else {
                None
            }
        })
        .collect()
}

pub fn calc_scenic_scores(grid: &Vec<Vec<u32>>) -> BinaryHeap<usize> {
    let height = grid.len();
    let width = grid[0].len();
    let coords: Vec<(usize, usize, &Vec<Vec<u32>>)> = (0..height)
        .flat_map(move |row| (0..width).map(move |col| (row, col, grid)))
        .collect();

    let mut heap = BinaryHeap::new();
    let scores: Vec<usize> = coords
        .par_iter()
        .map(|(y, x, grid)| scenic_score(*x, *y, grid))
        .collect();
    for s in scores {
        heap.push(s);
    }
    heap
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_grid(input);
    let visible_trees = get_visible_trees(&grid);
    Some(visible_trees.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_grid(input);
    let mut scenic_scores = calc_scenic_scores(&grid);
    scenic_scores.pop()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }

    #[test]
    fn test_scenic_score() {
        let input = advent_of_code::read_file("examples", 8);
        let grid = parse_grid(&input);
        assert_eq!(score_for_dir(2, 3, &grid, Direction::North), 2);
    }

    #[test]
    fn test_dir_iterator() {
        let input = advent_of_code::read_file("examples", 8);
        let grid = parse_grid(&input);
        let iter = DirIterator::new(Direction::North, 2, 3, &grid);
        let v = iter.collect_vec();
        assert_eq!(v, vec![3, 5, 3]);
    }
}
