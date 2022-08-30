use petgraph::algo::kosaraju_scc;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::IndexType;
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use petgraph::EdgeType;
use petgraph::Graph;

pub fn condensation<N: Clone, E: Clone, Ty, Ix>(
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
