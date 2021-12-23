use std::collections::HashMap;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;
use regex::Regex;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "22"
    }

    fn test1_result(&self) -> i64 {
        474140
    }
    fn test2_result(&self) -> i64 {
        2758514936282235
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let re = Regex::new(
            r"^([onf]+) x=([-\d]+)\.\.([-\d]+),y=([-\d]+)\.\.([-\d]+),z=([-\d]+)\.\.([-\d]+)$",
        )
        .unwrap();

        let mut reactor: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 101]; 101]; 101];

        for line in input {
            let cap_r = re.captures(&line);
            // println!("{}: {:?}", line, cap_r);
            // if cap_r.is_none() { continue; }
            let cap = cap_r.unwrap();
            let state = &cap[1];
            let coords: [i64; 6] = [
                cap[2].parse::<i64>().unwrap() + 50,
                cap[3].parse::<i64>().unwrap() + 50,
                cap[4].parse::<i64>().unwrap() + 50,
                cap[5].parse::<i64>().unwrap() + 50,
                cap[6].parse::<i64>().unwrap() + 50,
                cap[7].parse::<i64>().unwrap() + 50,
            ];
            let mut skip = false;
            for c in coords {
                if c < 0 || c > 100 {
                    // println!("Skipping {:?}", cap);
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }
            let new_state: i64 = if state == "on" { 1 } else { 0 };
            for x in coords[0]..coords[1] + 1 {
                for y in coords[2]..coords[3] + 1 {
                    for z in coords[4]..coords[5] + 1 {
                        reactor[x as usize][y as usize][z as usize] = new_state;
                    }
                }
            }
        }

        Ok(reactor.iter().flatten().flatten().sum::<i64>())
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let re = Regex::new(
            r"^([onf]+) x=([-\d]+)\.\.([-\d]+),y=([-\d]+)\.\.([-\d]+),z=([-\d]+)\.\.([-\d]+)$",
        )
        .unwrap();

        let mut reactor: HashMap<(usize, usize, usize), i64> = HashMap::new();
        let mut max = std::i64::MIN;
        let mut min = std::i64::MAX;
        
        for line in input {
            let cap_r = re.captures(&line);
            // println!("{}: {:?}", line, cap_r);
            // if cap_r.is_none() { continue; }
            let cap = cap_r.unwrap();
            let state = &cap[1];
            let coords: [i64; 6] = [
                cap[2].parse::<i64>().unwrap(),
                cap[3].parse::<i64>().unwrap(),
                cap[4].parse::<i64>().unwrap(),
                cap[5].parse::<i64>().unwrap(),
                cap[6].parse::<i64>().unwrap(),
                cap[7].parse::<i64>().unwrap(),
            ];
            max = std::cmp::max(max, *coords.iter().max().unwrap());
            min = std::cmp::min(min, *coords.iter().min().unwrap());
            // let new_state: i64 = if state == "on" { 1 } else { 0 };
            // for x in coords[0]..coords[1] + 1 {
            //     for y in coords[2]..coords[3] + 1 {
            //         for z in coords[4]..coords[5] + 1 {
            //             *reactor
            //                 .entry((x as usize, y as usize, z as usize))
            //                 .or_insert(0) = new_state;
            //         }
            //     }
            // }
        }
        
        println!("Max: {}, Min: {}", max, min);

        Ok(reactor.values().sum::<i64>())
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
