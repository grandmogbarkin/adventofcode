use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;
use std::cmp::{min, max};

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
  coords: Vec<Vec<usize>>,
  max_x: usize,
  max_y: usize,
}

impl SeaMap {
  fn new() -> Self {
    SeaMap{coords: vec!(), max_x: 0, max_y: 0}
  }
  
  fn add_coords(&mut self, coords: Coords, ignore_diags: bool) {
    // Break early if the line isn't vertical or horizontal
    let diagonal = coords.x1 != coords.x2 && coords.y1 != coords.y2;
    if ignore_diags && diagonal {
      return;
    }
    
    // Make sure our board is the right size. There's probably a better way to do this
    self.max_y = max(self.max_y, max(coords.y1, coords.y2)+1);
    self.max_x = max(self.max_x, max(coords.x1, coords.x2)+1);
    
    self.coords.resize(self.max_y, vec!());
    for i in 0..self.coords.len() {
      self.coords[i].resize(self.max_x, 0);
    }
    
    // Mark the lines
    if !diagonal {
      for x in min(coords.x1,coords.x2)..=max(coords.x1,coords.x2) {
        for y in min(coords.y1,coords.y2)..=max(coords.y1,coords.y2) {
          self.coords[y][x] += 1;
        }
      }
    } else {
      let mut x = coords.x1;
      let mut y = coords.y1;
      self.coords[y][x] += 1;
      while x != coords.x2 && y != coords.y2 {
        // Assuming 45-degree lines
        if coords.x2 > x {
          x += 1;
        } else {
          x -= 1;
        }
        if coords.y2 > y {
          y += 1;
        } else {
          y -= 1;
        }
        self.coords[y][x] += 1;
      }
    }
  }
  
  fn get_overlapping_line_count(&self) -> i64 {
    let mut count = 0_i64;
    for rows in &self.coords {
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

struct Day0 { }

impl Solution for Day0 {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let directions = read::read_lines(fs::File::open(filename)?)?;

    let mut sea_map = SeaMap::new();
    
    for d in directions.iter() {
      let s: Vec<&str> = d.split(" -> ").collect();
      let mut coords: Vec<Vec<usize>> = vec![];
      for i in 0..s.len() {
        coords.push(s[i].split(",").map(|s| s.parse::<usize>().unwrap()).collect());
      }
      sea_map.add_coords(Coords::new(coords), true);
    }
    println!("{:?}, {:?}", sea_map.max_x, sea_map.max_y);
    Ok(sea_map.get_overlapping_line_count())
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let directions = read::read_lines(fs::File::open(filename)?)?;

    let mut sea_map = SeaMap::new();
    
    for d in directions.iter() {
      let s: Vec<&str> = d.split(" -> ").collect();
      let mut coords: Vec<Vec<usize>> = vec![];
      for i in 0..s.len() {
        coords.push(s[i].split(",").map(|s| s.parse::<usize>().unwrap()).collect());
      }
      sea_map.add_coords(Coords::new(coords), false);
    }
    println!("{:?}, {:?}", sea_map.max_x, sea_map.max_y);
    Ok(sea_map.get_overlapping_line_count())
  }
}

pub fn main() {
  let d = Day0 {};

  let args1: Vec<String> = vec!["1".to_string(),
                                "inputs/input5.txt".to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                "inputs/input5.txt".to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_1() {
    let args: Vec<String> = vec!["1".to_string(),
                                 "inputs/input5.test.txt".to_string()];
    let d = super::Day0 {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 5);
  }

  #[test]
  fn test_2() {
    let args: Vec<String> = vec!["2".to_string(),
                                 "inputs/input5.test.txt".to_string()];
    let d = super::Day0 {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 12);
  }
}
