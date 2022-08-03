use anyhow::Result;
use petgraph::{
    algo::tarjan_scc,
    dot::{Config, Dot},
    graph::NodeIndex,
    Direction::Outgoing,
    Graph,
};
use std::{collections::HashSet, fmt::Display, fs::OpenOptions, io::Write, path::Path};

use crate::serialize::CycleNodesSorter;

pub fn strip_except_cycles<N, E>(graph: &mut Graph<N, E>, sorter: &dyn CycleNodesSorter<N, E>) {
    for ni in graph.node_indices() {
        if graph.neighbors(ni).count() == 0 {
            graph.remove_node(ni);
        }
    }

    let groups = tarjan_scc(&*graph);
    for group in groups {
        let group = group.into_iter().collect::<HashSet<_>>();
        if group.len() > 1 {
            dbg!(&group);
            let cycle_chain = sorter.sorted(graph, &group);
            for ni in cycle_chain {
                graph.remove_node(ni);
            }
        }
    }
}

pub fn dump_dot<N: Display, E: Display>(graph: &Graph<N, E>, path: &Path) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    let dot_str = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel,]));
    f.write(dot_str.as_bytes())?;
    Ok(())
}

pub fn dbg_cycles<N: Clone + Display, E>(graph: &Graph<N, E>, chain: &Vec<NodeIndex>) {
    println!(
        "\nchain:\n{}",
        chain
            .iter()
            .map(|i| format!(
                "{}(key={}, ref={}) [{}]",
                i.index(),
                graph[*i],
                graph.neighbors_directed(*i, Outgoing).count(),
                graph
                    .neighbors_directed(*i, Outgoing)
                    .map(|n| format!("{}", n.index()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .collect::<Vec<_>>()
            .join("\n-> ")
    );
}
