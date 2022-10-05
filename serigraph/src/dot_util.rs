use anyhow::Result;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn dbg_nis(nis: &Vec<NodeIndex>) -> String {
    nis.iter()
        .map(|ni| ni.index().to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

pub fn dump_cycles<N: Display + Default, E: Display + Clone>(
    graph: &Graph<N, E>,
    cycle_nis: Vec<NodeIndex>,
    path: &Path,
) -> Result<()> {
    let mut g = Graph::<N, E>::new();
    for i in 0..graph.node_count() {
        g.add_node(N::default());
    }
    for i in 0..cycle_nis.len() {
        let (a, b) = (cycle_nis[i], cycle_nis[(i + 1) % cycle_nis.len()]);
        let e = graph[graph.find_edge(a, b).unwrap()].clone();
        g.add_edge(cycle_nis[i], cycle_nis[(i + 1) % cycle_nis.len()], e);
    }

    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    let dot_str = format!("{}", Dot::with_config(&g, &[Config::NodeIndexLabel,]));
    f.write(dot_str.as_bytes())?;

    Ok(())
}

pub fn dump_dot<N: Display, E: Display>(graph: &Graph<N, E>, path: &Path) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    let dot_str = format!("{}", Dot::with_config(&graph, &[Config::NodeIndexLabel,]));
    f.write(dot_str.as_bytes())?;
    Ok(())
}
