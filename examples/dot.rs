use anyhow::Result;
use petgraph::{
    dot::{Config, Dot},
    Graph,
};
use std::{fmt::Display, fs::OpenOptions, io::Write, path::Path};

pub fn dump_dot<N: Display, E: Display>(graph: &Graph<N, E>, path: &Path) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    let dot_str = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel,]));
    f.write(dot_str.as_bytes())?;
    Ok(())
}

fn main() {}
