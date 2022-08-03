use std::{fmt::Display, fs::OpenOptions, io::Write, path::Path};

use crate::{readers::obsidian::Note, serialize::serialize_graph};
use anyhow::Result;
use clap::Parser;
use petgraph::{
    dot::{Config, Dot},
    Graph,
};
use readers::obsidian::into_graph;

pub mod readers;
pub mod serialize;

#[derive(Parser, Debug)]
pub struct Args {
    pub input_path: String,
}

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

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input_path);
    let content = std::fs::read_to_string(path)?;
    let notes: Vec<Note> = serde_json::from_str(&content)?;
    let mut graph = into_graph(&notes);

    // let edges = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 1)];
    // let edges = vec![(0, 1), (1, 2), (2, 0), (2, 4), (4, 3), (3, 0)];
    // let mut graph = Graph::<i32, i32>::from_edges(edges);

    let notes = serialize_graph(&mut graph, None)?;

    for note in notes {
        println!("- {}", note.title);
    }

    Ok(())
}
