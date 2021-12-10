use std::error::Error as Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "10" }

  fn test1_result(&self) -> i64 { 26397 }
  fn test2_result(&self) -> i64 { 288957 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;

    let mut syntax_error_score: i64 = 0;
    for line in input.iter() {
      let mut open_chunks: Vec<char> = vec![];
      let chunks: Vec<char> = line.chars().collect();
      for chunk in chunks.iter() {
        match chunk {
          '(' | '[' | '{' | '<' => open_chunks.push(*chunk),
          ')' => if open_chunks.pop() != Some('(') { syntax_error_score += 3; break; },
          ']' => if open_chunks.pop() != Some('[') { syntax_error_score += 57; break; },
          '}' => if open_chunks.pop() != Some('{') { syntax_error_score += 1197; break; },
          '>' => if open_chunks.pop() != Some('<') { syntax_error_score += 25137; break; },
          _ => simple_error::bail!(format!("Invalid character: {}", chunk)),
        }
      }
    }
    Ok(syntax_error_score)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;

    let mut completion_scores: Vec<i64> = vec![];
    for line in input.iter() {
      let mut open_chunks: Vec<char> = vec![];
      let chunks: Vec<char> = line.chars().collect();
      let mut invalid: bool = false;
      for chunk in chunks.iter() {
        match chunk {
          '(' | '[' | '{' | '<' => open_chunks.push(*chunk),
          ')' => if open_chunks.pop() != Some('(') { invalid = true; break; },
          ']' => if open_chunks.pop() != Some('[') { invalid = true; break; },
          '}' => if open_chunks.pop() != Some('{') { invalid = true; break; },
          '>' => if open_chunks.pop() != Some('<') { invalid = true; break; },
          _ => simple_error::bail!(format!("Invalid character: {}", chunk)),
        }
      }
      if invalid { continue; }
      let mut score: i64 = 0;
      // println!("Open chunks: {:?}", open_chunks);
      for open_chunk in open_chunks.iter().rev() {
        match open_chunk {
          '(' => { score *= 5; score += 1},
          '[' => { score *= 5; score += 2},
          '{' => { score *= 5; score += 3},
          '<' => { score *= 5; score += 4},
          _ => simple_error::bail!(format!("Invalid open chunk: {}", open_chunk)),
        }
        // println!("Running score: {}", score);
      }
      completion_scores.push(score);
    }
    println!("Found scores: {:?}", completion_scores);
    completion_scores.sort();
    Ok(completion_scores[(completion_scores.len() - 1) / 2])
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
