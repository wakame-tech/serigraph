pub mod dfs_ser;
pub mod scc_subgraph;
pub mod toposort_ser;

use std::fmt::Display;

use anyhow::Result;
use petgraph::Graph;

pub trait GraphSerializer<N: Clone + Display, E> {
    fn serialize(&self, graph: &mut Graph<N, E>) -> Result<Vec<N>>;
}
