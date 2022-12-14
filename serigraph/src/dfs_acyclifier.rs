use std::fmt::Display;

use crate::Acyclifier;

#[derive(Default, Debug)]
pub struct DfsAcyclifier;

impl<N: Display + Clone, E> Acyclifier<N, E> for DfsAcyclifier {
    fn acyclify(&self, graph: &mut petgraph::Graph<N, E>) {
        todo!()
        // let mut nodes_by_ref_count = graph.node_indices().collect::<Vec<_>>();
        // nodes_by_ref_count.sort_by_key(|n| {
        //     graph
        //         .neighbors_directed(*n, petgraph::Direction::Outgoing)
        //         .count()
        // });
        // nodes_by_ref_count.reverse();

        // let mut res: Vec<N> = vec![];
        // let mut flags: HashSet<NodeIndex> = HashSet::new();
        // for ni in nodes_by_ref_count {
        //     let mut q: VecDeque<NodeIndex> = VecDeque::new();
        //     q.push_back(ni);
        //     while let Some(ni) = q.pop_front() {
        //         for n in graph.neighbors_directed(ni, Outgoing) {
        //             if !flags.contains(&ni) {
        //                 q.push_back(n);
        //             }
        //         }
        //         if !flags.contains(&ni) {
        //             res.push(graph[ni].clone());
        //             flags.insert(ni);
        //         }
        //     }
        //     println!("")
        // }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Acyclifier;

    use super::DfsAcyclifier;
    use petgraph::algo::{min_spanning_tree, toposort};
    use petgraph::data::FromElements;
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;
    use ptree::graph::print_graph;

    #[test]
    fn test_ser() {
        let mut g = Graph::<i32, ()>::new();
        for i in 0..5 {
            g.add_node(i);
        }
        for (f, t) in &[(0, 1), (1, 2), (2, 0), (1, 3)] {
            g.add_edge(NodeIndex::new(*f), NodeIndex::new(*t), ());
        }
        let acy = DfsAcyclifier;
        acy.acyclify(&mut g);
        let res = toposort(&g, None).unwrap();
        dbg!(res);
    }

    fn make_g(n: usize, edges: &[(usize, usize)]) -> Graph<i32, ()> {
        let mut g = Graph::<i32, ()>::new();
        for i in 0..n {
            g.add_node(i as i32);
        }
        for (f, t) in edges {
            g.add_edge(NodeIndex::new(*f), NodeIndex::new(*t), ());
        }
        g
    }

    #[test]
    fn test_mst() {
        let g1 = make_g(5, &[(0, 1), (2, 0), (1, 2), (0, 3)]);
        let res = min_spanning_tree(&g1);
        let mst_g = Graph::<i32, ()>::from_elements(res);
        print_graph(&mst_g, NodeIndex::new(0));
        // 0
        // ?????? 3
        // ?????? 1
        //    ?????? 2

        let g2 = make_g(5, &[(0, 1), (1, 2), (2, 0), (0, 3)]);
        let res = min_spanning_tree(&g2);
        let mst_g = Graph::<i32, ()>::from_elements(res);
        print_graph(&mst_g, NodeIndex::new(0));
        // 0
        // ?????? 3
        // ?????? 1
    }
}
