use petgraph::{algo::tarjan_scc, graph::NodeIndex, Direction::Outgoing, Graph};
use std::collections::HashSet;

use crate::serialize::CycleNodesSorter;

pub struct ByOutGoingEdgeCountSorter {
    pub limit: usize,
}

impl Default for ByOutGoingEdgeCountSorter {
    fn default() -> Self {
        ByOutGoingEdgeCountSorter { limit: 100 }
    }
}

pub fn get_cycles<N, E>(graph: &Graph<N, E>) -> Vec<HashSet<NodeIndex>> {
    let groups = tarjan_scc(graph);
    groups
        .iter()
        .filter(|group| group.len() > 1 || graph.contains_edge(group[0], group[0]))
        .map(|group| HashSet::from_iter(group.iter().cloned()))
        .collect()
}

impl<N, E> CycleNodesSorter<N, E> for ByOutGoingEdgeCountSorter {
    fn sorted(&self, graph: &Graph<N, E>, group: &HashSet<NodeIndex>) -> Vec<NodeIndex> {
        let mut curr = *group.iter().next().unwrap();
        let mut visited: HashSet<NodeIndex<u32>> = HashSet::new();
        let mut chain: Vec<NodeIndex<u32>> = Vec::new();
        loop {
            chain.push(curr);
            // print!("{}({}) -> ", curr.index(), graph[curr]);
            let neighbors = graph
                .neighbors_directed(curr, Outgoing)
                .collect::<HashSet<_>>();
            let neighbors = neighbors
                .iter()
                .filter(|i| group.contains(i) && !visited.contains(i))
                .collect::<Vec<_>>();
            visited.insert(curr);

            // neighbors is empty means at the end of the cycle
            if let Some(next_ni) = neighbors.get(0) {
                curr = **next_ni;
            } else {
                let start_ni = graph
                    .neighbors_directed(curr, Outgoing).find(|ni| group.contains(ni))
                    .expect("start node must be in the group");
                let (index, _) = chain
                    .iter()
                    .enumerate()
                    .find(|(_, ni)| **ni == start_ni)
                    .unwrap();
                return chain[index..].to_vec();
            }
        }
    }

    fn unlink_cycle(&self, graph: &mut Graph<N, E>, cycle_node_set: &HashSet<NodeIndex>) {
        // unlink self-loops
        if cycle_node_set.len() == 1 {
            let ni = cycle_node_set.iter().next().unwrap();
            let e = graph.find_edge(*ni, *ni).unwrap();
            graph.remove_edge(e);
            return;
        }

        // dbg_cycles(graph, &group.iter().cloned().collect::<Vec<_>>());
        let chain = self.sorted(graph, cycle_node_set);
        let max_ref_node = chain
            .iter()
            .enumerate()
            .max_by_key(|(_, ni)| graph.neighbors_directed(**ni, Outgoing).count())
            .unwrap();
        let parent_node_index = chain[(max_ref_node.0 + chain.len() - 1) % chain.len()];
        let edge = graph.find_edge(parent_node_index, *max_ref_node.1).unwrap();
        graph.remove_edge(edge);
    }

    fn decompose_cycle(&self, graph: &mut Graph<N, E>) {
        let mut count = 0usize;
        while count < self.limit {
            for cycle_node_set in get_cycles(graph) {
                self.unlink_cycle(graph, &cycle_node_set);
            }
            let has_cycle = !get_cycles(graph).is_empty();
            if !has_cycle {
                break;
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;

    use crate::decomp_cycles::{ByOutGoingEdgeCountSorter, CycleNodesSorter};

    fn to_nis(idxs: Vec<usize>) -> Vec<NodeIndex> {
        idxs.iter().map(|i| NodeIndex::new(*i)).collect::<Vec<_>>()
    }

    /// 0 -> 1 -> 2 -> 0 : [0, 1, 2]
    #[test]
    pub fn test_get_cycle1() {
        let sorter: ByOutGoingEdgeCountSorter = Default::default();
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle = sorter.sorted(&mut graph, &to_nis(vec![0, 1, 2]).iter().cloned().collect());
        assert_eq!(cycle, to_nis(vec![0, 1, 2]));
    }

    /// 0 -> 1 -> 2 -> 1 : [1, 2]
    #[test]
    pub fn test_get_cycle2() {
        let sorter: ByOutGoingEdgeCountSorter = Default::default();
        let edges = vec![(0, 1), (1, 2), (2, 1)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle = sorter.sorted(&mut graph, &to_nis(vec![0, 1, 2]).iter().cloned().collect());
        assert_eq!(cycle, to_nis(vec![1, 2]));
    }
}
