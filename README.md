# serigraph
a graph serializer

```rust
use anyhow::Result;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serigraph::{outgoing_sorter::OutGoingCycleDecomposer, serialize::serialize};

fn main() -> Result<()> {
    let mut graph = Graph::<i32, ()>::new();
    for n in 0..4 {
        graph.add_node(n);
    }
    let edges = vec![(0, 1), (1, 2), (2, 0), (1, 3)];
    for (a, b) in edges {
        graph.add_edge(NodeIndex::new(a), NodeIndex::new(b), ());
    }
    let nodes = serialize(&mut graph, &OutGoingCycleDecomposer::default())?;
    assert_eq!(nodes, vec![1, 3, 2, 0]);
    Ok(())
}
```

## cycle decomposition algorithm
### `OutGoingCycleDecomposer` (naive)
Unlink the edge between the node with the largest degree of exit and its referenced node.

#### Example
$(V, E) = (\{1, 2, 3, 4\}, \{(1, 2), (2, 3), (2, 4), (3, 1)\})$

1. Find all cycles node set $\mathscr{C}$: $\mathscr{C} = \{\{1, 2, 3\}\}$
2. At each cycle, get dependencies chain and count outgoing nodes in all node in a cycle $[1(1), 2(2), 3(1)]$
3. Unlink the edge between the node with the highest outgoing order ($2$) and the referenced node for that node in the cycle ($1$)
4. Perform topological sorting, gets $[1, 3, 2, 0]$