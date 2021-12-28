extern crate rayon; // 1.0.3
use cached::proc_macro::cached;
use rayon::prelude::*;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

static CORRCOST: [[i64; 7]; 4] = [
    [2, 1, 1, 3, 5, 7, 8],
    [4, 3, 1, 1, 3, 5, 6],
    [6, 5, 3, 1, 1, 3, 4],
    [8, 7, 5, 3, 1, 1, 2],
];
static COST: [i64; 4] = [1, 10, 100, 1000];
static IDX: [[[i64; 4]; 7]; 4] = [
    [
        [0, 0, 0, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 2],
        [0, 0, 2, 3],
        [0, 2, 3, 4],
        [2, 3, 4, 5],
    ],
    [
        [0, 0, 1, 2],
        [0, 0, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 3],
        [0, 0, 3, 4],
        [0, 3, 4, 5],
    ],
    [
        [0, 1, 2, 3],
        [0, 0, 2, 3],
        [0, 0, 0, 3],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 4],
        [0, 0, 4, 5],
    ],
    [
        [1, 2, 3, 4],
        [0, 2, 3, 4],
        [0, 0, 3, 4],
        [0, 0, 0, 4],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 5],
    ],
];

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "23"
    }

    fn test1_result(&self) -> i64 {
        1
    }
    fn test2_result(&self) -> i64 {
        2
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let _test = read::read_lines(filename)?;

        println!("Hello 1!");
        Ok(1)
    }

    fn task_2(&self, _filename: String) -> Result<i64, Box<dyn Error>> {
        let input: [[i64; 4]; 4] = [[0, 2, 2, 3], [3, 2, 1, 0], [3, 1, 0, 2], [1, 3, 0, 1]];
        let corridor = [-1, -1, -1, -1, -1, -1, -1];

        Ok(run(input, corridor, 0))
    }
}

fn is_solved(input: [[i64; 4]; 4]) -> bool {
    for row in input.iter() {
        for (i, a) in row.iter().enumerate() {
            if i as i64 != *a {
                return false;
            }
        }
    }
    return true;
}

#[cached]
fn run(input: [[i64; 4]; 4], corridor: [i64; 7], initial_cost: i64) -> i64 {
    // println!("{:?} {:?}", input, corridor);
    if is_solved(input) {
        return initial_cost;
    }

    let mut output = input.clone();
    let mut cor_out = corridor.clone();
    let mut final_cost = initial_cost;

    let mut can_move = true;
    while can_move {
        can_move = false;
        for (i, a) in corridor.iter().enumerate() {
            if a < &0 {
                continue;
            }
            can_move = true;
            for c in IDX[*a as usize][i] {
                if c == 0 {
                    continue;
                }
                if corridor[c as usize] >= 0 {
                    can_move = false;
                }
            }
            for h in input[*a as usize].iter().rev() {
                if h >= &0 && h != a {
                    can_move = false;
                }
            }
            if can_move {
                // do move
                cor_out[i] = -1;
                let mut depth = 4;
                for h in output[*a as usize].iter_mut().rev() {
                    if h < &mut 0 {
                        *h = *a;
                        break;
                    }
                    depth -= 1;
                }
                final_cost += (CORRCOST[*a as usize][i] + depth) * COST[*a as usize];
                // return (output, cor_out, final_cost);
            }
        }
    }

    // let mut final_out = output.clone();
    // let mut final_cor = cor_out.clone();
    let mut min_cost = i64::MAX;
    for (i, row) in input.iter().enumerate() {
        for (j, a) in row.iter().enumerate() {
            if a < &0 {
                continue;
            }
            if *a == j as i64 {
                let mut can_move = true;
                for r in input.iter().take(i) {
                    if r[j] >= 0 {
                        can_move = false;
                    }
                }
                let mut is_final = true;
                for r in input.iter().skip(i + 1) {
                    if r[j] != *a {
                        is_final = false;
                    }
                }
                if !can_move || is_final {
                    continue;
                }
            }
            let res = corridor
                .par_iter()
                .enumerate()
                .map(|(c_i, c_a)| {
                    if c_a >= &0 {
                        return i64::MAX;
                    }
                    let mut is_blocked = false;
                    for c in IDX[*a as usize][c_i] {
                        if c == 0 {
                            continue;
                        }
                        if corridor[c as usize] >= 0 {
                            is_blocked = true;
                        }
                    }
                    if is_blocked {
                        return i64::MAX;
                    }
                    // can move to c_i
                    let mut tmp_out = output.clone();
                    let mut tmp_cor = cor_out.clone();
                    tmp_out[i][j] = -1;
                    tmp_cor[c_i] = *a;
                    let new_cost = final_cost
                        + ((CORRCOST[*a as usize][c_i] + i as i64 + 1) * COST[*a as usize]);
                    run(tmp_out, tmp_cor, new_cost)
                })
                .min().unwrap();
            if res < min_cost {
                min_cost = res;
            }
        }
    }

    min_cost
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
