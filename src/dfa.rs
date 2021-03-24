use std::{collections::HashMap, fs::File};

#[derive(Debug, Clone)]
pub struct DFANode {
    id: i32,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    from: i32,
    to: i32,
    cond: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DFA {
    nodes: HashMap<i32, DFANode>,
}

impl DFA {
    pub fn new_from_file(path: &String) -> Result<Self, dyn std::error::Error> {
        let mut file = File::open(path)?;
    }
}
