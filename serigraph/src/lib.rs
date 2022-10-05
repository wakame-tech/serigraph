pub mod dfs_acyclifier;
pub mod dot_util;
pub mod outgoing_acyclifier;

use petgraph::Graph;
use std::fmt::Display;

pub trait Acyclifier<N: Clone + Display, E> {
    fn acyclify(&self, graph: &mut Graph<N, E>);
}
