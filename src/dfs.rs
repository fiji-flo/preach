pub fn dfs<F, GT: ::graph::GraphTraversal>(g: &GT, s: usize, mapped_function: &F, marker: &mut ::mark::Marker) where F: Fn(usize) {
    mapped_function(s);
    marker.mark(s);
    for &v in g.fwd_edges(s) {
        if !marker.is_marked(v) {
            dfs(g, v, mapped_function, marker);
        }
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
    g.push_edgelist_bwd(vec![]);
    g.push_edgelist_bwd(vec![0]);
    g.push_edgelist_bwd(vec![0]);
    g.push_edgelist_bwd(vec![1]);
    g.push_edgelist_bwd(vec![1]);
    let mut dfs_out: Vec<usize> = vec![];
    let mut marker = ::mark::TMarker::new(g.len());
    //dfs(&g, 0, &(|x: usize| dfs_out.push(x)), &mut marker);
}
