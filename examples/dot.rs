use anyhow::Result;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serigraph::dot_util::dump_cycles;
use serigraph::outgoing_sorter::{
    sorted_cycle_by_outgoings, unlink_cycle, OutGoingCycleEliminator,
};
use serigraph::serialize::serialize;
use serigraph::{dot_util::dump_dot, outgoing_sorter::get_cycles};
use std::{
    fs::{self},
    path::Path,
};

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
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

use anyhow::anyhow;
fn parse_edges(path: &Path) -> Result<Vec<(u32, u32)>> {
    let input = fs::read_to_string(path)?;
    let mut edges: Vec<(u32, u32)> = vec![];
    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        let line = line.split(" ").collect::<Vec<_>>();
        if line.len() != 2 {
            return Err(anyhow!("len != 2"));
        }
        let (a, b) = (line[0].parse::<u32>()?, line[1].parse::<u32>()?);
        edges.push((a, b));
    }
    Ok(edges)
}

fn main() -> Result<()> {
    setup_logger()?;
    let edges = parse_edges(Path::new("incorrect.in"))?;
    let edges = vec![(0, 1), (1, 2), (1, 3), (2, 0)];
    let mut graph = Graph::<i32, String>::from_edges(edges);
    for i in 0..graph.node_count() {
        graph[NodeIndex::new(i)] = i as i32;
    }

    for (i, cycle) in get_cycles(&graph).iter().enumerate() {
        dbg!(cycle);
        // unlink_cycle(&mut graph, cycle);
        // dump_cycles(
        //     &graph,
        //     chain,
        //     Path::new(format!("cycle_{}.dot", i).as_str()),
        // )?;
    }

    let nodes = serialize(&mut graph, &OutGoingCycleEliminator::default())?;
    dbg!(nodes);

    dump_dot(&graph, Path::new("out.dot"))?;
    Ok(())
}
