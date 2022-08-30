use petgraph::visit::DfsPostOrder;
use petgraph::{graph::NodeIndex, Direction::Outgoing};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use super::GraphSerializer;

#[derive(Default)]
pub struct DfsSerializer;

impl<N: Clone + Display, E> GraphSerializer<N, E> for DfsSerializer {
    fn serialize(&self, graph: &mut petgraph::Graph<N, E>) -> anyhow::Result<Vec<N>> {
        let mut nodes_by_ref_count = graph.node_indices().collect::<Vec<_>>();
        nodes_by_ref_count.sort_by_key(|n| {
            graph
                .neighbors_directed(*n, petgraph::Direction::Outgoing)
                .count()
        });
        nodes_by_ref_count.reverse();

        let mut res: Vec<N> = vec![];
        let mut flags: HashSet<NodeIndex> = HashSet::new();
        for ni in nodes_by_ref_count {
            println!("");
            let mut q: VecDeque<NodeIndex> = VecDeque::new();
            q.push_back(ni);
            while let Some(ni) = q.pop_front() {
                print!(" -> {}", graph[ni]);
                for n in graph.neighbors_directed(ni, Outgoing) {
                    if !flags.contains(&ni) {
                        q.push_back(n);
                    }
                }
                if !flags.contains(&ni) {
                    res.push(graph[ni].clone());
                    flags.insert(ni);
                }
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
pub mod tests {
    use super::DfsSerializer;
    use crate::algo::GraphSerializer;
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;

    #[test]
    fn test_ser() {
        let mut g = Graph::<i32, ()>::new();
        for i in 0..5 {
            g.add_node(i);
        }
        for (f, t) in &[(0, 1), (1, 2), (2, 0), (1, 3)] {
            g.add_edge(NodeIndex::new(*f), NodeIndex::new(*t), ());
        }
        let ser = DfsSerializer;
        let res = ser.serialize(&mut g).unwrap();
        dbg!(res);
    }
}
