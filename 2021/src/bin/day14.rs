use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "14"
    }

    fn test1_result(&self) -> i64 {
        1588
    }
    fn test2_result(&self) -> i64 {
        2188189693529
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, 10)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, 40)
    }
}

fn run(filename: String, steps: usize) -> Result<i64, Box<dyn Error>> {
    const N: usize = 26;

    let input = read::read_lines(filename)?;

    let template = input[0]
        .bytes()
        .map(|b| (b - b'A') as usize)
        .collect::<Vec<_>>();

    // let mut rules: HashMap<usize, usize> = HashMap::new();
    let mut rules = [[0; N]; N];
    for i in 2..input.len() {
        let mut input_split = input[i].split(" -> ");
        let key_in = input_split
            .next()
            .unwrap()
            .bytes()
            .map(|b| (b - b'A') as usize)
            .collect::<Vec<_>>();
        let val = input_split
            .next()
            .unwrap()
            .bytes()
            .map(|b| (b - b'A') as usize)
            .collect::<Vec<_>>()[0];
        rules[key_in[0]][key_in[1]] = val;
    }

    // println!("{:?}", rules);

    let mut curr_template = [[0_usize; N]; N];
    for i in 1..template.len() {
        curr_template[template[i - 1]][template[i]] += 1;
    }

    for _ in 0..steps {
        let mut next_template = [[0_usize; N]; N];
        curr_template
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, &n)| n > 0)
            .for_each(|(i, &n)| {
                let intermediate = rules[i / N][i % N];
                next_template[i / N][intermediate] += n;
                next_template[intermediate][i % N] += n;
            });
        curr_template = next_template;
    }

    let mut freq_map = [0_usize; N];
    curr_template
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, &n)| n > 0)
        .for_each(|(i, &x)| {
            freq_map[i / N] += x;
            freq_map[i % N] += x;
        });

    // first and last letters were not counted
    freq_map[*template.first().unwrap()] += 1;
    freq_map[*template.last().unwrap()] += 1;

    let min_freq = freq_map.iter().filter(|&x| *x > 0).min().unwrap();
    let max_freq = freq_map.iter().max().unwrap();

    println!("{:?}", freq_map);
    Ok((max_freq - min_freq) as i64 / 2)
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
