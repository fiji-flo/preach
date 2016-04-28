use std::collections::VecDeque;
pub fn bfs<F, GT: ::graph::GraphTraversal>(g: &GT, s: usize, mapped_function: &mut Box<F> ,marker: &mut ::mark::Marker) where F: FnMut(usize) {
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(s);
    marker.mark(s);
    loop {
        if let Some(u) = q.pop_front() {
            mapped_function(u);
            q.extend(g.fwd_edges(u).iter().filter_map(|&x| {
                if !marker.is_marked(x) {
                    marker.mark(x);
                    Some(x)
                } else {
                    None
                }
            }));
        } else {
            break;
        }
    }
}

pub fn full_bfs<F, GT: ::graph::GraphTraversal>(g: &GT, mapped_function: &mut Box<F>, marker: &mut ::mark::Marker) where F: FnMut(usize) {
    for v in g.sources() {
        bfs(g, v, mapped_function, marker);
    }
}
