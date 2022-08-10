use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serigraph::outgoing_sorter::OutGoingCycleEliminator;
use serigraph::serialize::serialize;

fn add_random_nodes_and_edges<N: Default, E: Default>(
    graph: &mut Graph<N, E>,
    n_nodes: usize,
    density: f64,
) {
    for i in 0..n_nodes {
        graph.add_node(N::default());
    }
    let n_edges = ((n_nodes * (n_nodes - 1) / 2) as f64 * density) as usize;
    for _ in 0..n_edges {
        let a = rand::random::<usize>() % n_nodes;
        let b = rand::random::<usize>() % n_nodes;
        if a != b {
            graph.add_edge(NodeIndex::new(a), NodeIndex::new(b), E::default());
        }
    }
}

fn inspect_graph<N, E>(graph: &Graph<N, E>) {
    for ni in graph.node_indices() {
        let outs = graph
            .neighbors_directed(ni, petgraph::Outgoing)
            .map(|nni| nni.index().to_string())
            .collect::<Vec<_>>();
        // log::debug!("{}: {}", ni.index(), outs.join(", "));
        println!("{}: {}", ni.index(), outs.join(", "));
    }
}

fn serialize_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize");
    group.sample_size(10);
    // for n_nodes in [10, 50, 100] {
    for n_nodes in [10, 100] {
        group.throughput(criterion::Throughput::Elements(n_nodes));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_nodes),
            &n_nodes,
            |b, &n_nodes| {
                b.iter(|| {
                    let mut graph = Graph::<i64, i64>::new();
                    add_random_nodes_and_edges(
                        &mut graph,
                        (n_nodes as u64).try_into().unwrap(),
                        0.3,
                    );
                    let res = serialize(&mut graph, &OutGoingCycleEliminator::default());
                    if res.is_err() && n_nodes <= 30 {
                        inspect_graph(&graph);
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, serialize_test);
criterion_main!(benches);
