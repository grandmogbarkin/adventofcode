use cached::proc_macro::cached;
use regex::Regex;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "21"
    }

    fn test1_result(&self) -> i64 {
        739785
    }
    fn test2_result(&self) -> i64 {
        444356092776315
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let re = Regex::new(r"^Player ([\d]) starting position: ([\d])$").unwrap();

        let cap1 = re.captures(&input[0]).unwrap();
        let cap2 = re.captures(&input[1]).unwrap();

        let mut pos: Vec<usize> = vec![
            cap1[2].parse::<usize>().unwrap(),
            cap2[2].parse::<usize>().unwrap(),
        ];
        let mut scores: Vec<usize> = vec![0; 2];

        let mut player = 0;
        let mut d = 1;
        let mut count = 0;
        while scores[0] < 1000 && scores[1] < 1000 {
            count += 3;
            pos[player] += d + d + 1 + d + 2;
            pos[player] = ((pos[player] - 1) % 10) + 1;
            scores[player] += pos[player];
            player += 1;
            player %= 2;
            d += 2;
            d %= 100;
            d += 1;
        }
        let loser = std::cmp::min(scores[0], scores[1]);

        println!("{:?}: {}", scores, count);

        Ok(loser as i64 * count)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let re = Regex::new(r"^Player ([\d]) starting position: ([\d])$").unwrap();

        let cap1 = re.captures(&input[0]).unwrap();
        let cap2 = re.captures(&input[1]).unwrap();

        let pos_score: [(u128, u128); 2] = [
            (cap1[2].parse::<u128>().unwrap(), 0),
            (cap2[2].parse::<u128>().unwrap(), 0),
        ];

        let (p1_wins, p2_wins) = play_part_2(pos_score);
        println!("1: {} 2: {}", p1_wins, p2_wins);
        Ok(std::cmp::max(p1_wins, p2_wins) as i64)
    }
}

#[cached]
fn play_part_2(pos_score: [(u128, u128); 2]) -> (u128, u128) {
    // We only advance the player in position 2, swap the players on each call.
    // max score of 56 before u128 overflow
    if pos_score[1].1 >= 21 {
        return (0, 1);
    }
    const DIE_SUMS: [u128; 27] = [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];
    let mut p1_total_wins = 0;
    let mut p2_total_wins = 0;
    for sum in DIE_SUMS {
        // Advance player one
        let next_pos = ((pos_score[0].0 + sum - 1) % 10) + 1;
        let next_score = pos_score[0].1 + next_pos;
        // Swap players for next round
        let (p2_wins, p1_wins) = play_part_2([pos_score[1], (next_pos, next_score)]);
        p1_total_wins += p1_wins;
        p2_total_wins += p2_wins;
    }
    (p1_total_wins, p2_total_wins)
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
