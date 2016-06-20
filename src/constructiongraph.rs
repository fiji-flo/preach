#[derive(Clone)]
pub struct NodeCH {
    pub e_in: u32,
    pub visited_in: u32,
    pub e_out: u32,
    pub visited_out: u32,
    pub rch_deg: u32,
    pub con: bool,
}

#[derive(Clone)]
pub struct EdgeCH {
    to: usize,
    rev_index: usize,
    direction: u8,
}

pub struct ConstructionGraph {
    n: usize,
    m: usize,
    nodes: Vec<NodeCH>,
    fwd_edges: Vec<Vec<EdgeCH>>,
    bwd_edges: Vec<Vec<EdgeCH>>,
    fwd_m: usize,
    bwd_m: usize,
    num_sinks: usize,
    num_sources: usize,
}

impl EdgeCH {
    pub fn new(to: usize, rev_index: usize, direction: u8) -> EdgeCH {
        EdgeCH { to: to, rev_index: rev_index, direction: direction}
    }
}

impl NodeCH {
    pub fn new() -> NodeCH {
        NodeCH { e_in: 0, visited_in: 0, e_out: 0, visited_out: 0, rch_deg: 0, con: false }
    }
}

impl ConstructionGraph {
    pub fn new(n: usize) -> ConstructionGraph {
        ConstructionGraph {
            n: n,
            m: 0,
            nodes: vec!(NodeCH::new(); n),
            fwd_edges: vec!(Vec::new(); n),
            bwd_edges: vec!(Vec::new(); n),
            fwd_m: 0,
            bwd_m: 0,
            num_sinks: n,
            num_sources: n
        }
    }
    pub fn new_cg_from_file(filename: &String) -> Option<ConstructionGraph> {
        let (n, m, edges) = ::graphreader::edges_from_file(filename).unwrap_or((0, 0, Vec::new()));
        let mut cg = ConstructionGraph::new(n);
        edges.iter().map(|&(s, t)| {
            if cg.fwd_edges[s].is_empty() { cg.num_sinks -= 1; }
            if cg.bwd_edges[t].is_empty() { cg.num_sources -= 1; }
            cg.nodes[s].e_out += 1;
            cg.nodes[t].e_in += 1;

            cg.fwd_edges[s].push(EdgeCH::new(t, s, 0));
            cg.bwd_edges[t].push(EdgeCH::new(s, t, 0));
        }).collect::<Vec<_>>();

        println!("sources: {}, sinks: {}", cg.num_sources, cg.num_sinks);
        Some(cg)
    }

    pub fn split() {

    }
}
/*
pub fn constuct(gt: &::graph::Graph, marker: &mut ::mark::Marker) -> ConstructionGraph {
    let cg = ConstructionGraph {
    };
    cg
}
*/
