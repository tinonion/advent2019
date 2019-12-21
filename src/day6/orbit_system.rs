use std::collections::{ HashMap, VecDeque, HashSet };

pub type EdgeIndex = usize;
pub type NodeIndex = usize;

const ROOT: &str = "COM";
const YOU: &str = "YOU";
const SANTA: &str = "SAN";

pub struct OrbitEdge {
    pub target: NodeIndex,
    pub next_edge: Option<EdgeIndex>
}

pub struct OrbitNode {
    pub name: String,
    pub first_edge: Option<EdgeIndex>
}

pub struct OrbitSystem {
    nodes: Vec<OrbitNode>,
    edges: Vec<OrbitEdge>,
    node_map: HashMap<String, NodeIndex>
}

impl OrbitSystem {
    pub fn new() -> OrbitSystem {
        OrbitSystem {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_map: HashMap::new()
        }
    }

    pub fn add_relation(&mut self, primary: &str, satellite: &str) {
        // initialize necessary nodes
        let primary_ind = self.find_node_index(primary);
        let satellite_ind = self.find_node_index(satellite);

        self.create_edge(primary_ind, satellite_ind);
        self.create_edge(satellite_ind, primary_ind);
    }

    fn create_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let source_node = &mut self.nodes[source];

        let new_edge_index = self.edges.len();
        let new_edge = OrbitEdge {
            target: target,
            next_edge: source_node.first_edge
        };
        self.edges.push(new_edge);
        
        source_node.first_edge = Some(new_edge_index);
    }

    pub fn find_distance(&self) -> i32 {
        let root = *self.node_map.get(ROOT).unwrap();

        self.node_dive(root, root, 0)
    }

    pub fn path_to_santa(&self) -> i32 {
        // start at "YOU" node
        let curr_node = self.node_map.get(YOU).unwrap();
        let mut curr_node = &self.nodes[*curr_node];

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut search_queue: VecDeque<(NodeIndex, i32)> = VecDeque::new();
        let mut steps = 0;
        let mut explored = 0;

        loop {
            if curr_node.name == SANTA {
                // subtract two to exclude YOU and SAN as orbitals that need to be traversed
                return steps - 2;
            }

            // add current node's edges' nodes to search queue
            let mut next_edge = curr_node.first_edge;
            while let Some(e) = next_edge {
                let edge = &self.edges[e];

                if !visited.contains(&edge.target) {
                    search_queue.push_back((edge.target, steps + 1));
                    visited.insert(edge.target);
                }

                next_edge = edge.next_edge;
            }

            if search_queue.is_empty() {
                panic!("search queue emptied before finding santa, explored {} nodes", explored);
            }

            let (next_node_i, node_depth) = search_queue.pop_front().unwrap();
            curr_node = &self.nodes[next_node_i];
            steps = node_depth;

            explored += 1;
        }

    }

    fn node_dive(&self, last_node: NodeIndex, node: NodeIndex, depth: i32) -> i32 {
        let mut distance = depth;

        let mut next_edge = self.nodes[node].first_edge;
        while let Some(next_e) = next_edge {
            let edge = &self.edges[next_e];

            if edge.target != last_node {
                distance += self.node_dive(node, edge.target, depth + 1);
            }

            next_edge = edge.next_edge;
        } 

        distance
    }

    fn find_node_index(&mut self, node_name: &str) -> NodeIndex {
        if !self.node_map.contains_key(node_name) {
            let new_node_ind = self.nodes.len();

            let new_node = OrbitNode {
                name: String::from(node_name),
                first_edge: None
            };           
            self.nodes.push(new_node);

            self.node_map.insert(String::from(node_name), new_node_ind);
        }

        *self.node_map.get(node_name).unwrap()
    }
}