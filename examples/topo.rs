use std::collections::HashMap;

use anyhow::Result;
use petgraph::{algo::toposort, Graph};

#[derive(Debug, Default, Clone)]
struct Doc {
    name: String,
    links: Vec<String>,
}

impl Doc {
    fn new(name: &str, links: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            links,
        }
    }
}

fn to_graph(g: &mut Graph<String, ()>, docs: &[Doc]) {
    let mut map = HashMap::<String, u32>::new();
    for doc in docs {
        let id = g.add_node(doc.name.clone());
        map.insert(doc.name.clone(), id.index() as u32);
    }

    for doc in docs {
        let from = map[&doc.name];
        let mut links = doc.links.clone();
        links.sort();
        for link in links {
            dbg!(&link);
            let to = map[&link];
            let _ = g.add_edge(from.into(), to.into(), ());
        }
    }
}

fn main() -> Result<()> {
    let mut g = Graph::<String, ()>::new();
    let docs = vec![
        Doc::new("doc1", vec!["doc3".to_string(), "doc4".to_string()]),
        Doc::new("doc2", vec![]),
        Doc::new("doc3", vec!["doc2".to_string()]),
        Doc::new("doc4", vec![]),
    ];
    to_graph(&mut g, &docs);

    dbg!(&g);

    let res = toposort(&g, None)
        .unwrap()
        .iter()
        .map(|n| g[*n].clone())
        .collect::<Vec<_>>();
    dbg!(res);

    Ok(())
}
