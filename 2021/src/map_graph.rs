use std::collections::{HashMap, HashSet};

pub struct Tree {
    pub nodes: HashMap<String, HashSet<String>>,
    pub paths: Vec<Vec<String>>,
    pub visited: HashSet<String>,
    pub cur_path: Vec<String>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            paths: vec![],
            visited: HashSet::new(),
            cur_path: vec![],
        }
    }

    pub fn add_nodes_pair(&mut self, node1: &str, node2: &str) {
        let n1 = self
            .nodes
            .entry(node1.to_string())
            .or_insert(HashSet::new());
        n1.insert(node2.to_string());
        let n2 = self
            .nodes
            .entry(node2.to_string())
            .or_insert(HashSet::new());
        n2.insert(node1.to_string());
    }

    pub fn number_of_paths(&self) -> usize {
        self.paths.len()
    }

    pub fn walk_paths(&mut self, visit_twice: bool) {
        self.dfs("start".to_string(), "".to_string(), visit_twice);
    }

    fn dfs(&mut self, start_node: String, mut visited_twice: String, visit_twice: bool) {
        let is_small_cave = start_node.chars().next().unwrap().is_ascii_lowercase();
        if is_small_cave {
            if visit_twice && self.visited.contains(&start_node) {
                visited_twice = start_node.clone();
            }
            self.visited.insert(start_node.clone());
        }

        self.cur_path.push(start_node.clone());

        if start_node == "end" {
            // println!("Found path {:?}", self.cur_path);
            self.paths.push(self.cur_path.clone());
        } else {
            let next_nodes = self.nodes.get(&start_node).unwrap().clone();
            for node in next_nodes {
                if node == "start" {
                    continue;
                }
                let already_visited = self.visited.contains(&node);
                if !already_visited || (visit_twice && visited_twice == "") {
                    self.dfs(node.clone(), visited_twice.clone(), visit_twice);
                }
            }
        }

        self.cur_path.pop();
        if visited_twice != start_node {
            self.visited.remove(&start_node);
        }
    }
}
