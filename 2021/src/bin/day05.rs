use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Coords {
  x1: usize,
  y1: usize,
  x2: usize,
  y2: usize,
}

impl Coords {
  fn new(c: Vec<Vec<usize>>) -> Self {
    assert_eq!(2, c.len());
    assert_eq!(2, c[0].len());
    assert_eq!(2, c[1].len());
    
    Coords{x1: c[0][0], y1: c[0][1], x2: c[1][0], y2: c[1][1]}
  }
}

struct SeaMap {
  diagram: Vec<Vec<usize>>,
  max_x: usize,
  max_y: usize,
}

impl SeaMap {
  fn new() -> Self {
    SeaMap{diagram: vec!(), max_x: 0, max_y: 0}
  }
  
  fn add_coords(&mut self, coords: Coords, ignore_diags: bool) {
    // Break early if the line isn't vertical or horizontal
    let diagonal = coords.x1 != coords.x2 && coords.y1 != coords.y2;
    if ignore_diags && diagonal {
      return;
    }
    
    // Make sure our board is the right size. There's probably a better way to do this
    self.max_y = std::cmp::max(self.max_y, std::cmp::max(coords.y1, coords.y2)+1);
    self.max_x = std::cmp::max(self.max_x, std::cmp::max(coords.x1, coords.x2)+1);
    
    self.diagram.resize(self.max_y, vec!());
    for c in &mut self.diagram {
      c.resize(self.max_x, 0);
    }
    
    // Mark the lines
    //   // One of these loops is a single iteration by design, but don't care to check which
    //   for x in std::cmp::min(coords.x1,coords.x2)..=std::cmp::max(coords.x1,coords.x2) {
    //     for y in std::cmp::min(coords.y1,coords.y2)..=std::cmp::max(coords.y1,coords.y2) {
    //       self.diagram[y][x] += 1;
    //     }
    //   }

    // Walk the lines.
    let mut x = coords.x1;
    let mut y = coords.y1;
    self.diagram[y][x] += 1;
    while x != coords.x2 || y != coords.y2 {
      // Feels like there's a 1-liner way to do this that's clean and mathematical.
      if coords.x2 > x {
        x += 1;
      } else if coords.x2 < x {
        x -= 1;
      }
      if coords.y2 > y {
        y += 1;
      } else if coords.y2 < y {
        y -= 1;
      }
      self.diagram[y][x] += 1;
    }
  }
  
  fn get_overlapping_line_count(&self) -> i64 {
    let mut count = 0_i64;
    for rows in &self.diagram {
      // println!("{:?}", rows);
      for col in rows {
        if col >= &2 {
          count += 1;
        }
      }
    }
    count
  }
}

fn parse_input(filename: String, ignore_diags: bool) -> Result<i64, Box<dyn Error>> {
  let directions = read::read_lines(fs::File::open(filename)?)?;

  let mut sea_map = SeaMap::new();
  
  // For each set of coords, add them to the sea map. No need to keep track of the coords,
  // we draw the lines on the map as we go.
  for d in directions.iter() {
    let s: Vec<&str> = d.split(" -> ").collect();
    let mut coords: Vec<Vec<usize>> = vec![];
    for i in 0..s.len() {
      coords.push(s[i].split(",").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    sea_map.add_coords(Coords::new(coords), ignore_diags);
  }
  println!("Map size: {:?}, {:?}", sea_map.max_x, sea_map.max_y);
  Ok(sea_map.get_overlapping_line_count())
}

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "5" }

  fn test1_result(&self) -> i64 { 5 }
  fn test2_result(&self) -> i64 { 12 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    parse_input(filename, true)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    parse_input(filename, false)
  }
}

pub fn main() {
  let d = Solution {};

  let args1: Vec<String> = vec!["1".to_string(),
                                format!("inputs/input{}.txt", d.day()).to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                format!("inputs/input{}.txt", d.day()).to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let d = Solution {};
    let args: Vec<String> = vec!["1".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), d.test1_result());
  }

  #[test]
  fn test_2() {
    let d = Solution {};
    let args: Vec<String> = vec!["2".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), d.test2_result());
  }
}
