use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub title: String,
    pub path: String,
    pub content: String,
    pub backlinks: Vec<Backlink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Backlink {
    pub title: String,
    pub exists: bool,
    pub path: String,
}

pub fn into_graph(notes: &[Note]) -> Graph<&Note, String> {
    let mut graph: Graph<&Note, String> = Graph::new();
    let mut map = HashMap::<String, u32>::new();

    for note in notes {
        let id = graph.add_node(note);
        map.insert(note.title.clone(), id.index() as u32);
    }

    for note in notes {
        for backlink in &note.backlinks {
            if backlink.exists {
                let from = map[&note.title];
                let to = map[&backlink.title];
                let _ = graph.add_edge(from.into(), to.into(), "".to_string());
            }
        }
    }
    graph
}
