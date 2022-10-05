pub mod condensation;
pub mod dfs_serializer;
pub mod dot_util;
pub mod outgoing_sorter;
pub mod print;
pub mod scc_subgraph_ser;
pub mod toposort_ser;

use std::fmt::Display;

use anyhow::Result;
use petgraph::Graph;

pub trait GraphSerializer<N: Clone + Display, E> {
    fn serialize(&self, graph: &mut Graph<N, E>) -> Result<Vec<N>>;
}
