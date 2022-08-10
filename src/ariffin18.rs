use crate::serialize::CycleEliminator;

struct Ariffin18;

impl<N, E> CycleEliminator<N, E> for Ariffin18 {
    fn eliminate_cycles(&self, graph: &mut petgraph::Graph<N, E>) {
        todo!()
    }
}
