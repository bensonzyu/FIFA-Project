use crate::player::Player;
use std::collections::{HashMap, HashSet};

pub struct Graph {
    nodes: Vec<Player>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, player: Player) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(player);
        self.edges.insert(idx, Vec::new());
        idx
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        self.edges.get_mut(&src).unwrap().push(dest);
        self.edges.get_mut(&dest).unwrap().push(src);
    }

    pub fn degree(&self, node: usize) -> usize {
        self.edges.get(&node).unwrap().len()
    }
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        
        for node in 0..self.nodes.len() {
            let degree = self.degree(node);
            let count = distribution.entry(degree).or_insert(0);
            *count += 1;
        }
        
        distribution
    }
    

    pub fn dfs(&self, visited: &mut HashSet<usize>, node: usize) -> Vec<&Player> {
        let mut result = vec![&self.nodes[node]];
        visited.insert(node);

        for neighbor in self.edges.get(&node).unwrap() {
            if !visited.contains(neighbor) {
                result.extend(self.dfs(visited, *neighbor));
            }
        }

        result
    }

    pub fn connected_components(&self) -> Vec<Vec<&Player>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for i in 0..self.nodes.len() {
            if !visited.contains(&i) {
                components.push(self.dfs(&mut visited, i));
            }
        }

        components
    }
}

#[test]
fn test_add_node() {
    let mut graph = Graph::new();
    let player = Player { overall: 90, value_eur: 100_000 };
    let node = graph.add_node(player.clone());

    assert_eq!(graph.nodes[node], player);
}

#[test]
fn test_add_edge() {
    let mut graph = Graph::new();
    let player1 = Player { overall: 90, value_eur: 100_000 };
    let player2 = Player { overall: 89, value_eur: 90_000 };

    let node1 = graph.add_node(player1.clone());
    let node2 = graph.add_node(player2.clone());

    graph.add_edge(node1, node2);

    assert!(graph.edges[&node1].contains(&node2));
    assert!(graph.edges[&node2].contains(&node1));
}

#[test]
fn test_same_overall_edge() {
    let mut graph = Graph::new();
    let player1 = Player { overall: 90, value_eur: 100_000 };
    let player2 = Player { overall: 89, value_eur: 90_000 };
    let player3 = Player { overall: 89, value_eur: 90_000 };
    let player4 = Player { overall: 87, value_eur: 80_000 };


    let node1 = graph.add_node(player1.clone());
    let node2 = graph.add_node(player2.clone());
    let node3 = graph.add_node(player3.clone());
    let node4 = graph.add_node(player4.clone());

    // Connect nodes with the same overall rating
    for i in 0..graph.nodes.len() {
        for j in (i + 1)..graph.nodes.len() {
            if graph.nodes[i].overall == graph.nodes[j].overall {
                graph.add_edge(i, j);
            }
        }
    }

    assert_eq!(graph.degree(node1), 0);
    assert_eq!(graph.degree(node2), 1);
    assert_eq!(graph.degree(node3), 1);
    assert_eq!(graph.degree(node4), 0);

}
