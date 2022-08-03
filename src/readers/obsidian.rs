use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::OpenOptions,
    io::Write,
};

use std::path::Path;

use anyhow::Result;
use clap::Parser;

use crate::serialize::serialize_graph;

#[derive(Parser, Debug)]
pub struct Args {
    pub input_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub title: String,
    pub path: String,
    pub content: String,
    pub backlinks: Vec<Backlink>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Backlink {
    pub title: String,
    pub exists: bool,
    pub path: String,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

pub fn write_as_md(notes: &[&Note], path: &Path) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    for note in notes {
        f.write(format!("{}\n", note.content).as_bytes())?;
    }
    Ok(())
}

pub fn cli() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input_path);
    let content = std::fs::read_to_string(path)?;
    let notes: Vec<Note> = serde_json::from_str(&content)?;

    let mut graph = into_graph(&notes);
    let sorter: crate::decomp_cycles::ByOutGoingEdgeCountSorter = Default::default();
    let notes = serialize_graph(&mut graph, &sorter)?;
    write_as_md(&notes, Path::new("./out.md"))?;
    Ok(())
}

pub fn into_graph(notes: &[Note]) -> Graph<&Note, String> {
    let mut graph: Graph<&Note, String> = Graph::new();
    let mut map = HashMap::<String, u32>::new();

    for note in notes {
        let id = graph.add_node(note);
        map.insert(note.title.clone(), id.index() as u32);
    }

    for note in notes {
        let backlinks = &note
            .backlinks
            .iter()
            .cloned()
            .collect::<HashSet<Backlink>>();

        for backlink in backlinks.iter() {
            if backlink.exists {
                let from = map[&note.title];
                let to = map[&backlink.title];
                let _ = graph.add_edge(from.into(), to.into(), "".to_string());
            }
        }
    }
    graph
}
