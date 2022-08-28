use anyhow::Result;
use petgraph::graph::{self, NodeIndex};
use petgraph::Graph;
use serigraph::outgoing_sorter::OutGoingCycleEliminator;
use serigraph::serialize::serialize;

fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.target().split("::").last().unwrap(),
                // record.level(),
                message
            ))
        })
        // .level(log::LevelFilter::Info)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn add_random_nodes_and_edges<N: Default, E: Default>(
    graph: &mut Graph<N, E>,
    n_nodes: usize,
    density: f64,
) {
    for i in 0..n_nodes {
        graph.add_node(N::default());
    }

    for i in 0..n_nodes {
        for j in 0..n_nodes {
            // let a = rand::random::<usize>() % n_nodes;
            // let b = rand::random::<usize>() % n_nodes;`
            let a = NodeIndex::new(i);
            let b = NodeIndex::new(j);
            graph.add_edge(a, b, E::default());
        }
    }

    // let n_edges = ((n_nodes * (n_nodes - 1) / 2) as f64 * density) as usize;
    // for _ in 0..n_edges {
    //     let a = rand::random::<usize>() % n_nodes;
    //     let b = rand::random::<usize>() % n_nodes;
    //     if a != b && !graph.contains_edge(NodeIndex::new(a), NodeIndex::new(b)) {
    //         graph.add_edge(NodeIndex::new(a), NodeIndex::new(b), E::default());
    //     }
    // }
}

fn inspect_graph<N, E>(graph: &Graph<N, E>) {
    // println!("{} nodes {} edges", graph.node_count(), graph.edge_count());
    for ni in graph.node_indices() {
        let outs = graph
            .neighbors_directed(ni, petgraph::Outgoing)
            .collect::<Vec<_>>();
        for out in outs {
            println!("{} {}", ni.index(), out.index());
        }
    }
}

fn find_invalid_input() {
    let decomposer = OutGoingCycleEliminator::default();
    let mut trial = 0;

    let n_nodes = 14;
    let density = 1.0;

    loop {
        log::debug!("trial {}", trial);
        trial += 1;
        let mut graph = Graph::<i64, i64>::new();
        add_random_nodes_and_edges(&mut graph, n_nodes, density);

        // inspect_graph(&graph);

        let res = serialize(&mut graph, &decomposer);
        if let Err(e) = res {
            eprintln!("{} is cycle", e);
            inspect_graph(&graph);
            break;
        }
    }
}

fn bench(n_nodes: usize) {
    let decomposer = OutGoingCycleEliminator::default();
    let mut graph = Graph::<i64, i64>::new();
    add_random_nodes_and_edges(&mut graph, n_nodes, 1.0);
    serialize(&mut graph, &decomposer);
}

fn main() -> Result<()> {
    setup_logger()?;
    for n_nodes in [50, 100, 150, 200] {
        dbg!(n_nodes);
        bench(n_nodes);
    }
    Ok(())
}
