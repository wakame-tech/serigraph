use anyhow::Result;
use petgraph::Graph;
use ptree::graph::print_graph;

fn main() -> Result<()> {
    let mut g = Graph::<&str, &str>::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.extend_with_edges(&[(a, b), (a, c)]);
    print_graph(&g, a)?;
    Ok(())
}
