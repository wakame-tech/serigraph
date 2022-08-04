use anyhow::Result;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serigraph::{outgoing_sorter::OutGoingCycleEliminator, serialize::serialize};

fn main() -> Result<()> {
    let mut graph = Graph::<i32, ()>::new();
    for n in 0..4 {
        graph.add_node(n);
    }
    let edges = vec![(0, 1), (1, 2), (2, 0), (1, 3)];
    for (a, b) in edges {
        graph.add_edge(NodeIndex::new(a), NodeIndex::new(b), ());
    }
    let nodes = serialize(&mut graph, &OutGoingCycleEliminator::default())?;
    assert_eq!(nodes, vec![1, 3, 2, 0]);
    Ok(())
}
