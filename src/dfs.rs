pub fn dfs<F, GT: ::graph::GraphTraversal>(g: &GT, s: usize, mapped_function: &mut Box<F>, marker: &mut ::mark::Marker) where F: FnMut(usize) {
    mapped_function(s);
    marker.mark(s);
    for &v in g.fwd_edges(s) {
        if !marker.is_marked(v) {
            dfs(g, v, mapped_function, marker);
        }
    }
}

pub fn full_dfs<F, GT: ::graph::GraphTraversal>(g: &GT, mapped_function: &mut Box<F>, marker: &mut ::mark::Marker) where F: FnMut(usize) {
    for v in g.sources() {
        dfs(g, v, mapped_function, marker);
    }
}

#[test]
fn simple_dws() {
    let mut g = ::graph::Graph::new();
    g.push_edgelist_fwd(vec![1,2]);
    g.push_edgelist_fwd(vec![3,4]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_bwd(vec![]);
    g.push_edgelist_bwd(vec![0]);
    g.push_edgelist_bwd(vec![0]);
    g.push_edgelist_bwd(vec![1]);
    g.push_edgelist_bwd(vec![1]);
    g.push_edgelist_bwd(vec![]);
    let mut dfs_out: Vec<usize> = vec![];
    {
        let mut p = Box::new(|x: usize| { dfs_out.push(x); });
        let mut marker = ::mark::TMarker::new(g.len());
        full_dfs(&g, &mut p, &mut marker);
    }
    assert_eq!(dfs_out, [0,1,3,4,2,5]);
}
