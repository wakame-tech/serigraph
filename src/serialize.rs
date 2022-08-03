use anyhow::{anyhow, Result};
use petgraph::{algo::toposort, graph::NodeIndex, Graph};
use std::{collections::HashSet, fmt::Display};

pub trait CycleNodesSorter<N, E> {
    /// returns chain of cycle nodes in order
    fn sorted(&self, graph: &Graph<N, E>, cycle_node_set: &HashSet<NodeIndex>) -> Vec<NodeIndex>;

    fn unlink_cycle(&self, graph: &mut Graph<N, E>, cycle_node_set: &HashSet<NodeIndex>);

    fn decompose_cycle(&self, graph: &mut Graph<N, E>);
}

pub fn serialize_graph<N: Clone + Display, E>(
    graph: &mut Graph<N, E>,
    sorter: &dyn CycleNodesSorter<N, E>,
) -> Result<Vec<N>> {
    // decompose cycles while there are cycles
    sorter.decompose_cycle(graph);
    let nodes = toposort(&*graph, None).map_err(|e| anyhow!("{}", graph[e.node_id()]))?;
    Ok(nodes.iter().map(|n| graph[*n].clone()).collect::<Vec<_>>())
}
