use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "20"
    }

    fn test1_result(&self) -> i64 {
        35
    }
    fn test2_result(&self) -> i64 {
        3351
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, 2)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, 50)
    }
}

fn run(filename: String, steps: usize) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;

    // b'#' = 35 = 0b100011, b'.' = 46 = 0b101110
    let algo = input[0]
        .bytes()
        .map(|b| (b & 1) as usize)
        .collect::<Vec<_>>();

    let mut image: Vec<Vec<usize>> = vec![];
    for line in input.iter().skip(2) {
        image.push(line.bytes().map(|b| (b & 1) as usize).collect::<Vec<_>>());
    }

    // println!("{:?}", image);
    for step in 0..steps {
        let out_dim: usize = image.len() + 2;
        let mut out: Vec<Vec<usize>> = vec![vec![0; out_dim]; out_dim];
        for (i, row) in out.iter_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                *c = process(step, &algo, &image, i, j);
            }
        }

        image = out;
        // println!("{:?}", image);
    }

    Ok(image.iter().flatten().sum::<usize>() as i64)
}

fn process(step: usize, algo: &[usize], image: &[Vec<usize>], i: usize, j: usize) -> usize {
    let mut index: usize = 0;
    let max_dim = image.len() as i32;
    // If the first bit of our algo is a 1, then on every odd step, all the "off" bits in our
    //   infinite image will become "on". So pretend they're on.
    // This exercise doesn't make sense in this case if the last bit of the algo is a 1, or if
    //   we're doing an odd number of steps, as the number of "on" bits will be infinite. So
    //   we assume that there's an even number of steps, and that the last bit is off, which it
    //   happens to be in our input.
    let mut toggle: usize = 0;
    if algo[0] == 1 && (step % 2) == 1 {
        toggle = 1;
    }
    // 0,0 in out centers on -1, -1 of the image, so offset the 9 cells by -1.
    for (di, dj) in [
        (-2, -2),
        (-2, -1),
        (-2, 0),
        (-1, -2),
        (-1, -1),
        (-1, 0),
        (0, -2),
        (0, -1),
        (0, 0),
    ] {
        index <<= 1;
        let x = i as i32 + di;
        let y = j as i32 + dj;
        if x < 0 || y < 0 || x >= max_dim || y >= max_dim {
            index |= toggle;
            continue;
        }
        index |= image[x as usize][y as usize] as usize;
    }
    // println!("{}: {}", index, algo[index]);
    algo[index]
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
