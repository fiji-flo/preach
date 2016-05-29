struct NodeCH {
    e_in: u32,
    vistited_in: u32,
    e_out: u32,
    visited_out: u32,
    rch_deg: u32,
    con: bool,
}

struct EdgeCH {
    to: usize,
    rev_index: usize,
    direction: u8,
}

struct ConstructionGraph {
    n: usize,
    m: usize,
    nodes: Vec<NodeCH>,
    fwd_edges: Vec<EdgeCH>,
    bwd_edges: Vec<EdgeCH>,
    fwd_m: usize,
    bwd_m: usize,
    num_sinks: usize,
    num_sources: usize,
}
/*
pub fn constuct(gt: &::graph::Graph, marker: &mut ::mark::Marker) -> ConstructionGraph {
    let cg = ConstructionGraph {
    };
    cg
}
*/
