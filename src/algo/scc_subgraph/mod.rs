use anyhow::Result;
use petgraph::Graph;
use std::fmt::Display;

use crate::algo::{scc_subgraph::condensation::condensation, toposort_ser::ToposortSerializer};

use super::GraphSerializer;

pub mod condensation;

#[derive(Debug)]
pub struct SccSubgraphSerializer;

impl<N: Clone + Display + PartialEq, E: Clone> GraphSerializer<N, E> for SccSubgraphSerializer {
    fn serialize(&self, graph: &mut Graph<N, E>) -> Result<Vec<N>> {
        let mut res: Vec<N> = Vec::new();
        let serializer = ToposortSerializer::default();
        let set_graph = condensation(graph, true);
        dbg!(&set_graph.node_count());
        for ws in set_graph.node_weights() {
            let mut subgraph = graph.filter_map(
                |ni, w| {
                    if ws.contains(w) {
                        Some(graph[ni].clone())
                    } else {
                        None
                    }
                },
                |_, e| Some(e.clone()),
            );
            let v = serializer.serialize(&mut subgraph).unwrap();
            res.extend(v);
        }
        Ok(res)
    }
}
