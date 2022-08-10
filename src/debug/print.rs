use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::fmt::Display;

pub fn inspect_tree<N: Display, E>(graph: &Graph<N, E>, node_indices: &Vec<NodeIndex>) {
    fn visit<N: Display, E>(
        graph: &Graph<N, E>,
        index: NodeIndex,
        visited: &mut Vec<bool>,
        nest: usize,
    ) {
        if visited[index.index()] {
            return;
        }
        visited[index.index()] = true;
        let hiphen = std::iter::repeat('-').take(nest + 1).collect::<String>();
        println!("{} {}", hiphen, graph[index]);
        for child in graph.neighbors(index) {
            visit(graph, child, visited, nest + 1);
        }
    }

    let mut visited = vec![false; graph.node_count()];
    for node in node_indices {
        visit(graph, *node, &mut visited, 0);
    }
}
