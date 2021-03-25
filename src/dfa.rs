use std::{collections::HashMap, fs::File, io::Read, vec};

use crate::regular_exp::ReExpr;

#[derive(Debug, Clone)]
pub struct DFANode {
    id: i32,
    edges: Vec<Edge>,
}

impl DFANode {
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn conditions_to(&self, to: i32) -> Vec<String> {
        let mut re = Vec::new();
        for t in self.edges.iter() {
            if t.from == self.id && t.to == to {
                for i in t.cond.iter() {
                    re.push(i.clone());
                }
            }
        }
        re
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
                    map.insert(
                        id,
                        DFANode {
                            id: id,
                            edges: vec![edge],
                        },
                    );
                }
            }
        }

        Ok(Self { nodes: map })
    }

    pub fn construct_regx(&self) {
        let buff: HashMap<(i32, i32, i32), ReExpr> = HashMap::new();
        let node_count = self.nodes.len() as i32;

        for i in 0..(node_count + 1) {
            for j in 0..(node_count + 1) {
                let node = self.nodes.get(&i).unwrap();
                let cond = node.conditions_to(j);
                let regx = if i == j {
                    let mut temp = ReExpr::Epsilon;
                    for con in cond.iter() {
                        temp.or(ReExpr::Value(con.clone()));
                    }
                    temp
                } else {
                    let temp = if cond.len() == 0 {
                        ReExpr::Null
                    } else {
                        let mut t = ReExpr::Value(cond.first().unwrap().clone());
                        for i in 1..cond.len() {
                            t.or(ReExpr::Value(cond[i].clone()))
                        }
                        t
                    };
                    temp
                };

                println!("{}", regx.to_string())
            }
        }

        for k in 0..(node_count + 1) {
            for i in 0..(node_count + 1) {
                for j in 0..(node_count + 1) {}
            }
        }
    }
}
