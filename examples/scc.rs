use anyhow::Result;
use petgraph::{algo::tarjan_scc, Graph};

fn main() -> Result<()> {
    let edges = vec![
        (1, 2),
        (2, 7),
        (7, 1),
        (4, 2),
        (5, 4),
        (9, 5),
        (6, 9),
        (4, 6),
        (6, 8),
        (8, 3),
        (3, 8),
    ];
    let g = Graph::<i32, ()>::from_edges(&edges);
    let node_indices = tarjan_scc(&g);
    println!("{:?}", node_indices);

    Ok(())
}
