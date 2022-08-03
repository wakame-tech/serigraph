use petgraph::{algo::tarjan_scc, graph::NodeIndex, Direction::Outgoing, Graph};
use std::collections::HashSet;

use crate::serialize::CycleDecomposer;

pub struct OutGoingCycleDecomposer {
    pub limit: usize,
}

impl Default for OutGoingCycleDecomposer {
    fn default() -> Self {
        OutGoingCycleDecomposer { limit: 100 }
    }
}

fn get_cycles<N, E>(graph: &Graph<N, E>) -> Vec<HashSet<NodeIndex>> {
    let groups = tarjan_scc(graph);
    groups
        .iter()
        .filter(|group| group.len() > 1 || graph.contains_edge(group[0], group[0]))
        .map(|group| HashSet::from_iter(group.iter().cloned()))
        .collect()
}

fn sorted_cycle_by_outgoings<N, E>(
    graph: &Graph<N, E>,
    group: &HashSet<NodeIndex>,
) -> Vec<NodeIndex> {
    let mut curr = *group.iter().next().unwrap();
    let mut visited: HashSet<NodeIndex<u32>> = HashSet::new();
    let mut chain: Vec<NodeIndex<u32>> = Vec::new();
    loop {
        chain.push(curr);
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
                .neighbors_directed(curr, Outgoing)
                .find(|ni| group.contains(ni))
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

fn unlink_cycle<N, E>(graph: &mut Graph<N, E>, cycle_node_set: &HashSet<NodeIndex>) {
    // unlink self-loops
    if cycle_node_set.len() == 1 {
        let ni = cycle_node_set.iter().next().unwrap();
        let e = graph.find_edge(*ni, *ni).unwrap();
        graph.remove_edge(e);
        return;
    }

    // dbg_cycles(graph, &group.iter().cloned().collect::<Vec<_>>());
    let chain = sorted_cycle_by_outgoings(graph, cycle_node_set);
    let max_ref_node = chain
        .iter()
        .enumerate()
        .max_by_key(|(_, ni)| graph.neighbors_directed(**ni, Outgoing).count())
        .unwrap();
    let parent_node_index = chain[(max_ref_node.0 + chain.len() - 1) % chain.len()];
    let edge = graph.find_edge(parent_node_index, *max_ref_node.1).unwrap();
    graph.remove_edge(edge);
}

impl<N, E> CycleDecomposer<N, E> for OutGoingCycleDecomposer {
    /// decompose cycles while there are cycles
    fn decompose_cycles(&self, graph: &mut Graph<N, E>) {
        let mut count = 0usize;
        while count < self.limit {
            for cycle_node_set in get_cycles(graph) {
                unlink_cycle(graph, &cycle_node_set);
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

    use crate::outgoing_sorter::sorted_cycle_by_outgoings;

    fn to_nis(idxs: Vec<usize>) -> Vec<NodeIndex> {
        idxs.iter().map(|i| NodeIndex::new(*i)).collect::<Vec<_>>()
    }

    /// 0 -> 1 -> 2 -> 0 : {0, 1, 2}
    #[test]
    pub fn test_get_cycle1() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle =
            sorted_cycle_by_outgoings(&mut graph, &to_nis(vec![0, 1, 2]).iter().cloned().collect());
        assert_eq!(cycle, to_nis(vec![0, 1, 2]));
    }

    /// 0 -> 1 -> 2 -> 1 : [1, 2]
    #[test]
    pub fn test_get_cycle2() {
        let edges = vec![(0, 1), (1, 2), (2, 1)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle =
            sorted_cycle_by_outgoings(&mut graph, &to_nis(vec![0, 1, 2]).iter().cloned().collect());
        assert_eq!(cycle, to_nis(vec![1, 2]));
    }
}
