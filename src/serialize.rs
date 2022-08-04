use anyhow::anyhow;
use petgraph::{algo::toposort, Graph};

pub trait CycleEliminator<N, E> {
    ///
    fn eliminate_cycles(&self, graph: &mut Graph<N, E>);
}

pub fn serialize<N: Clone, E>(
    graph: &mut Graph<N, E>,
    cycle_eliminator: &dyn CycleEliminator<N, E>,
) -> anyhow::Result<Vec<N>> {
    cycle_eliminator.eliminate_cycles(graph);
    let nodes = toposort(&*graph, None).map_err(|e| anyhow!("{}", e.node_id().index()))?;
    Ok(nodes.iter().map(|n| graph[*n].clone()).collect::<Vec<_>>())
}
