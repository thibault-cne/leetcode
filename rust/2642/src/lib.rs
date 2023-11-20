pub type NodeIndex = i32;

pub type EdgeIndex = i32;

#[derive(Debug, PartialEq)]
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, PartialEq)]
pub struct EdgeData {
    target: i32,
    weight: i32,
    next_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut nodes: Vec<NodeData> = (0..n as usize)
            .map(|_| NodeData {
                first_outgoing_edge: None,
            })
            .collect();
        let mut edges_data = Vec::new();

        for e in edges {
            // For safety
            assert_eq!(e.len(), 3);

            let (from, to, weight) = (e[0], e[1], e[2]);
            let index = edges_data.len();
            let node = &mut nodes[from as usize];
            edges_data.push(EdgeData {
                target: to,
                weight,
                next_outgoing_edge: node.first_outgoing_edge,
            });
            node.first_outgoing_edge = Some(index as i32);
        }

        Graph {
            nodes,
            edges: edges_data,
        }
    }

    pub fn add_edge(&mut self, edge: Vec<i32>) {
        // For safety
        assert_eq!(edge.len(), 3);

        let (from, to, weight) = (edge[0], edge[1], edge[2]);
        let index = self.edges.len();
        let node = &mut self.nodes[from as usize];
        self.edges.push(EdgeData {
            target: to,
            weight,
            next_outgoing_edge: node.first_outgoing_edge,
        });
        node.first_outgoing_edge = Some(index as i32);
    }

    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut dist = vec![i32::MAX; self.nodes.len()];
        let mut queue = (0..self.nodes.len()).collect::<Vec<usize>>();

        dist[node1 as usize] = 0;

        loop {
            queue.sort_unstable_by(|&a, &b| dist[b].cmp(&dist[a]));

            if let Some(node) = queue.pop() {
                if node == node2 as usize {
                    break;
                }

                for &neighbour in queue.iter().filter(|&&n| self.are_neighbours(node, n)) {
                    let alt = match dist[node].checked_add(self.edge_weight(node, neighbour).unwrap()) {
                        Some(d) => d,
                        None => i32::MAX,
                    };
                    if alt < dist[neighbour] {
                        dist[neighbour] = alt;
                    }
                }
            } else {
                break;
            }
        }

        match dist[node2 as usize] {
            d if d == i32::MAX => -1,
            d => d,
        }
    }

    pub fn are_neighbours(&self, node1: usize, node2: usize) -> bool {
        let mut edge = self.nodes[node1].first_outgoing_edge;

        loop {
            match edge {
                Some(e) if self.edges[e as usize].target as usize == node2 => return true,
                Some(e) => edge = self.edges[e as usize].next_outgoing_edge,
                None => break,
            }
        }

        false
    }

    pub fn edge_weight(&self, node1: usize, node2: usize) -> Option<i32> {
        let mut edge = self.nodes[node1].first_outgoing_edge;

        loop {
            match edge {
                Some(e) if self.edges[e as usize].target as usize == node2 => {
                    return Some(self.edges[e as usize].weight)
                }
                Some(e) => edge = self.edges[e as usize].next_outgoing_edge,
                None => break,
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let graph = Graph::new(4, vec![vec![0, 2, 1], vec![0, 3, 1], vec![1, 2, 1]]);

        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(graph.edges.len(), 3);

        assert_eq!(
            graph.nodes,
            vec![
                NodeData {
                    first_outgoing_edge: Some(1),
                },
                NodeData {
                    first_outgoing_edge: Some(2),
                },
                NodeData {
                    first_outgoing_edge: None,
                },
                NodeData {
                    first_outgoing_edge: None,
                },
            ]
        );

        assert_eq!(
            graph.edges,
            vec![
                EdgeData {
                    target: 2,
                    weight: 1,
                    next_outgoing_edge: None,
                },
                EdgeData {
                    target: 3,
                    weight: 1,
                    next_outgoing_edge: Some(0),
                },
                EdgeData {
                    target: 2,
                    weight: 1,
                    next_outgoing_edge: None,
                },
            ]
        );
    }

    #[test]
    fn test_2() {
        let mut graph = Graph::new(4, vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]]);

        assert_eq!(graph.shortest_path(3, 2), 6);
        assert_eq!(graph.shortest_path(0, 3), -1);

        graph.add_edge(vec![1, 3, 4]);

        assert_eq!(graph.shortest_path(0, 3), 6);
    }

    #[test]
    fn test_3() {
        let mut graph = Graph::new(6,  vec![ vec![0,4,617630], vec![5,3,501040], vec![3,4,654340], vec![5,1,277928], vec![4,3,519665]]);
        graph.add_edge(vec![4,2,36803]);
        graph.add_edge(vec![5,2,156440]);
        graph.add_edge(vec![4,1,138227]);
        graph.add_edge(vec![3,2,484787]);
        graph.add_edge(vec![4,5,1455]);

        assert_eq!(graph.shortest_path(2, 3), -1);
    }
}
