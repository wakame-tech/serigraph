use petgraph::{algo::kosaraju_scc, graph::NodeIndex, Direction::Outgoing, Graph};
use std::{collections::HashSet, fmt::Display};

use crate::Acyclifier;

pub struct OutGoingAcyclifier {
    pub limit: Option<usize>,
}

impl Default for OutGoingAcyclifier {
    fn default() -> Self {
        OutGoingAcyclifier { limit: None }
    }
}

pub fn get_cycle_chain<N, E>(graph: &Graph<N, E>, cycle_set: &Vec<NodeIndex>) -> Vec<NodeIndex> {
    let cycle_hashset = cycle_set.iter().cloned().collect::<HashSet<_>>();
    let mut curr = *cycle_hashset.iter().next().unwrap();
    let mut visited: HashSet<NodeIndex<u32>> = HashSet::new();
    let mut chain: Vec<NodeIndex<u32>> = Vec::new();

    loop {
        chain.push(curr);
        visited.insert(curr);
        let neighbor = graph
            .neighbors_directed(curr, Outgoing)
            .filter(|i| cycle_hashset.contains(i) && !visited.contains(i))
            .next();

        // neighbors is empty means at the end of the cycle
        if let Some(next_ni) = neighbor {
            curr = next_ni;
        } else {
            break;
        }
    }

    let start_ni = graph
        .neighbors_directed(curr, Outgoing)
        .find(|ni| cycle_set.contains(ni))
        .expect("start node must be in the group");
    let (index, _) = chain
        .iter()
        .enumerate()
        .find(|(_, ni)| **ni == start_ni)
        .unwrap();
    // log::debug!(
    //     "chain: {}\ncycle: {}",
    //     dbg_nis(&chain),
    //     dbg_nis(&chain[index..].to_vec())
    // );
    return chain[index..].to_vec();
}

pub fn unlink_cycle<N, E>(graph: &mut Graph<N, E>, component: &Vec<NodeIndex>) {
    if component.len() == 1 {
        let ni = component[0];
        if let Some(e) = graph.find_edge(ni, ni) {
            graph.remove_edge(e);
        }
        return;
    }
    // log::debug!("cycle: {:?}", &component);

    let chain = get_cycle_chain(graph, component);
    let max_ref_node = chain
        .iter()
        .enumerate()
        .max_by_key(|(_, ni)| graph.neighbors_directed(**ni, Outgoing).count())
        .unwrap();
    let parent_node_index = chain[(max_ref_node.0 + chain.len() - 1) % chain.len()];
    // log::debug!(
    //     "unlink: {} -> {}",
    //     parent_node_index.index(),
    //     max_ref_node.1.index()
    // );
    let edge = graph.find_edge(parent_node_index, *max_ref_node.1).unwrap();
    graph.remove_edge(edge);
}

impl<N: Display + Clone, E> Acyclifier<N, E> for OutGoingAcyclifier {
    /// decompose cycles while there are cycles
    fn acyclify(&self, graph: &mut Graph<N, E>) {
        let mut count = 0usize;
        loop {
            // let sccs = tarjan_scc(&*graph);
            let sccs = kosaraju_scc(&*graph);
            // log::debug!("iter: {}/{:?}, {} cycles", count, self.limit, sccs.len());
            if sccs.len() == graph.node_count() {
                break;
            }

            for component in sccs {
                unlink_cycle(graph, &component);
            }
            count += 1;
            if let Some(limit) = self.limit {
                if limit < count {
                    break;
                }
            }
        }
        // dbg!(count);
    }
}

#[cfg(test)]
mod tests {
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;

    use crate::outgoing_acyclifier::get_cycle_chain;

    fn to_nis(idxs: Vec<usize>) -> Vec<NodeIndex> {
        idxs.iter().map(|i| NodeIndex::new(*i)).collect::<Vec<_>>()
    }

    /// 0 -> 1 -> 2 -> 0 : {0, 1, 2}
    #[test]
    pub fn test_get_cycle1() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle = get_cycle_chain(&mut graph, &to_nis(vec![0, 1, 2]).iter().cloned().collect());
        assert_eq!(cycle, to_nis(vec![0, 1, 2]));
    }

    /// 0 -> 1 -> 2 -> 1 : [1, 2]
    #[test]
    pub fn test_get_cycle2() {
        let edges = vec![(0, 1), (1, 2), (2, 1)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle = get_cycle_chain(&mut graph, &to_nis(vec![0, 1, 2]).iter().cloned().collect());
        assert_eq!(cycle, to_nis(vec![1, 2]));
    }
}
