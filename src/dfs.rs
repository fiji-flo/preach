pub fn dfs<F, GT: ::graph::GraphTraversal>(g: &GT, s: usize, mapped_function: &Box<F>, marker: &mut ::mark::Marker) where F: Fn(usize) {
    mapped_function(s);
    marker.mark(s);
    for &v in g.fwd_edges(s) {
        if !marker.is_marked(v) {
            dfs(g, v, mapped_function, marker);
        }
    }
}

pub fn full_dfs<F, GT: ::graph::GraphTraversal>(g: &GT, s: usize, mapped_function: &Box<F>, marker: &mut ::mark::Marker) where F: Fn(usize) {
    for v in g.sources() {
        mapped_function(v)
    }
}

#[cfg(test)]
mod tests {
    use super::dfs;

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
        //let mut dfs_out: Vec<usize> = vec![];
        //{
        //    let mut p = Box::new(|x: usize| dfs_out.push(x));
        //    let mut marker = ::mark::TMarker::new(g.len());
        //    dfs(&g, 0, &p, &mut marker);
        //}
        //for v in dfs_out {
        //    println!("{}", v);
        //}
    }
}
