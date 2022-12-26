use std::cell::{Ref, RefCell};
use std::rc::Rc;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Debug, Hash)]
#[display("{size} {name}")]
pub struct File {
    name: String,
    size: usize,
}

#[derive(Display, FromStr, Debug, Default)]
#[display("dir {name}")]
#[from_str(default)]
pub struct Directory {
    name: String,
    size: usize,
    subdirs: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    fn from_name(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn calc_size(&mut self) {
        self.size = 0;
        for d in self.subdirs.iter() {
            let mut d = d.borrow_mut();
            d.calc_size();
            self.size += d.size;
        }
        for f in self.files.iter() {
            self.size += f.size;
        }
    }

    fn filter_subdirs<F>(&self, filter_fn: &F, recurse: bool) -> Vec<Rc<RefCell<Directory>>>
    where
        F: Fn(Ref<Directory>) -> bool,
    {
        let mut matching = Vec::new();
        for subd in self.subdirs.iter() {
            if recurse {
                matching.extend(subd.borrow().filter_subdirs(filter_fn, recurse));
            }
            if filter_fn(subd.borrow()) {
                matching.push(Rc::clone(subd));
            }
        }
        matching
    }

    fn add_subdir(&mut self, subd: Directory) {
        self.subdirs.push(Rc::new(RefCell::new(subd)));
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn get_mut_subdir(&self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        self.subdirs
            .iter()
            .find(|d| d.borrow().name.eq_ignore_ascii_case(name))
            .cloned()
    }
}

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
pub enum Command {
    #[display("$ cd {0}")]
    CD(String),
    #[display("$ ls")]
    LS,
    #[display("{size} {name}")]
    FILE { size: usize, name: String },
    #[display("dir {0}")]
    DIR(String),
}

#[derive(Debug)]
pub struct DirStack {
    stack: Vec<Rc<RefCell<Directory>>>,
    root: Rc<RefCell<Directory>>,
    current: Rc<RefCell<Directory>>,
}

impl DirStack {
    pub fn new() -> Self {
        let root = Directory::default();
        let cell = Rc::new(RefCell::new(root));
        DirStack {
            stack: vec![cell.clone()],
            root: cell.clone(),
            current: cell,
        }
    }

    pub fn cd(&mut self, name: &str) {
        if name == "/" {
            loop {
                if self.current.borrow().name.is_empty() {
                    break;
                }
                self.stack.pop();
                self.current = self.stack.last().unwrap().clone();
            }
        } else if name == ".." {
            self.stack.pop();
            self.current = self.stack.last().unwrap().clone();
        } else {
            let next = self.current.borrow().get_mut_subdir(name).unwrap();
            self.current = Rc::clone(&next);
            self.stack.push(next);
        }
    }

    pub fn add_dir(&self, dir: Directory) {
        self.current.borrow_mut().add_subdir(dir);
    }

    pub fn add_file(&self, file: File) {
        self.current.borrow_mut().add_file(file)
    }
}

impl Default for DirStack {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_commands(input: &str) -> Directory {
    let mut stack = DirStack::new();
    for l in input.lines() {
        //println!("{:#?}", stack);
        //println!("LINE :> {}", l);
        if let Ok(cmd) = l.parse::<Command>() {
            match cmd {
                Command::CD(dir) => {
                    stack.cd(&dir);
                }
                Command::DIR(name) => {
                    let new_dir = Directory::from_name(&name);
                    stack.add_dir(new_dir);
                }
                Command::FILE { size, name } => {
                    let new_file = File { size, name };
                    stack.add_file(new_file);
                }
                Command::LS => {}
            }
        } else {
            panic!("Could not parse line {}", l);
        }
    }
    let mut root = stack.root.take();
    root.calc_size();
    root
}

pub fn part_one(input: &str) -> Option<usize> {
    let root = parse_commands(input);
    let sub100k: Vec<Rc<RefCell<Directory>>> = root
        .filter_subdirs(&|d| d.size <= 100000, true)
        .into_iter()
        .collect();
    //println!("Sub: {:#?}", &sub100k);
    let subsizes = sub100k.iter().map(|d| d.borrow().size).sum();
    Some(subsizes)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
