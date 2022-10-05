use anyhow::{anyhow, Result};
use petgraph::algo::toposort;
use petgraph::Graph;
use ptree::graph::print_graph;
use serde::{Deserialize, Serialize};
use serigraph::outgoing_acyclifier::OutGoingAcyclifier;
use serigraph::Acyclifier;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;
use std::{fmt::Display, fs::OpenOptions, io::Write, path::Path};
use urlencoding::encode;

#[derive(Debug)]
pub struct Book {
    pub graph: Graph<Note, String>,
    pub resources: HashMap<String, String>,
}

fn serialize<N: Clone + Display, E: Clone>(graph: &mut Graph<N, E>) -> Result<Vec<N>> {
    let acy = OutGoingAcyclifier::default();
    acy.acyclify(graph);

    for parent in graph.node_indices() {
        print_graph(&graph, parent)?;
    }

    let notes = toposort(&*graph, None)
        .map_err(|e| anyhow!("{:?}", e))
        .map(|nis| nis.iter().map(|ni| graph[*ni].clone()).collect::<Vec<_>>())?;
    Ok(notes)
}

fn into_graph(notes: &[Note]) -> Graph<Note, String> {
    let mut graph: Graph<Note, String> = Graph::new();
    let mut map = HashMap::<String, u32>::new();

    for note in notes.iter() {
        let id = graph.add_node(note.clone());
        map.insert(note.title.clone(), id.index() as u32);
    }

    for note in notes.iter() {
        let backlinks = &note
            .backlinks
            .iter()
            .cloned()
            .collect::<HashSet<Backlink>>();
        println!(
            "{} -> {}",
            note.title,
            backlinks
                .iter()
                .map(|e| e.title.clone())
                .collect::<Vec<_>>()
                .join(",")
        );

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

impl Book {
    pub fn from_path(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let book: BookJson = serde_json::from_str(&content)?;
        let graph = into_graph(&book.notes);
        Ok(Self {
            graph,
            resources: book.resources,
        })
    }

    pub fn export_as_mdbook(&mut self, path: &Path, range: Range<usize>) -> Result<()> {
        let book_toml = r#"
[book]
authors = ["author"]
language = "ja"
multilingual = false
src = "src"
title = "vault-book"
[output.html]
mathjax-support = true
[output.pdf]
        "#;

        let notes = serialize(&mut self.graph)?[range].to_vec();

        // make dirs
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
        if !path.exists() {
            fs::create_dir(path)?;
        }
        let src_path = path.join("src");
        if !src_path.exists() {
            fs::create_dir(src_path.clone())?;
        }
        let image_path = src_path.join("images");
        if !image_path.exists() {
            fs::create_dir(image_path.clone())?;
        }

        for (to_image_path, from_image_path) in self.resources.iter() {
            let from_path = Path::new(from_image_path);
            let to_path = &image_path.join(to_image_path.as_str());
            if !from_path.exists() {
                log::warn!("{} not found", from_path.to_string_lossy().to_string());
            }
            fs::copy(from_path, to_path)?;
            log::debug!(
                "copy {} -> {}",
                from_path.to_string_lossy().to_string(),
                to_path.to_string_lossy().to_string()
            );
        }

        // book.toml
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.join("book.toml"))?;
        f.write(format!("{}", book_toml).as_bytes())?;

        // SUMMARY.md
        let summary_path = src_path.join("SUMMARY.md");
        let mut summary = String::new();
        summary += "# Summary\n";

        for note in notes.iter() {
            let link = note.title.replace(" ", "%20");
            summary += format!("- [{}](./{}.md)\n", note.title, link).as_str();
        }
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(summary_path)?;
        f.write(format!("{}", summary).as_bytes())?;

        // src/*.md
        for note in notes.iter() {
            let mut f = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(src_path.join(format!("{}.md", note.title).as_str()))?;
            f.write(format!("{}", note.content).as_bytes())?;
        }
        Ok(())
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.graph.node_count();
        let e = self.graph.edge_count();
        write!(
            f,
            "{} nodes, {} edges, density = {:.4}",
            n,
            e,
            e as f64 / (n as f64 * (n - 1) as f64)
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookJson {
    notes: Vec<Note>,
    resources: HashMap<String, String>,
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
        write!(f, "{}({})", self.title, self.backlinks.len(),)
    }
}
