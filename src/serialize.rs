use anyhow::anyhow;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::IndexType;
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use petgraph::EdgeType;
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

fn condensation<N: Clone, E: Clone, Ty, Ix>(
    g: &Graph<N, E, Ty, Ix>,
    make_acyclic: bool,
) -> Graph<Vec<N>, E, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    let sccs = kosaraju_scc(g);
    let mut condensed: Graph<Vec<N>, E, Ty, Ix> = Graph::with_capacity(sccs.len(), g.edge_count());

    // Build a map from old indices to new ones.
    let mut node_map = vec![NodeIndex::end(); g.node_count()];
    for comp in sccs {
        let new_nix = condensed.add_node(Vec::new());
        for nix in comp {
            node_map[nix.index()] = new_nix;
        }
    }

    // Consume nodes and edges of the old graph and insert them into the new one.
    for (nix, node) in g.node_references() {
        condensed[node_map[nix.index()]].push(node.clone());
    }
    for edge in g.edge_references() {
        let source = node_map[edge.source().index()];
        let target = node_map[edge.target().index()];
        if make_acyclic {
            if source != target {
                condensed.update_edge(source, target, edge.weight().clone());
            }
        } else {
            condensed.add_edge(source, target, edge.weight().clone());
        }
    }
    condensed
}

pub fn serialize2<N: Clone + Display + PartialEq, E: Clone>(
    graph: &mut Graph<N, E>,
    cycle_eliminator: &dyn CycleEliminator<N, E>,
) -> anyhow::Result<Vec<N>> {
    let mut res: Vec<N> = Vec::new();
    let set_graph = condensation(graph, true);
    dbg!(&set_graph.node_count());
    for ws in set_graph.node_weights() {
        let mut subgraph = graph.filter_map(
            |ni, w| {
                if ws.contains(w) {
                    Some(graph[ni].clone())
                } else {
                    None
                }
            },
            |_, e| Some(e.clone()),
        );
        let v = serialize(&mut subgraph, cycle_eliminator).unwrap();
        res.extend(v);
    }
    Ok(res)
}
