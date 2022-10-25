use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use simple_pagerank::Pagerank;

fn with_page_rank(graph: &)

fn main() {
    let edges = &[(0, 1), (1, 2), (1, 3), (2, 3)];
    let mut graph = Graph::<usize, usize>::from_edges(edges);

    let mut pr = Pagerank::<NodeIndex>::new();
    for e in graph.edge_references() {
        pr.add_edge(e.source(), e.target());
    }
    pr.calculate();
    for (score, index) in pr.nodes() {
        println!("{:?}: {:?}", index, score);
    }
}
