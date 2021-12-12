use std::collections::HashSet;
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

    let node = "start".to_string();
    let visited_nodes: HashSet<String> = HashSet::new();
    Ok(next_step(
        &tree,
        &node,
        visited_nodes.clone(),
        visit_small_cave_twice,
    ))
}

fn next_step(
    tree: &map_graph::Tree,
    start_node: &String,
    mut visited_nodes: HashSet<String>,
    visit_small_cave_twice: bool,
) -> i64 {
    println!("Visiting {} with visited: {:?}", start_node, visited_nodes);
    if start_node == "end" {
        return 1;
    }
    let is_small_cave = start_node.chars().next().unwrap().is_ascii_lowercase();
    if is_small_cave {
        visited_nodes.insert(start_node.clone());
    }
    let next_nodes = tree.get_nodes(&start_node).unwrap();
    let unvisited_nodes: HashSet<&String> = next_nodes.difference(&visited_nodes).collect();
    println!("Unvisited nodes: {:?}", unvisited_nodes);
    if 0 == unvisited_nodes.len() {
        return 0;
    }
    let mut path_len = 0;
    for next_node in unvisited_nodes.iter() {
        path_len += next_step(
            tree,
            next_node,
            visited_nodes.clone(),
            visit_small_cave_twice,
        );
    }
    if visit_small_cave_twice && is_small_cave && start_node != "start" {
        println!("In cave: {}, visited: {:?}", start_node, visited_nodes);
        visited_nodes.remove(start_node);
        for next_node in next_nodes.difference(&visited_nodes) {
            path_len += next_step(tree, next_node, visited_nodes.clone(), false);
        }
    }
    path_len
}

pub fn main() {
    let d = Solution {};
    Exercise::run(&d, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_is_working() {
    //   let d = Solution {};
    //   Exercise::run(&d, true)
    // }

    #[test]
    fn tiny_test() {
        let d = Solution {};
        Exercise::custom_test(&d, ".test.tiny", 10, 36)
    }

    // #[test]
    // fn small_test() {
    //     let d = Solution {};
    //     Exercise::custom_test(&d, ".test.small", 19, 103)
    // }
}
