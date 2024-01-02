use std::fmt::Debug;

use crate::prelude::*;

use petgraph::visit::{
    Data, GraphBase, IntoEdgeReferences, IntoEdgesDirected, IntoNodeIdentifiers,
};

#[derive(Debug, Clone)]
pub struct FlowAnalysis<Flow, NodeId, EdgeRef> {
    pub max_flow: Flow,
    pub source_partition: HashSet<NodeId>,
    pub sink_partition: HashSet<NodeId>,
    pub min_cut: Vec<EdgeRef>,
}

/// Returns the flow graph after running the max-flow computation.
pub fn edmonds_karp<NodeId, EdgeId, Node, Flow, GR, G>(
    g: GR,
    source: NodeId,
    sink: NodeId,
) -> FlowAnalysis<Flow, NodeId, <GR as IntoEdgeReferences>::EdgeRef>
where
    NodeId: Hash + Eq + Clone + Debug,
    EdgeId: Hash + Eq + Clone + Debug,
    Flow: Eq + Ord + Default + Debug + Clone + AddAssign<Flow> + SubAssign<Flow>,
    GR: IntoEdgesDirected
        + IntoNodeIdentifiers
        + Data<NodeWeight = Node, EdgeWeight = Flow>
        + GraphBase<NodeId = NodeId, EdgeId = EdgeId>
        + Deref<Target = G>,
    G: Index<EdgeId, Output = Flow>,
{
    let mut flow = HashMap::default();
    let mut max_flow = Default::default();

    loop {
        // Do a BFS from source->target following paths where the available
        // capacity is nonzero.
        let mut q = VecDeque::new();
        q.push_back(source.clone());

        let mut predecessor: HashMap<NodeId, (EdgeId, Direction, NodeId, Flow)> =
            HashMap::default();

        while let Some(n) = q.pop_front() {
            if predecessor.contains_key(&sink) {
                break;
            }

            // We need to traverse the regular graph (`Outgoing`), _and_ the reversed graph (`Incoming`)
            // in order to traverse the entire residual graph in our BFS
            let outgoing = g
                .edges_directed(n.clone(), Outgoing)
                .map(|e| (Outgoing, e.source(), e.target(), e.id(), e.weight().clone()));
            // Note that the capacity of all reverse edges is 0 (i.e. Default::default())
            let incoming = g
                .edges_directed(n.clone(), Incoming)
                .map(|e| (Incoming, e.target(), e.source(), e.id(), Default::default()));

            for (dir, from, to, id, capacity) in outgoing.chain(incoming) {
                let flow = flow.get(&(id.clone(), dir)).cloned().unwrap_or_default();
                if !predecessor.contains_key(&to) && to != source && capacity > flow {
                    predecessor.insert(to.clone(), (id, dir, from, capacity));
                    q.push_back(to);
                }
            }
        }

        if predecessor.contains_key(&sink) {
            let mut min = None;
            let mut s = &sink;
            while let Some((e, d, p, c)) = predecessor.get(s) {
                let mut r = c.clone();
                r -= flow.get(&(e.clone(), *d)).cloned().unwrap_or_default();

                if min.as_ref().map(|m| &r < m).unwrap_or(true) {
                    min = Some(r);
                }
                s = p;
            }
            let min = min.unwrap();
            // Update the flow for all those edges
            let mut s = &sink;
            while let Some((e, d, p, _)) = predecessor.get(s) {
                *flow.entry((e.clone(), *d)).or_default() += min.clone();
                let reverse = match d {
                    Direction::Outgoing => Direction::Incoming,
                    Direction::Incoming => Direction::Outgoing,
                };
                *flow.entry((e.clone(), reverse)).or_default() -= min.clone();
                s = p;
            }
            max_flow += min;
        } else {
            break;
        }
    }

    // Compute the cuts -- BFS from the source in the residual graph
    let mut s = HashSet::default();
    let mut q = VecDeque::new();
    q.push_back(source.clone());
    while let Some(n) = q.pop_front() {
        s.insert(n.clone());
        for edge in g.edges_directed(n.clone(), Outgoing) {
            if !s.contains(&edge.target())
                && edge.target() != source
                && *edge.weight()
                    > flow
                        .get(&(edge.id(), Outgoing))
                        .cloned()
                        .unwrap_or_default()
            {
                q.push_back(edge.target());
            }
        }
    }

    FlowAnalysis {
        max_flow,
        min_cut: g
            .edge_references()
            .filter(|e| {
                s.contains(&e.source())
                    && !s.contains(&e.target())
                    && *e.weight() > Default::default()
            })
            .collect::<Vec<_>>(),
        sink_partition: g.node_identifiers().filter(|n| !s.contains(n)).collect(),
        source_partition: s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edmonds_karp() {
        let mut g = DiGraphMap::new();
        g.add_edge("a", "b", 3);
        g.add_edge("a", "d", 3);
        g.add_edge("d", "f", 6);
        g.add_edge("f", "g", 9);
        g.add_edge("e", "g", 1);
        g.add_edge("d", "e", 2);
        g.add_edge("b", "c", 4);
        g.add_edge("c", "a", 3);
        g.add_edge("c", "d", 1);
        g.add_edge("c", "e", 2);

        let max_flow = edmonds_karp(&g, "a", "g");
        assert_eq!(max_flow.max_flow, 5);
        assert_eq!(max_flow.min_cut.len(), 3);
    }

    #[test]
    fn test_edmonds_karp_with_reversed_loop() {
        let mut g = DiGraphMap::new();
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 4, 1);
        g.add_edge(0, 3, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 7, 1);
        g.add_edge(3, 5, 1);
        g.add_edge(5, 6, 1);
        g.add_edge(6, 7, 1);

        let max_flow = edmonds_karp(&g, 0, 7);
        assert_eq!(max_flow.max_flow, 2);
        assert_eq!(max_flow.min_cut.len(), 2);
    }
}
