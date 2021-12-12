use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "11"
    }

    fn test1_result(&self) -> i64 {
        1656
    }
    fn test2_result(&self) -> i64 {
        195
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let mut octopuses = get_octopus_map(filename)?;

        let mut flashes: i64 = 0;
        for _ in 0..100 {
            octopuses
                .iter_mut()
                .for_each(|r| r.iter_mut().for_each(|e| *e += 1));
            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    flashes += flash(&mut octopuses, i, j);
                }
            }
            octopuses.iter_mut().for_each(|r| {
                r.iter_mut().for_each(|e| {
                    if *e > 9 {
                        *e = 0
                    }
                })
            });
        }

        Ok(flashes)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let mut octopuses = get_octopus_map(filename)?;

        let mut step: i64 = 0;
        loop {
            step += 1;
            octopuses
                .iter_mut()
                .for_each(|r| r.iter_mut().for_each(|e| *e += 1));
            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    if flash(&mut octopuses, i, j) == 100 {
                        return Ok(step);
                    }
                }
            }
            octopuses.iter_mut().for_each(|r| {
                r.iter_mut().for_each(|e| {
                    if *e > 9 {
                        *e = 0
                    }
                })
            });
        }

        // simple_error::bail!("It's a miracle, we exited an infinite loop.");
    }
}

fn flash(octopuses: &mut Vec<Vec<usize>>, i: usize, j: usize) -> i64 {
    // Ignore any octopuses that won't fire this step
    if octopuses[i][j] <= 9 {
        return 0;
    };
    // Octopuses fire at 10, and fire all their neighbors at 11.
    // If this one is at 11, it's already fired its neighbors.
    if octopuses[i][j] >= 11 {
        return 0;
    };
    octopuses[i][j] += 1; // About to fire the neighbors, make octopus 11.
    let affected = get_affected(&octopuses, i, j);
    for (x, y) in &affected {
        if octopuses[*x][*y] <= 9 {
            octopuses[*x][*y] += 1; // bring them up to 10
        }
    }
    let mut flashes: i64 = 1;
    // Recurse.
    for (x, y) in &affected {
        flashes += flash(octopuses, *x, *y);
    }
    flashes
}

fn get_affected(octopuses: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut affected: Vec<(usize, usize)> = vec![];
    let mut rows: Vec<usize> = vec![i];
    let mut cols: Vec<usize> = vec![j];
    if i > 0 {
        rows.push(i - 1);
    }
    if i < octopuses.len() - 1 {
        rows.push(i + 1);
    }
    if j > 0 {
        cols.push(j - 1);
    }
    if j < octopuses[i].len() - 1 {
        cols.push(j + 1);
    }
    for x in &rows {
        for y in &cols {
            if (*x != i) || (*y != j) {
                affected.push((*x, *y));
            }
        }
    }
    affected
}

fn get_octopus_map(filename: String) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let input = read::read_lines(filename)?;
    Ok(input
        .iter()
        .map(|line| {
            line.chars()
                .map(|e| e.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect())
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
