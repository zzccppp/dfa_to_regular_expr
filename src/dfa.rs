use std::{collections::HashMap, fs::File, io::Read, vec};

#[derive(Debug, Clone)]
pub struct DFANode {
    id: i32,
    edges: Vec<Edge>,
}

impl DFANode {
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
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
    pub fn new_from_file(path: &String) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut map: HashMap<i32, DFANode> = HashMap::new();
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        for line in buf.lines() {
            let sp: Vec<String> = line.split('|').map(|s| String::from(s)).collect();
            let id: i32 = sp[0].parse().unwrap();
            let edge_sp: Vec<String> = sp[1].split("->").map(|s| String::from(s)).collect();
            let edge = Edge {
                from: edge_sp[0].parse().unwrap(),
                to: edge_sp[1].parse().unwrap(),
                cond: sp[2].split(",").map(|s| String::from(s.trim())).collect(),
            };
            match map.get_mut(&id) {
                Some(x) => {
                    x.add_edge(edge);
                }
                None => {
                    map.insert(id, DFANode{
                        id: id,
                        edges: vec![edge],
                    });
                }
            }
        }

        Ok(Self { nodes: map })
    }
}
