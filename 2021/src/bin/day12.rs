// use std::collections::HashSet;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::map_graph;
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "12"
    }

    fn test1_result(&self) -> i64 {
        226
    }
    fn test2_result(&self) -> i64 {
        3509
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, false)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        run(filename, true)
    }
}

fn run(filename: String, visit_small_cave_twice: bool) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(filename)?;

    let mut tree = map_graph::Tree::new();
    for line in input.iter() {
        let pair: Vec<&str> = line.split("-").collect();
        if pair.len() != 2 {
            simple_error::bail!(format!("Invalid input: {:?}", pair));
        }
        tree.add_nodes_pair(pair[0], pair[1]);
    }

    // println!("{:?}", tree.nodes);

    tree.walk_paths(visit_small_cave_twice);
    Ok(tree.number_of_paths() as i64)
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

    #[test]
    fn tiny_test() {
        let d = Solution {};
        Exercise::custom_test(&d, ".test.tiny", 10, 36)
    }

    #[test]
    fn small_test() {
        let d = Solution {};
        Exercise::custom_test(&d, ".test.small", 19, 103)
    }
}
