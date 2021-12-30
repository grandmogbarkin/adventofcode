use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

#[derive(Clone, PartialEq, Debug)]
enum Cucumber {
    Empty,
    East,
    South,
}

#[derive(Debug)]
struct SeaBed {
    floor: Vec<Vec<Cucumber>>,
}

impl SeaBed {
    pub fn new() -> Self {
        SeaBed { floor: vec![] }
    }

    pub fn add_row(&mut self, row: &str) {
        let idx = self.floor.len();
        self.floor.push(vec![]);
        for c in row.chars() {
            match c {
                '>' => {
                    self.floor[idx].push(Cucumber::East);
                }
                'v' => {
                    self.floor[idx].push(Cucumber::South);
                }
                '.' => {
                    self.floor[idx].push(Cucumber::Empty);
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn step(&mut self) -> i64 {
        let mut moves = 0;

        let mut floor_new = self.floor.clone();
        for (x, row) in self.floor.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                if *c != Cucumber::East {
                    continue;
                }
                let next_y = if y + 1 < row.len() { y + 1 } else { 0 };
                if self.floor[x][next_y] == Cucumber::Empty {
                    moves += 1;
                    floor_new[x][y] = Cucumber::Empty;
                    floor_new[x][next_y] = Cucumber::East;
                }
            }
        }

        self.floor = floor_new.clone();

        for (x, row) in self.floor.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                if *c != Cucumber::South {
                    continue;
                }
                let next_x = if x + 1 < self.floor.len() { x + 1 } else { 0 };
                if self.floor[next_x][y] == Cucumber::Empty {
                    moves += 1;
                    floor_new[x][y] = Cucumber::Empty;
                    floor_new[next_x][y] = Cucumber::South;
                }
            }
        }

        self.floor = floor_new.clone();
        moves
    }
}

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "25"
    }

    fn test1_result(&self) -> i64 {
        58
    }
    fn test2_result(&self) -> i64 {
        2
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut bed = SeaBed::new();

        for line in input {
            bed.add_row(&line);
        }

        let mut steps = 0;
        while bed.step() > 0 {
            steps += 1;
            // if steps % 10 == 0   {
            //     println!("{}: {:?}", steps, bed)
            // }
        }

        Ok(steps + 1)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let _test = read::read_lines(filename)?;

        println!("Hello 2!");
        Ok(2)
    }
}

pub fn main() {
    let d = Solution {};
    Exercise::run(&d, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_working() {
        let d = Solution {};
        Exercise::run(&d, true)
    }
}
