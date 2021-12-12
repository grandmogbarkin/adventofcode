use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "10"
    }

    fn test1_result(&self) -> i64 {
        26397
    }
    fn test2_result(&self) -> i64 {
        288957
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut syntax_error_score: i64 = 0;
        for line in input.iter() {
            let (score, _) = find_invalid_chunk(line)?;
            syntax_error_score += score;
        }
        Ok(syntax_error_score)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut completion_scores: Vec<i64> = vec![];
        for line in input.iter() {
            let (syntax_error_score, open_chunks) = find_invalid_chunk(line)?;
            if syntax_error_score > 0 {
                continue;
            }
            // println!("Open chunks: {:?}", open_chunks);
            let mut score: i64 = 0;
            for open_chunk in open_chunks.iter().rev() {
                match open_chunk {
                    '(' => {
                        score *= 5;
                        score += 1
                    }
                    '[' => {
                        score *= 5;
                        score += 2
                    }
                    '{' => {
                        score *= 5;
                        score += 3
                    }
                    '<' => {
                        score *= 5;
                        score += 4
                    }
                    _ => simple_error::bail!(format!("Invalid open chunk: {}", open_chunk)),
                }
            }
            completion_scores.push(score);
        }
        println!("Found scores: {:?}", completion_scores);
        if (completion_scores.len() % 2) == 0 {
            simple_error::bail!(format!(
                "Invalid number of completion scores: {}",
                completion_scores.len()
            ));
        }
        completion_scores.sort();
        Ok(completion_scores[(completion_scores.len() - 1) / 2])
    }
}

// A return value of > 0 means invalid chunks
// A return value of 0 means valid. If open_chunks.len() > 0, then incomplete.
fn find_invalid_chunk(line: &String) -> Result<(i64, Vec<char>), Box<dyn Error>> {
    let mut score: i64 = 0;
    let mut open_chunks: Vec<char> = vec![];
    let chunks: Vec<char> = line.chars().collect();
    for chunk in chunks.iter() {
        match chunk {
            '(' | '[' | '{' | '<' => open_chunks.push(*chunk),
            ')' => {
                if open_chunks.pop() != Some('(') {
                    score = 3
                }
            }
            ']' => {
                if open_chunks.pop() != Some('[') {
                    score = 57
                }
            }
            '}' => {
                if open_chunks.pop() != Some('{') {
                    score = 1197
                }
            }
            '>' => {
                if open_chunks.pop() != Some('<') {
                    score = 25137
                }
            }
            _ => simple_error::bail!(format!("Invalid character: {}", chunk)),
        }
        if score > 0 {
            break;
        }
    }
    Ok((score, open_chunks))
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
