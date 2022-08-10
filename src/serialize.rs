use anyhow::anyhow;
use petgraph::graph::NodeIndex;
use petgraph::{
    algo::{toposort, DfsSpace},
    Graph,
};
use std::fmt::Display;

pub trait CycleEliminator<N, E> {
    ///
    fn eliminate_cycles(&self, graph: &mut Graph<N, E>);
}

pub fn serialize<N: Clone + Display, E>(
    graph: &mut Graph<N, E>,
    cycle_eliminator: &dyn CycleEliminator<N, E>,
) -> anyhow::Result<Vec<N>> {
    cycle_eliminator.eliminate_cycles(graph);
    let mut space = DfsSpace::new(&*graph);
    let nodes =
        toposort(&*graph, Some(&mut space)).map_err(|e| anyhow!("{}", e.node_id().index()))?;
    // inspect_tree(graph, &nodes);
    Ok(nodes
        .iter()
        .map(|n| graph[*n].clone())
        .rev()
        .collect::<Vec<_>>())
}
