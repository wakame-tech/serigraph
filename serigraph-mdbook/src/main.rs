use petgraph::Graph;
use serde::{Deserialize, Serialize};
use serigraph::{dfs_serializer::DfsSerializer, GraphSerializer};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::OpenOptions,
    io::Write,
};

use std::path::Path;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub input_path: String,

    #[clap(short = 'O', long)]
    pub output_path: String,

    #[clap(long)]
    pub from: Option<usize>,

    #[clap(long)]
    pub to: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub title: String,
    pub path: String,
    pub content: String,
    pub backlinks: Vec<Backlink>,
    pub references: Vec<String>,
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

fn write_as_md(notes: &[&Note], path: &Path) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    for (i, note) in notes.iter().enumerate() {
        // println!("[{}] {}", i, note.title);
        f.write(format!("\n\n{}\n\n", note.content).as_bytes())?;
    }
    Ok(())
}

fn into_graph(notes: &[Note]) -> Graph<&Note, String> {
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

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input_path);
    dbg!(&path);
    let content = std::fs::read_to_string(path)?;
    let notes: Vec<Note> = serde_json::from_str(&content)?;
    let mut graph = into_graph(&notes);

    let n = graph.node_count();
    let e = graph.edge_count();
    println!(
        "{} nodes, {} edges, density = {:.4}",
        n,
        e,
        e as f64 / (n as f64 * (n - 1) as f64)
    );

    let ser = DfsSerializer::default();
    dbg!(&ser);
    let notes = ser.serialize(&mut graph)?;
    let (from, to) = (args.from.unwrap_or(0), args.to.unwrap_or(notes.len()));

    let notes = notes[from..to].to_vec();
    write_as_md(&notes, Path::new(args.output_path.as_str()))?;
    Ok(())
}
