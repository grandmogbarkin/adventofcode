use std::cmp::max;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "13"
    }

    fn test1_result(&self) -> i64 {
        17
    }
    fn test2_result(&self) -> i64 {
        3072
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut board: Vec<Vec<bool>> = vec![];
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        for line in input.iter() {
            if line == "" {
                break;
            }
            let coords: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
            if coords.len() != 2 {
                simple_error::bail!(format!("Invalid coordinate input: {:?}", coords));
            }
            max_x = max(max_x, coords[0] + 1);
            max_y = max(max_y, coords[1] + 1);

            if max_y > board.len() {
                board.resize(max_y, vec![false; max_x]);
            }
            if max_x > board[0].len() {
                for col in board.iter_mut() {
                    col.resize(max_x, false);
                }
            }
            board[coords[1]][coords[0]] = true;
        }
        // println!("Board: {:?}", board);
        let mut first_fold: String = "".to_string();
        for line in input.iter().rev() {
            if line == "" {
                break;
            }
            first_fold = line.split(" along ").collect::<Vec<&str>>()[1].to_string();
        }
        let mut fold_split = first_fold.split("=");
        let fold: (&str, usize) = (
            fold_split.next().unwrap(),
            fold_split.next().unwrap().parse().unwrap(),
        );
        println!("{:?}", fold);

        match fold.0 {
            "y" => {
                let mut offset = (max_y - 1 - fold.1) * 2;
                for y in (fold.1 + 1..max_y).rev() {
                    for x in 0..max_x {
                        board[y - offset][x] |= board[y][x];
                        board[y][x] = false;
                    }
                    offset -= 2;
                }
            }
            "x" => {
                let mut offset = (max_x - 1 - fold.1) * 2;
                for x in (fold.1 + 1..max_x).rev() {
                    for y in 0..max_y {
                        board[y][x - offset] |= board[y][x];
                        board[y][x] = false;
                    }
                    offset -= 2;
                }
            }
            _ => simple_error::bail!(format!("Invalid fold direction: {}", fold.0)),
        }
        // println!("Folded board: {:?}", board);
        let mut dot_count: i64 = 0;
        for i in board.iter().flatten() {
            if *i {
                dot_count += 1;
            }
        }
        Ok(dot_count)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut board: Vec<Vec<bool>> = vec![];
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        for line in input.iter() {
            if line == "" {
                break;
            }
            let coords: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
            if coords.len() != 2 {
                simple_error::bail!(format!("Invalid coordinate input: {:?}", coords));
            }
            max_x = max(max_x, coords[0] + 1);
            max_y = max(max_y, coords[1] + 1);

            if max_y > board.len() {
                board.resize(max_y, vec![false; max_x]);
            }
            if max_x > board[0].len() {
                for col in board.iter_mut() {
                    col.resize(max_x, false);
                }
            }
            board[coords[1]][coords[0]] = true;
        }
        let mut folds: Vec<(String, usize)> = vec![];
        for line in input.iter().rev() {
            if line == "" {
                break;
            }
            let fold = line.split(" along ").collect::<Vec<&str>>()[1].to_string();
            let mut fold_split = fold.split("=");
            folds.push((
                fold_split.next().unwrap().to_string(),
                fold_split.next().unwrap().parse().unwrap(),
            ))
        }
        folds.reverse();
        println!("{:?}", folds);

        for fold in folds {
            match fold.0.as_str() {
                "y" => {
                    let mut offset = (max_y - 1 - fold.1) * 2;
                    // println!("max_y: {}, y fold: {}, {}", max_y, fold.1, offset);
                    for y in (fold.1 + 1..max_y).rev() {
                        for x in 0..max_x {
                            board[y - offset][x] |= board[y][x];
                            board[y][x] = false;
                        }
                        offset -= 2;
                    }
                    max_y = fold.1;
                    board.truncate(fold.1);
                }
                "x" => {
                    let mut offset = (max_x - 1 - fold.1) * 2;
                    // println!("max x: {}, x fold: {}, {}", max_x, fold.1, offset);
                    for x in (fold.1 + 1..max_x).rev() {
                        for y in 0..max_y {
                            board[y][x - offset] |= board[y][x];
                            board[y][x] = false;
                        }
                        offset -= 2;
                    }
                    for y in 0..max_y {
                        board[y].truncate(fold.1);
                    }
                    max_x = fold.1;
                }
                _ => simple_error::bail!(format!("Invalid fold direction: {}", fold.0)),
            }
        }
        for y in board.iter() {
            for x in y {
                if *x {
                    eprint!("#")
                } else {
                    eprint!(".")
                }
            }
            eprint!("\n");
        }
        let mut dot_count: i64 = 0;
        let mut dot_sum: usize = 0;
        for (i, v) in board.iter().flatten().enumerate() {
            if *v {
                dot_count += 1;
                dot_sum += i;
            }
        }
        Ok(dot_count * dot_sum as i64)
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
