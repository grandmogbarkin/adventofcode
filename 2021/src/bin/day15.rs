use pathfinding::astar;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "15"
    }

    fn test1_result(&self) -> i64 {
        40
    }
    fn test2_result(&self) -> i64 {
        315
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, 1)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, 5)
    }
}

fn run(filename: String, multiplier: usize) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;
    let cave: Vec<Vec<usize>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|e| e.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let max_dim = cave.len();
    let full_dim = max_dim * multiplier;

    // let mut full_cave: Vec<Vec<usize>> = vec![vec![0; full_dim]; full_dim];
    //
    // for i in 0..(multiplier*multiplier) {
    //     let row_offset = i / multiplier;
    //     let col_offset = i % multiplier;
    //     for (y, r) in cave.iter().enumerate() {
    //         for (x, v) in r.iter().enumerate() {
    //             let new_y = y + (max_dim * row_offset);
    //             let new_x = x + (max_dim * col_offset);
    //             let new_v = *v + row_offset + col_offset;
    //             full_cave[new_y][new_x] = ((new_v - 1) % 9) + 1;
    //         }
    //     }
    // }
    // println!("{:?}", cave);
    // println!("{:?}", full_cave);
    
    let goal: Pos = Pos(full_dim - 1, full_dim - 1);
    println!("{:?}", goal);
    let result = astar(
        &Pos(0, 0),
        |p| p.neighbours(&cave, full_dim),
        |p| (full_dim - p.0) + (full_dim - p.1),
        |p| *p == goal,
    );
    // println!("{:?}", result);
    Ok(result.expect("no path found").1 as i64)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours(&self, cave: &Vec<Vec<usize>>, full_dim: usize) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut out: Vec<Pos> = vec![];
        if x > 0 {
            out.push(Pos(x - 1, y));
        }
        if x + 1 < full_dim {
            out.push(Pos(x + 1, y));
        }
        if y > 0 {
            out.push(Pos(x, y - 1));
        }
        if y + 1 < full_dim {
            out.push(Pos(x, y + 1));
        }
        let res = out
            .into_iter()
            .map(|p| {
                let max_dim = cave.len();
                let full_pos = Pos(p.0 % max_dim, p.1 % max_dim);
                let val_offset = (p.0 / max_dim) + (p.1 / max_dim);
                let v = cave[full_pos.0][full_pos.1] + val_offset;
                (p, ((v - 1) % 9) + 1)
            })
            .collect();
        // println!("{:?}: {:?}", self, res);
        res
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
