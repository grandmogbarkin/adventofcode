use std::collections::{HashMap, HashSet};

pub struct Tree {
    pub nodes: HashMap<String, HashSet<String>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
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

    pub fn get_nodes(&self, start_node: &String) -> Option<&HashSet<String>> {
        self.nodes.get(start_node)
    }
}
