use std::collections::{HashMap, HashSet};
use std::error::Error as Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "8" }

  // 1->2, 4->4, 7->3, 8->7
  fn test1_result(&self) -> i64 { 26 }
  fn test2_result(&self) -> i64 { 61229 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;
    
    let mut unique_segments: i64 = 0;
    for values in input.iter() {
      let parsed: Vec<&str> = values.split(" | ").collect();
      let output: Vec<&str> = parsed[1].split(" ").collect();
      for d in output.iter() {
        match d.len() {
          2 | 3 | 4 | 7 => unique_segments += 1,
          _ => continue,
        }
      }
    }

    Ok(unique_segments)
  }

  //   0:      1:      2:      3:      4:
  //  aaaa    ....    aaaa    aaaa    ....
  // b    c  .    c  .    c  .    c  b    c
  // b    c  .    c  .    c  .    c  b    c
  //  ....    ....    dddd    dddd    dddd
  // e    f  .    f  e    .  .    f  .    f
  // e    f  .    f  e    .  .    f  .    f
  //  gggg    ....    gggg    gggg    ....
  //
  //   5:      6:      7:      8:      9:
  //  aaaa    aaaa    aaaa    aaaa    aaaa
  // b    .  b    .  .    c  b    c  b    c
  // b    .  b    .  .    c  b    c  b    c
  //  dddd    dddd    ....    dddd    dddd
  // .    f  e    f  .    f  e    f  .    f
  // .    f  e    f  .    f  e    f  .    f
  //  gggg    gggg    ....    gggg    gggg
  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;
    
    let mut output_sum: i64 = 0;
    for values in input.iter() {
      let mut codes: Vec<String> = vec![];
      codes.resize(10, "".to_string());

      let parsed: Vec<&str> = values.split(" | ").collect();
      let input: Vec<&str> = parsed[0].split(" ").collect();
      let output: Vec<&str> = parsed[1].split(" ").collect();
      // 1->2, 4->4, 7->3, 8->7
      for d in input.iter() {
        match d.len() {
          2 => codes[1] = sort_chars(d.to_string()),
          3 => codes[7] = sort_chars(d.to_string()),
          4 => codes[4] = sort_chars(d.to_string()),
          7 => codes[8] = sort_chars(d.to_string()),
          _ => continue,
        }
      }
      // 0, 6, 9
      // 6 does not contain 1
      // 0 does not contain 4
      let one_segments: HashSet<char> = codes[1].chars().collect();
      let four_segments: HashSet<char> = codes[4].chars().collect();
      for d in input.iter() {
        if d.len() != 6 { continue; }
        let input_segments: HashSet<char> = d.chars().collect();
        if !one_segments.is_subset(&input_segments) {
          codes[6] = sort_chars(d.to_string());
        } else if !four_segments.is_subset(&input_segments) {
          codes[0] = sort_chars(d.to_string());
        } else {
          codes[9] = sort_chars(d.to_string());
        }
      }
      // 2, 3, 5
      // 3 contains 1
      // 2 contains 'cc' segment
      let six_segments: HashSet<char> = codes[6].chars().collect();
      let cc_segment: &char = one_segments.difference(&six_segments).next().unwrap();
      println!("{:?}", cc_segment);
      for d in input.iter() {
        if d.len() != 5 { continue; }
        let input_segments: HashSet<char> = d.chars().collect();
        if one_segments.is_subset(&input_segments) {
          codes[3] = sort_chars(d.to_string());
        } else if input_segments.contains(&cc_segment) {
          codes[2] = sort_chars(d.to_string());
        } else {
          codes[5] = sort_chars(d.to_string());
        }
      }
      
      let mut decoded = HashMap::new();
      for i in 0..codes.len() {
        decoded.insert(&codes[i], i);
      }
      println!("Decoded: {:?}", decoded);
      
      let mut output_number = 0;
      for d in output.iter() {
        output_number *= 10;
        let val = decoded.get(&sort_chars(d.to_string()));
        if val.is_none() {
          output_number += 5;
        } else {
          output_number += val.unwrap();
        }
      }
      output_sum += output_number as i64;
      println!("{:?} is {}", parsed[1], output_number);
    }

    Ok(output_sum)
  }
}

fn sort_chars(s: String) -> String {
  let mut v: Vec<char> = s.chars().collect();
  v.sort();
  v.into_iter().collect::<String>()
}

pub fn main() {
  let d = Solution {};
  let _r2 = Exercise::run(&d, false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_is_working() {
    let d = Solution {};
    Exercise::run(&d, true);
  }
}
