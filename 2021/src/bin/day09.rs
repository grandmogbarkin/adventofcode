use std::cmp::min;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "09"
    }

    fn test1_result(&self) -> i64 {
        15
    }
    fn test2_result(&self) -> i64 {
        1134
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let heightmap = build_heightmap(&input);

        let mut risk_level: i64 = 0;
        for i in 0..heightmap.len() {
            for j in 0..heightmap[i].len() {
                if is_low_point(&heightmap, i, j) {
                    risk_level += heightmap[i][j] + 1;
                }
            }
        }

        Ok(risk_level)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut heightmap = build_heightmap(&input);

        let mut basin_size: Vec<i64> = vec![];
        for i in 0..heightmap.len() {
            for j in 0..heightmap[i].len() {
                // if is_low_point(&heightmap, i, j) {
                // Just calculate the basin starting at every point.
                // The method will flood each basin it finds, so they'll only be counted once.
                // A few 0s at the beginning will soon be pushed out.
                let cur_basin_size = basin_count(&mut heightmap, i, j);
                if basin_size.len() < 3 {
                    basin_size.push(cur_basin_size);
                } else {
                    let min_basin = *basin_size.iter().min().unwrap();
                    if min_basin >= cur_basin_size {
                        continue;
                    }
                    for i in 0..basin_size.len() {
                        if basin_size[i] == min_basin {
                            basin_size[i] = cur_basin_size;
                            break;
                        }
                    }
                }
                println!("Counted basin: {}, {:?}", cur_basin_size, basin_size);
                // }
            }
        }

        // println!("{:?}", heightmap);
        Ok(basin_size[0] * basin_size[1] * basin_size[2])
    }
}

fn build_heightmap(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut heightmap: Vec<Vec<i64>> = vec![];
    const RADIX: u32 = 10;
    for line in input.iter() {
        heightmap.push(
            line.chars()
                .map(|s| s.to_digit(RADIX).unwrap() as i64)
                .collect(),
        );
    }
    heightmap
}

fn is_low_point(heightmap: &Vec<Vec<i64>>, i: usize, j: usize) -> bool {
    let mut min_point = 9;

    // Check every neighbor, see if we're the lowest point
    if i > 0 {
        min_point = min(min_point, heightmap[i - 1][j]);
    }
    if i < heightmap.len() - 1 {
        min_point = min(min_point, heightmap[i + 1][j]);
    }
    if j > 0 {
        min_point = min(min_point, heightmap[i][j - 1]);
    }
    if j < heightmap[i].len() - 1 {
        min_point = min(min_point, heightmap[i][j + 1]);
    }

    min_point > heightmap[i][j]
}

fn basin_count(heightmap: &mut Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    // usize indices - can't compare < 0, check later.
    if i >= heightmap.len() {
        return 0;
    }
    if j >= heightmap[i].len() {
        return 0;
    }
    if heightmap[i][j] == 9 {
        return 0;
    }
    // println!("Counting basin at {} {}", i, j);
    heightmap[i][j] = 9;
    1 + if i > 0 {
        basin_count(heightmap, i - 1, j)
    } else {
        0
    } + if j > 0 {
        basin_count(heightmap, i, j - 1)
    } else {
        0
    } + basin_count(heightmap, i + 1, j)
        + basin_count(heightmap, i, j + 1)
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
