use anyhow::{anyhow, Result};
use petgraph::{
    algo::{tarjan_scc, toposort},
    graph::NodeIndex,
    Direction::Outgoing,
    Graph,
};
use std::{collections::HashSet, fmt::Display};

fn dbg_cycles<N: Clone + Display, E>(graph: &Graph<N, E>, chain: &Vec<NodeIndex>) {
    println!(
        "\nchain:\n{}",
        chain
            .iter()
            .map(|i| format!(
                "{}(key={}, ref={}) [{}]",
                i.index(),
                graph[*i],
                graph.neighbors_directed(*i, Outgoing).count(),
                graph
                    .neighbors_directed(*i, Outgoing)
                    .map(|n| format!("{}", n.index()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .collect::<Vec<_>>()
            .join("\n-> ")
    );
}

/// returns chain of cycle nodes in order
fn get_cycle<N: Clone, E>(graph: &mut Graph<N, E>, group: Vec<NodeIndex>) -> Vec<NodeIndex> {
    let mut curr = group[0];
    let mut visited: HashSet<NodeIndex<u32>> = HashSet::new();
    let mut chain: Vec<NodeIndex<u32>> = Vec::new();
    loop {
        chain.push(curr);
        // print!("{}({}) -> ", curr.index(), graph[curr]);
        let neighbors = graph
            .neighbors_directed(curr, Outgoing)
            .filter(|i| group.contains(i) && !visited.contains(&i))
            .collect::<Vec<_>>();
        visited.insert(curr);

        // neighbors is empty means at the end of the cycle
        if let Some(next_ni) = neighbors.get(0) {
            curr = *next_ni;
        } else {
            let start_ni = graph
                .neighbors_directed(curr, Outgoing)
                .filter(|ni| group.contains(ni))
                .nth(0)
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

/// unlink edge links from min ref node to max ref node
fn unlink_by_max_ref_node<N: Clone, E>(graph: &mut Graph<N, E>, group: Vec<NodeIndex>) {
    let chain = get_cycle(graph, group);
    // dbg_cycles(graph, &chain);
    let max_ref_node = chain
        .iter()
        .enumerate()
        .max_by_key(|(_, ni)| graph.neighbors_directed(**ni, Outgoing).count())
        .unwrap();
    let parent_node_index = chain[(max_ref_node.0 + chain.len() - 1) % chain.len()];
    // println!(
    //     "\tunlink edge: {}({}) -> {}({})",
    //     parent_node_index.index(),
    //     graph[parent_node_index],
    //     max_ref_node.1.index(),
    //     graph[*max_ref_node.1],
    // );
    let edge = graph.find_edge(parent_node_index, *max_ref_node.1).unwrap();
    graph.remove_edge(edge);
}

pub fn decompose_cycle<N: Clone, E>(graph: &mut Graph<N, E>) -> Result<bool> {
    let groups = tarjan_scc(&*graph);
    for group in groups {
        if group.len() > 1 {
            unlink_by_max_ref_node(graph, group);
        }
    }
    let has_cycle = tarjan_scc(&*graph).iter().any(|group| group.len() > 1);
    Ok(has_cycle)
}

pub fn serialize_graph<N: Clone, E>(
    graph: &mut Graph<N, E>,
    limit: Option<usize>,
) -> Result<Vec<N>> {
    // decompose cycles while there are cycles
    let mut count = 0usize;
    // while count < limit.unwrap_or(100) {
    loop {
        dbg!(count);
        println!("{} nodes {} edges", graph.node_count(), graph.edge_count());
        if decompose_cycle(graph)? {
            count += 1;
        } else {
            break;
        }
    }
    let nodes = toposort(&*graph, None).map_err(|e| anyhow!("{:?}", e))?;
    Ok(nodes.iter().map(|n| graph[*n].clone()).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use crate::serialize::get_cycle;
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;

    fn to_nis(idxs: Vec<usize>) -> Vec<NodeIndex> {
        idxs.iter().map(|i| NodeIndex::new(*i)).collect::<Vec<_>>()
    }

    /// 0 -> 1 -> 2 -> 0 : [0, 1, 2]
    #[test]
    pub fn test_get_cycle1() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle = get_cycle(&mut graph, to_nis(vec![0, 1, 2]));
        assert_eq!(cycle, to_nis(vec![0, 1, 2]));
    }

    /// 0 -> 1 -> 2 -> 1 : [1, 2]
    #[test]
    pub fn test_get_cycle2() {
        let edges = vec![(0, 1), (1, 2), (2, 1)];
        let mut graph = Graph::<i32, i32>::from_edges(edges);
        let cycle = get_cycle(&mut graph, to_nis(vec![0, 1, 2]));
        assert_eq!(cycle, to_nis(vec![1, 2]));
    }
}
