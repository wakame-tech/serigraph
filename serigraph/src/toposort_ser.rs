use anyhow::anyhow;
use anyhow::Result;
use petgraph::{
    algo::{toposort, DfsSpace},
    Graph,
};
use std::fmt::Display;

use crate::outgoing_sorter::eliminate_cycles;

use super::GraphSerializer;

#[derive(Debug)]
pub struct ToposortSerializer {
    pub limit: Option<usize>,
}

impl Default for ToposortSerializer {
    fn default() -> Self {
        Self { limit: None }
    }
}

impl<N: Clone + Display, E> GraphSerializer<N, E> for ToposortSerializer {
    fn serialize(&self, graph: &mut Graph<N, E>) -> Result<Vec<N>> {
        eliminate_cycles(graph, self.limit);
        let mut space = DfsSpace::new(&*graph);
        let nodes =
            toposort(&*graph, Some(&mut space)).map_err(|e| anyhow!("{}", e.node_id().index()))?;
        // inspect_tree(graph, &nodes);
        Ok(nodes
            .iter()
            .map(|n| graph[*n].clone())
            .rev()
            .collect::<Vec<_>>())
    }
}
