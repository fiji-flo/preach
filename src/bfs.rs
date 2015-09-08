use std::collections::VecDeque;
pub fn bfs<F, GT: ::graph::GraphTraversal>(g: &GT, s: usize, mapped_function: &F) where F: Fn(usize) {
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(s);
    loop {
        if let Some(u) = q.pop_front() {
            mapped_function(u);
            q.extend(g.fwd_edges(u));
        } else {
            break;
        }
    }
}
